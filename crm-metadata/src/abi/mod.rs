use tonic::Streaming;
use crate::{MetaDataService, ResponseStream, ServiceResult};
use crate::pb::MaterializeRequest;

impl MetaDataService {
    pub async fn materialize(&self,req:Streaming<MaterializeRequest>)->ServiceResult<ResponseStream>{
        todo!()
    }

}