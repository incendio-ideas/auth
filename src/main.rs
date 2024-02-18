use auth::auth_server::{Auth, AuthServer};
use auth::{LoginRequest, LoginResponse, RegisterRequest, RegisterResponse};
use log::info;
use std::net::SocketAddr;
use tonic::transport::Server;

pub mod auth {
    tonic::include_proto!("auth");
}

#[derive(Debug, Default)]
pub struct MyAuth {}

#[tonic::async_trait]
impl Auth for MyAuth {
    async fn login(
        &self,
        request: tonic::Request<LoginRequest>,
    ) -> Result<tonic::Response<LoginResponse>, tonic::Status> {
        info!("Got a request: {:?}", request);

        let reply = LoginResponse {
            token: "token".into(),
        };

        Ok(tonic::Response::new(reply))
    }

    async fn register(
        &self,
        request: tonic::Request<RegisterRequest>,
    ) -> Result<tonic::Response<RegisterResponse>, tonic::Status> {
        info!("Got a request: {:?}", request);

        let reply = RegisterResponse {
            token: "token".into(),
        };

        Ok(tonic::Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let addr: SocketAddr = "0.0.0.0:50051".parse()?;

    Server::builder()
        .add_service(AuthServer::new(MyAuth::default()))
        .serve(addr)
        .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tonic::Request;

    #[tokio::test]
    async fn test_login() {
        let my_auth = MyAuth::default();
        let request = Request::new(LoginRequest::default());

        let response = my_auth.login(request).await;

        assert!(response.is_ok());
        let response = response.unwrap().into_inner();
        assert_eq!(response.token, "token");
    }

    #[tokio::test]
    async fn test_register() {
        let my_auth = MyAuth::default();
        let request = Request::new(RegisterRequest::default());

        let response = my_auth.register(request).await;

        assert!(response.is_ok());
        let response = response.unwrap().into_inner();
        assert_eq!(response.token, "token");
    }
}
