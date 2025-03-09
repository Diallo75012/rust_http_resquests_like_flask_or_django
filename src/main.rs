// we first allow `dead_code` to limit warnings
// as for eg. `POST` requests won't be called sometimes
#[allow(dead_code)]

// we do our imports
// start with get request `simple mode`
use axum::{
  // http methods (`get`, `post`, `delete`, ..etc...)
  routing::{
    get,
    post,
  },
  // urls.py like stuff
  Router,
  // to be able to handle
  // different types (`tuple`, `String`, `Json`...)
  response::IntoResponse,
  // to be able to show status codes
  // (we won't see those using curl but works if using Postman... for eg.)
  http::StatusCode,
  // to be able to use `Json` struct
  // which will help with any custom `struct`s
  Json
};
// `serde` here to avoid errors as we need to
// `Serialize`/`Deserialize` `struct` to `Json`
use serde::{Serialize, Deserialize};


// we will do all in this main function
// here we will use `tokio` because we love `Japan`
// and accessory helps as well to run `async` processes
// and accept `TCP` connections
#[tokio::main]
async fn main() {
  // define an address where our local server will be
  // listening to ..
  let addr = "127.0.0.1:8001";

  // we need to create a listener to be able to
  // accept `TCP` connections
  // we could have imported at the top of the page...
  // bit it talks to me better like that here in the `fn`
  let listener = tokio::net::TcpListener::bind(
      addr
    // like python `async`/`await`
    ).await;
  println!("Server running at address: {}", addr);

  // now let's use something else other than `unwrap()`
  // like a `match`
  match listener {
    // if `listens` -> `Ok()`
    // we pass it to `axum::serve()`
    // which accepts `TcpListener`
    Ok(listens) => match axum::serve(
          // we are going to make the router `fn`
          listens, router()
        ).await {
          // everything went well? -> we print the message
          Ok(message) => println!("{:?}", message),
          Err(e) => eprintln!("{:?}", e),
        },
    Err(e) => eprintln!("{:?}", e),
  };
}

// `router()` is not `async` just defining routes
// but the `fn` that those routes call will be `async`
fn router() -> Router {
  // here like in `Flask` or `Django` create your routes
  Router::new()
    .route(
      // the address path to reach this function job
      // could be `API` endpoint serving information
      "/message",
      // Now we need the function associated to the route
      // like `views.py` function in (Django)
      // function to be defined after....
      // how to do a `post` request?? we will see 2 ways:
      // 1-`post` request from the same route as `get` one
      get(
        helper_function_get_message
      )
      // this is the first way (same route path)
      .post(
        // we need to make this function...
        post_message_handler
      )
    )
    // 2- create another route path to make post request
    // or any logic there
    .route(
      "/structured-output",
      post(post_message_json_handler)
      .get(friendly_utopic_message)
    )
}

// now let;s make the function executed by the route
// when path is called
async fn helper_function_get_message(
  ) -> &'static str {
  "Hello Shibuya! From Manga Kissa!\n Yo!\n"
}

// we want to have the possibility
// to manage different types
// we will use `post` request method
// and implement `IntoResponse` `struct` from `axum`
// Reminder: `handler` fn (called by (`get`, `post`,...))
// HAVE TO BE: `async` othersie it won't work!
// now let's put input parameters in the function that would 
// need `Derialiazation`
// while the rendered response
// for some types might need `Serialization`
async fn post_message_handler(
    // our input paramter type Json<Our Custom Struct>
    // now you understand that `payload` have to match `struct`
    Json(payload): Json<MessagePostSent>,
  ) -> impl IntoResponse {
  // return type we choose `Tuple` for example
  // second element will implement `IntoResponse`
  // first elem will be `StatusCode`
  // Note: `IntoResponse` accept only pair tuples not more
  ( 
    // will work fine but we won't see it as I use `curl`
    StatusCode::CREATED,
    format!(
      "Post Request Successful! The Crossing is Cleared!\n{}\n",
      payload.message
    )
  )
}

// this will be the payload structure sent message
// we need `Deserialize` because we need to match
// to this `struct` otherwise `error` 
// (Full control of structure of what you accept to receive or to send eventually)
// we need the decorator from `serde` to help us
// we will be posting from outside to our `API` endpoint
// therefore we just need to `Deserialize`
#[derive(Deserialize)]
struct MessagePostSent {
  message: String,
}


// so here we are going to do same use `struct`s
// but see more:
//  - `Serialize`(What we show/output)
//  - `Deserialize`(What we receive in `post` requests)
// CONTROL: structure of comes in and goes out OR `ERROR`
async fn post_message_json_handler(
  // input parameter need to match of `struct` `Tsutaya`
  Json(payload): Json<Tsutaya>
  ) -> impl IntoResponse {
  // let's define two variabes
  // so the `Serialization` will be done for those two vars
  // the `Deserialization` for the incoming imput parameter
  let extra_info = String::from(
    "I an a Gyaru from the API\n"
  );
  let tsutaya_from_api = Tsutaya {
    affluence: 357,
    location: String::from("Shibuya"),
    day: String::from("Monday"),
    starbucks_present: true,
  };
  // now we get the incoming `post`ed message to the endpoint
  let response_data = TsutayaResponse {
    received_data: payload,
    extra_info,
    tsutaya_from_api
  };
  // now we can return the response
  // let's use `Tuple` again to simulate the `API` response
  (
    StatusCode::CREATED,
    Json(response_data),
  )
}

// let's make our `struct`s
// and then finish our function logic
// `Serialize` and `Deserialize` cause we will send it out and receive
#[derive(Serialize, Deserialize)]
// serde parsing is `lazy`
// imagine that you are calling an LLM and you just
// need some specific fields.
// You use `struct` to catch those
// and it behavior is to ignore other fields (eg:`metrics`)
// here we are going to make it strict
// so it doesn't ignore fields but return an error
// more strict = using decorator with `deny_unknown_fields`
#[serde(deny_unknown_fields)]
struct Tsutaya {
  affluence: u64,
  location: String,
  day: String,
  starbucks_present: bool,
}

// this is the schema of our response, it is an example
// just to play with it
#[derive(Serialize)]
struct TsutayaResponse {
  received_data: Tsutaya,
  extra_info: String,
  tsutaya_from_api: Tsutaya,
}

async fn friendly_utopic_message() -> impl IntoResponse {
  "No war, Peace and Love!\n"
}



