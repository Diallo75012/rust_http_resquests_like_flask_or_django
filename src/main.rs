// we allow `dead_code` as some request like `POST` won't be triggered
// we limit the compilor warning output like that
// I will be assume some python stuff to keep my head around this
#![allow(dead_code)]
// start import `get` starting easy mode
use axum::{
  // http types (`get`, `post`, `delete`...)
  routing::{
    get,
    post,
  },
  // for `urls.py`
  Router,
  // to be able to handle different types ()
  response::IntoResponse,
  // to be able to show status codes
  http::StatusCode,
  // to be able to use `Json` `struct`
  // which will help with any of our custom `struct`s
  Json,
};
// we use `serde` to avoid errors as we need to serialize
// the later `struct` that we are going to start to `Json`
use serde::{Serialize};

// let's do all in the main function for simplicity
// and use `tokio` because we love `Japan`
// this is the way to be able to sun `asynchronously`
// and accept `TCP` connections
#[tokio::main]
async fn main() {
  //define an address where our local server will be listening to
  let addr = "127.0.0.1:8001";

  // create a listener to be able to accept `TCP` connections
  // this returns a type `Result`
  let listener = tokio::net::TcpListener::bind(addr).await;
  println!("Server is running at address: {:?}", addr);

  // let's use something other than `unwrap()` like a `Result`
  match listener {
    // if it listens `Ok()`
    // we pass it to `axum::serve()` which except a type `TcpListener`
    Ok(listener) => match axum::serve(listener, router()).await {
        // here we chain with another `result`
        Ok(something) => println!("{:?}\n", something),
        Err(e) => eprintln!("serve router issue: {:?}", e),
      }
    Err(e) => eprintln!("listener issue: {:?}", e),
  }
}

// the following `fn` is not `async` just defining routes
fn router() -> Router {
  // here like in `Flask` or `Django` create your routes
  Router::new()
    .route(
      // the address path to reach this function calling job
      // could be `API` endpoint serving information
      // do not forget the `/` otherwise it is not going to work
      "/message",
      // this route need a function associated to it
      // Note: while the router itself is not `async`,
      // the method called here will be `async`
      // we will manage `get` request
      // let's now see the post request...
      // first create the function chained in the same route
      // `=` 1 `route` accepting `GET` and `POST` requests
      get(helper_function_get_message).post(post_message_handler),
    )
    // this is how to add extra route-path
    .route(
      // we are going to use more complexe structure here
      // imagine calling an llm and getting a json
      // or more WE being the service rendering Json output
      // as here I am going to use only `Serialize` from `serde`
      "/structured_output",
      post(post_message_json_handler),
    )
}

// now let's make the function logic
// that the route will execute when the path `/message` is called
async fn helper_function_get_message() -> &'static str {
  "Hello Shibuya! From Manga Kissa\n"
}

// let's define this `fn` activated on `POST` requests
// we want the to have the possibility to manage different types
// we will use the implementation of struct `IntoResponse`
// handler should be `async`
async fn post_message_handler() -> impl IntoResponse {
  // we will return here a type `tuple`
  // `IntoResponse` friend is here to handle those types for us
  // we will return a status code
  // but I use `Curl` so we won't see it
  (StatusCode::CREATED, "Did You just arrived in Meidaimae?\n")
}

// let's play a bit more
// 1 `GET` then 1 `POST` in the same route path
// what if i need another path... and little bit more...

async fn post_message_json_handler() -> (
    StatusCode,
    Json<Tsutaya>
  ) {
  // we instantiate our `struct`
  let tsutaya_info = Tsutaya {
    affluence: 357,
    location: String::from("Shibuya"),
    day: String::from("Monday"),
    starbucks_present: true,
  };

  // we use the `Json` `struct` to return our custom `struct`
  // tuple returned as defined above
  (
    StatusCode::CREATED,
    Json(tsutaya_info),
  )
}


// let's create a struct to see if we can return our custom object
// we use here decorator to `Serialize` the structure to `Json`
#[derive(Serialize, Debug)]
struct Tsutaya {
  affluence: u64,
  location: String,
  day: String,
  starbucks_present: bool,
}


