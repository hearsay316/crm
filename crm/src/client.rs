use crm::pb::user_service_client::UserServiceClient;
use crm::pb::CreateUserRequest;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut client = UserServiceClient::connect("http://[::1]:50051").await?;
    let request = tonic::Request::new(CreateUserRequest {
        name: "zhang".to_string(),
        email: "qazwsx0@examplse.com".to_string(),
    });
    let response = client.creatr_user(request).await?;
    println!("response :{:?}", response);
    Ok(())
}
