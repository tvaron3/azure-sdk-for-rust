// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Operation trait and shared types for perf test operations.
//!
//! To add a new operation:
//! 1. Create a new file in this directory implementing [`Operation`].
//! 2. Register it in [`create_operations`].
//! 3. Add a CLI flag in `config.rs` to enable/disable it.

mod create_item;
mod query_items;
mod read_item;
mod upsert_item;

use async_trait::async_trait;
use azure_data_cosmos::clients::ContainerClient;
use azure_data_cosmos::options::{
    AvailabilityStrategy, ExcludedRegions, HedgeThreshold, HedgingStrategy, ItemReadOptions,
    ItemWriteOptions, OperationOptions, QueryOptions,
};
use azure_data_cosmos::regions::Region;
use azure_data_cosmos::ResponseHeaders;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;

use crate::config::{Config, ExcludeRegionsScope, HedgingMode};
pub use crate::operations::create_item::CreateItemOperation;
pub use crate::operations::query_items::QueryItemsOperation;
pub use crate::operations::read_item::ReadItemOperation;
pub use crate::operations::upsert_item::UpsertItemOperation;
use crate::seed::SharedItems;

/// Extracts the server-reported request duration from a Cosmos response.
///
/// Returns `None` when the header is missing (e.g., on responses served
/// from cache or when the gateway omitted the diagnostic header) or when
/// the value is infinite or negative.
pub(crate) fn extract_backend_duration(headers: &ResponseHeaders) -> Option<Duration> {
    headers
        .server_duration_ms()
        .map(|ms| Duration::from_secs_f64(ms / 1000.0))
}

/// A single executable perf test operation.
///
/// Implementations are expected to be stateless or use interior mutability.
/// They will be called concurrently from multiple tasks.
#[async_trait]
pub trait Operation: Send + Sync {
    /// Returns the display name of this operation (e.g., "ReadItem").
    fn name(&self) -> &'static str;

    /// Executes one instance of the operation.
    ///
    /// Returns `Ok(Some(d))` when the server reported a processing duration
    /// via the `x-ms-request-duration-ms` response header (this is the
    /// backend latency surfaced separately from the client-observed
    /// wall-clock latency). Returns `Ok(None)` when no backend duration
    /// could be observed (multi-page query streams may aggregate, see
    /// individual implementations).
    async fn execute(
        &self,
        container: &ContainerClient,
    ) -> azure_data_cosmos::Result<Option<Duration>>;
}

/// The item type used for seeding, reading, querying, and upserting.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerfItem {
    pub id: String,
    pub partition_key: String,
    pub value: u64,
    pub payload: String,
}

/// Creates the list of enabled operations based on CLI configuration.
pub fn create_operations(
    config: &Config,
    seeded_items: Arc<SharedItems>,
) -> Vec<Arc<dyn Operation>> {
    let mut ops: Vec<Arc<dyn Operation>> = Vec::new();

    let (read_options, query_options, write_options) = build_item_options(config);

    if !config.no_reads {
        ops.push(Arc::new(ReadItemOperation::new(
            seeded_items.clone(),
            read_options.clone(),
        )));
    }
    if !config.no_queries {
        ops.push(Arc::new(QueryItemsOperation::new(
            seeded_items.clone(),
            query_options.clone(),
        )));
    }
    if !config.no_upserts {
        ops.push(Arc::new(UpsertItemOperation::new(
            seeded_items.clone(),
            write_options.clone(),
        )));
    }
    if !config.no_creates {
        ops.push(Arc::new(CreateItemOperation::new(
            seeded_items,
            write_options,
        )));
    }

    ops
}

/// Builds per-operation options for reads, queries, and writes based on
/// excluded regions, session-token, and hedging configuration.
///
/// Queries are treated as read-side operations for the `--exclude-regions-for`
/// scope. Returns `(None, None, None)` only when none of the contributing
/// settings require per-operation overrides — preserves the prior fast path
/// of passing `None` straight through to the SDK.
fn build_item_options(
    config: &Config,
) -> (
    Option<ItemReadOptions>,
    Option<QueryOptions>,
    Option<ItemWriteOptions>,
) {
    // Shared availability strategy applies to both reads and writes — note that
    // the current SDK only effectively hedges point reads; writes attach the
    // override but the driver does not race them. Stamping the override on
    // writes still lets us record operator intent in result documents.
    let availability = match config.hedging {
        HedgingMode::Default => None,
        HedgingMode::On => {
            // Validated as > 0 in main.rs, so HedgeThreshold::new never returns None here.
            let threshold = HedgeThreshold::new(Duration::from_millis(config.hedging_threshold_ms))
                .expect("--hedging-threshold-ms is validated > 0 in main");
            Some(AvailabilityStrategy::Hedging(HedgingStrategy::new(
                threshold,
            )))
        }
        HedgingMode::Off => Some(AvailabilityStrategy::Disabled),
    };

    let session_capturing_disabled = if config.no_session_token {
        Some(true)
    } else {
        None
    };

    let excluded = if config.excluded_regions.is_empty() {
        None
    } else {
        let regions: Vec<Region> = config
            .excluded_regions
            .iter()
            .map(|r| r.clone().into())
            .collect();
        Some(ExcludedRegions::from_iter(regions))
    };

    // Queries follow the read-side scope for --exclude-regions-for.
    let read_excluded = excluded.as_ref().filter(|_| {
        matches!(
            config.exclude_regions_for,
            ExcludeRegionsScope::Reads | ExcludeRegionsScope::Both
        )
    });
    let write_excluded = excluded.as_ref().filter(|_| {
        matches!(
            config.exclude_regions_for,
            ExcludeRegionsScope::Writes | ExcludeRegionsScope::Both
        )
    });

    let build = |regions: Option<&ExcludedRegions>| -> Option<OperationOptions> {
        if regions.is_none() && session_capturing_disabled.is_none() && availability.is_none() {
            return None;
        }
        let mut op = OperationOptions::default();
        op.excluded_regions = regions.cloned();
        op.session_capturing_disabled = session_capturing_disabled;
        op.availability_strategy = availability;
        Some(op)
    };

    let read = build(read_excluded).map(|op| ItemReadOptions::default().with_operation_options(op));
    let query = build(read_excluded).map(|op| QueryOptions::default().with_operation_options(op));
    let write =
        build(write_excluded).map(|op| ItemWriteOptions::default().with_operation_options(op));

    (read, query, write)
}
