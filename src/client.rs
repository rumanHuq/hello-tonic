pub mod hello {
    tonic::include_proto!("hello");
}

pub mod person {
    tonic::include_proto!("person");
}
use hello::greeter_client::GreeterClient;
use hello::HelloRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GreeterClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(HelloRequest {
        name: "Ruman".into(),
    });

    let response = client.say_hello(request).await?;

    println!("RESPONSE={:#?}", response);

    Ok(())
}
