use crate::prelude::*;
use azure_core::{headers::*, prelude::*, RequestId};
use chrono::{DateTime, Utc};
use http::method::Method;

#[derive(Debug, Clone)]
pub struct AcquireLeaseBuilder {
    container_client: ContainerClient,
    lease_duration: LeaseDuration,
    client_request_id: Option<ClientRequestId>,
    timeout: Option<Timeout>,
    lease_id: Option<LeaseId>,
    proposed_lease_id: Option<ProposedLeaseId>,
}

impl AcquireLeaseBuilder {
    pub(crate) fn new(container_client: ContainerClient, lease_duration: LeaseDuration) -> Self {
        AcquireLeaseBuilder {
            container_client,
            lease_duration,
            client_request_id: None,
            timeout: None,
            lease_id: None,
            proposed_lease_id: None,
        }
    }

    setters! {
        lease_id: LeaseId => Some(lease_id),
        proposed_lease_id: ProposedLeaseId => Some(proposed_lease_id),
        client_request_id: ClientRequestId => Some(client_request_id),
        timeout: Timeout => Some(timeout),
    }

    pub fn into_future(self) -> Response {
        Box::pin(async move {
            let mut url = self.container_client.url_with_segments(None)?;

            url.query_pairs_mut().append_pair("restype", "container");
            url.query_pairs_mut().append_pair("comp", "lease");

            self.timeout.append_to_url_query(&mut url);

            let mut request =
                self.container_client
                    .prepare_request(url.as_str(), Method::PUT, None)?;
            request.insert_header(LEASE_ACTION, "acquire");
            request.add_mandatory_header(&self.lease_duration);
            request.add_optional_header(&self.client_request_id);
            request.add_optional_header(&self.lease_id);
            request.add_optional_header(&self.proposed_lease_id);

            let response = self
                .container_client
                .storage_client()
                .storage_account_client()
                .http_client()
                .execute_request_check_status(&request)
                .await?;

            AcquireLeaseResponse::from_headers(response.headers())
        })
    }
}

azure_storage::response_from_headers!(AcquireLeaseResponse ,
    etag_from_headers => etag: String,
    last_modified_from_headers => last_modified: DateTime<Utc>,
    lease_id_from_headers => lease_id: LeaseId,
    request_id_from_headers => request_id: RequestId,
    date_from_headers => date: DateTime<Utc>
);

pub type Response = futures::future::BoxFuture<'static, azure_core::Result<AcquireLeaseResponse>>;

#[cfg(feature = "into_future")]
impl std::future::IntoFuture for AcquireLeaseBuilder {
    type IntoFuture = Response;
    type Output = <Response as std::future::Future>::Output;
    fn into_future(self) -> Self::IntoFuture {
        Self::into_future(self)
    }
}