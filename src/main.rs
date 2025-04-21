use tonic::{transport::Server, Request, Response, Status};
use unitsrv::wireguard_server::{Wireguard, WireguardServer};
use unitsrv::{GetPublicKeyRequest, PublicKey};
use std::fs;

pub mod unitsrv {
    tonic::include_proto!("unit.network.v0");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("unitsrv_descriptor");
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
    let reflection = tonic_reflection::server::Builder::configure()
            .register_encoded_file_descriptor_set(unitsrv::FILE_DESCRIPTOR_SET)
            .build_v1()?;

    println!("WireguardServer listening on {}", addr);

    Server::builder()
        .add_service(reflection)
        .add_service(WireguardServer::new(wireguard))
        .serve(addr)
        .await?;

    Ok(())
}
