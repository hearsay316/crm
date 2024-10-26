use anyhow::Result;
use crm::pb::user_service_server::{UserService, UserServiceServer};
use crm::pb::{CreateUserRequest, GetUserRequest, User};
use tonic::transport::Server;
use tonic::{Request, Response, Status};
#[derive(Debug, Default)]
pub struct UserServer;
#[tonic::async_trait]
impl UserService for UserServer {
    async fn get_user(&self, request: Request<GetUserRequest>) -> Result<Response<User>, Status> {
        let input = request.into_inner();
        println!("User :{:?}", input);
        Ok(Response::new(User::default()))
    }

    async fn creatr_user(
        &self,
        request: Request<CreateUserRequest>,
    ) -> Result<Response<User>, Status> {
        let input = request.into_inner();
        println!("User :{:?}", input);
        let user = User::new(1, "Tyr chen", "teh@aaa.com");
        Ok(Response::new(user))
    }
}
// include!(concat!(env!("OUT_DIR"), "/crm.rs"));
#[tokio::main]
async fn main() -> Result<()> {
    let addr = "[::1]:50051".parse()?;
    let svc = UserServer;
    println!("UserServiceServer  listening on {}", addr);
    Server::builder()
        .add_service(UserServiceServer::new(svc))
        .serve(addr)
        .await?;

    Ok(())
}
