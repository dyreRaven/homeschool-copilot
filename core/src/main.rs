use tonic::{transport::Server, Request, Response, Status};

pub mod health {
    tonic::include_proto!("copilot.health");
}

use health::health_server::{Health, HealthServer};
use health::{Empty, HealthStatus};
#[derive(Default)]
struct HealthSvc;

#[tonic::async_trait]
impl Health for HealthSvc {
    async fn check(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<HealthStatus>, Status> {
        Ok(Response::new(HealthStatus { serving: true }))
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let addr = "127.0.0.1:50051".parse()?;
    println!("Health server 🩺 listening on {addr}");

    Server::builder()
        .add_service(HealthServer::new(HealthSvc::default()))
        .serve(addr)
        .await?;

    Ok(())
}
