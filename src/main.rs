mod proto_gen {
    pub mod helloworld;
}

use std::fs;
use std::path::Path;
use proto_gen::helloworld::{
    HelloReply, HelloRequest,
    greeter_server::{Greeter, GreeterServer},
};

use tonic::{
    transport::{
        server::{TcpConnectInfo, TlsConnectInfo},
        Identity, Server, ServerTlsConfig,
    },
    Request, Response, Status,
};
#[derive(Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a request from {:?}", request.remote_addr());

        let reply = HelloReply {
            message: format!("Hello {}!", request.into_inner().name),
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let key_path = Path::new("SSL/ed25519.key");
    let crt_path = Path::new("SSL/ed25519.crt");

    let key_content = fs::read_to_string(key_path)?;
    let crt_content = fs::read_to_string(crt_path)?;
    let identity = Identity::from_pem(crt_content, key_content);

    let addr = "0.0.0.0:50051".parse().unwrap();
    let greeter = MyGreeter::default();
    println!("GreeterServer listening on {}", addr);

    Server::builder()
        .tls_config(ServerTlsConfig::new().identity(identity))?
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
