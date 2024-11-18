use crate::pb::MaterializeRequest;
use crate::{MetaDataService, ResponseStream, ServiceResult};
use tonic::Streaming;

impl MetaDataService {
    pub async fn materialize(
        &self,
        req: Streaming<MaterializeRequest>,
    ) -> ServiceResult<ResponseStream> {
        todo!()
    }
}
