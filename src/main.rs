// use tide::prelude::*;

// #[async_std::main]
// async fn main() -> tide::Result<()> {
//     async_std::task::spawn(async {
//         let mut app = tide::new();
//         app.at("/").get(|_| async { Ok("Hello, World!") });
//         app.listen("127.0.0.1:8000").await?;
//         Result::<_, std::io::Error>::Ok(())
//     })
//     .await
//     .unwrap();
    
//     Ok(())
// }
 // adding post request now 
use serde::Deserialize;
use tide::{Request, Response, Body};

#[derive(Deserialize)]
struct Person {
    name: String,
    age: u32,
}

async fn handle_post(request: Request<Body>) -> tide::Result<Response> {
    // Deserialize the request body into a Person struct
    let person: Person = request.body_json().await?;
    
    // Access the name and age
    let name = person.name;
    let age = person.age;
    
    // Process the name and age or perform other operations
    // ...
    
    // Create a response
    let response = Response::builder(200)
        .body(format!("Received POST request with name: {} and age: {}", name, age))
        .build();

    Ok(response)
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();
    
    // Define the POST route
    app.at("/post").post(handle_post);

    app.listen("127.0.0.1:8000").await?;
    Ok(())
}
