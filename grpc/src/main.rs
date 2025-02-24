use hello::greeter_server::{Greeter, GreeterServer};
use hello::{HelloReply, HelloRequest};
use std::sync::{Arc, Mutex};
use tonic::{transport::Server, Request, Response, Status};

pub mod hello {
    tonic::include_proto!("hello");
}

#[derive(Debug)]
pub struct MyGreeter {
    pub count: Arc<Mutex<i32>>, // Use Arc<Mutex> for thread-safe mutability
}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        let name = request.into_inner().name;

        // Lock the mutex to access and modify the count
        let mut count = self.count.lock().unwrap();
        *count += 1;

        // Print the number of requests processed
        println!("Number of requests: {}", *count);

        let reply = HelloReply {
            message: format!("Hello, {}!", name),
            count: *count,
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;

    // Initialize the counter inside an Arc<Mutex>
    let greeter = MyGreeter {
        count: Arc::new(Mutex::new(0)),
    };

    println!("GreeterServer listening on {}", addr);

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
