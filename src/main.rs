use tonic::{transport::Server, Request, Response, Status};
use unitsrv::wireguard_server::{Wireguard, WireguardServer};
use unitsrv::{GetPublicKeyRequest, PublicKey};
use std::fs;

pub mod unitsrv {
    tonic::include_proto!("unit.network.v0");
}

#[derive(Default)]
pub struct MyWireguard {}

#[tonic::async_trait]
impl Wireguard for MyWireguard {
    async fn get_public_key(
        &self,
        request: Request<GetPublicKeyRequest>,
    ) -> Result<Response<PublicKey>, Status> {
        println!("Got a request from {:?}", request.remote_addr());
        let public_key: String = fs::read_to_string("/root/.wg/publickey")?;
        let reply = unitsrv::PublicKey {
            value: public_key,
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse().unwrap();
    let wireguard = MyWireguard::default();

    println!("WireguardServer listening on {}", addr);

    Server::builder()
        .add_service(WireguardServer::new(wireguard))
        .serve(addr)
        .await?;

    Ok(())
}
