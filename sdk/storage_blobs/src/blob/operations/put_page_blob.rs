use crate::{blob::operations::PutBlobResponse, prelude::*};
use azure_core::{
    headers::{Headers, BLOB_CONTENT_LENGTH, BLOB_TYPE},
    prelude::*,
};

#[derive(Debug, Clone)]
pub struct PutPageBlobBuilder {
    blob_client: BlobClient,
    length: u128,
    content_type: Option<ContentType>,
    content_encoding: Option<ContentEncoding>,
    content_language: Option<ContentLanguage>,
    content_disposition: Option<ContentDisposition>,
    metadata: Option<Metadata>,
    // TODO: Support tags
    lease_id: Option<LeaseId>,
    sequence_number: Option<SequenceNumber>,
    context: Context,
}

impl PutPageBlobBuilder {
    pub(crate) fn new(blob_client: BlobClient, length: u128) -> Self {
        Self {
            blob_client,
            length,
            content_type: None,
            content_encoding: None,
            content_language: None,
            content_disposition: None,
            metadata: None,
            lease_id: None,
            sequence_number: None,
            context: Context::new(),
        }
    }

    setters! {
        content_type: ContentType => Some(content_type),
        content_encoding: ContentEncoding => Some(content_encoding),
        content_language: ContentLanguage => Some(content_language),
        content_disposition: ContentDisposition => Some(content_disposition),
        metadata: Metadata => Some(metadata),
        lease_id: LeaseId => Some(lease_id),
        sequence_number: SequenceNumber => Some(sequence_number),
    }

    pub fn into_future(mut self) -> Response {
        Box::pin(async move {
            let url = self.blob_client.url_with_segments(None)?;

            let mut headers = Headers::new();
            headers.insert(BLOB_TYPE, "PageBlob");
            headers.insert(BLOB_CONTENT_LENGTH, &format!("{}", self.length));
            headers.add(self.content_type);
            headers.add(self.content_encoding);
            headers.add(self.content_language);
            headers.add(self.content_disposition);
            if let Some(metadata) = &self.metadata {
                for m in metadata.iter() {
                    headers.add(m);
                }
            }
            headers.add(self.lease_id);
            headers.add(self.sequence_number);

            let mut request =
                self.blob_client
                    .finalize_request(url, azure_core::Method::Put, headers, None)?;

            let response = self
                .blob_client
                .send(&mut self.context, &mut request)
                .await?;
            PutBlobResponse::from_headers(response.headers())
        })
    }
}

pub type Response = futures::future::BoxFuture<'static, azure_core::Result<PutBlobResponse>>;
#[cfg(feature = "into_future")]
impl std::future::IntoFuture for PutPageBlobBuilder {
    type IntoFuture = Response;
    type Output = <Response as std::future::Future>::Output;
    fn into_future(self) -> Self::IntoFuture {
        Self::into_future(self)
    }
}
