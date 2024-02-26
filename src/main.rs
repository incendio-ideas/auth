use auth::auth_server::{Auth, AuthServer};
use auth::{LoginRequest, LoginResponse, RegisterRequest, RegisterResponse};
use log::info;
use sea_orm::Database;
use sea_orm::ActiveValue::Set;
use std::net::SocketAddr;
use tonic::transport::Server;
use sea_orm::EntityTrait;

use crate::entities::key;
use crate::entities::user;

mod entities;

mod auth {
    tonic::include_proto!("auth");
}

#[derive(Debug, Default)]
pub struct MyAuth {
    db: std::sync::Arc<sea_orm::DatabaseConnection>,
}

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

        let insert_result = user::Entity::insert(user::ActiveModel {
            id: Set(uuid::Uuid::new_v4()),
            email: Set(request.get_ref().email.clone()),
        }).exec(&*self.db).await;

        let user_id = match insert_result {
            Ok(result) => result.last_insert_id,
            Err(_) => return Err(tonic::Status::internal("Failed to insert user")),
        };

        let _ = key::Entity::insert(key::ActiveModel {
            id: Set(format!("email:{}", request.get_ref().email)),
            hashed_password: Set(request.get_ref().password.clone()),
            user_id: Set(user_id),
            primary: Set(true),
        }).exec(&*self.db).await;

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
    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://admin:pass@localhost:5432/auth".to_string());
    let db = Database::connect(database_url).await?;

    Server::builder()
        .add_service(AuthServer::new(MyAuth {
            db: std::sync::Arc::new(db),
        }))
        .serve(addr)
        .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tonic::Request;

    use sea_orm::{
        DatabaseBackend, MockDatabase, MockExecResult,
    };

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
        let db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_exec_results(vec![
                MockExecResult {
                    last_insert_id: 1,
                    rows_affected: 1,
                },
                MockExecResult {
                    last_insert_id: 1,
                    rows_affected: 1,
                },
            ]).into_connection();

        let my_auth = MyAuth {
            db: std::sync::Arc::new(db),
        };

        let request = Request::new(RegisterRequest::default());
        let response = my_auth.register(request).await;

        assert!(response.is_ok());
        let response = response.unwrap().into_inner();
        assert_eq!(response.token, "token");
    }
}
