pub mod hello {
    tonic::include_proto!("hello");
}
pub mod person {
    tonic::include_proto!("person");
}

use tonic::{transport::Server, Request, Response, Status};

use hello::{
    greeter_server::{Greeter, GreeterServer},
    HelloReply, HelloRequest,
};
use person::Person;

#[derive(Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a request from {:?}", request.remote_addr());

        let reply = hello::HelloReply {
            person: Some(Person {
                name: request.into_inner().name,
                occupations: vec!["comp_deg".into()],
                marriagestatus: 1,
            }),
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let greeter = MyGreeter::default();

    println!("GreeterServer listening on {}", addr);

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
