use rocket::http::uri::Origin;
use rocket::http::Status;
use rocket::response::Redirect;
use rocket::serde::json::{json, Value};

use openai_api_rs::v1::api::Client;
use openai_api_rs::v1::chat_completion::{self, ChatCompletionRequest};
use openai_api_rs::v1::common::GPT4;
use dotenv::dotenv;

// pub mod ia;
// use crate::ia::open_ai::chat_ai;

#[macro_use] 
extern crate rocket;

const GPTHOLA: Origin<'static> = uri!("/long-laoshi");

#[get("/")] 
fn index() -> Redirect {
    let msg: Option<&str> = None;
    Redirect::to(uri!(GPTHOLA, google_keep_desktop_api("windows-x86_64", "v1.0.14", msg)))
}

// endpoint
// /tauri-releases/google-keep-desktop&win64&1.18.0?msg=""
#[get("/google-keep-desktop/<_platform>/<_current_version>?<msg>")]
fn google_keep_desktop_api(_platform: &str, _current_version: &str, msg: Option<&str>) -> Result<Value, Status> {
    // Status::NoContent
    if let Some(msg) = msg {
        println!("{msg}");
        return  Err(Status::NoContent);
    }
    
    let string = String::from("It's working");
    
    Ok(json!({
        "notes": string
    }))
}

#[get("/chat_with_ai?<msg>")]
fn open_ai_chat(msg: String) -> Result<Value, Status> {
    // let string = chat_ai::chat(msg).unwrap_or(Some("carajo mierda".to_string()));
    let string = chat(msg).unwrap_or(Some("carajo mierda".to_string()));
    
    Ok(json!({
        "notes": string
    }))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount(GPTHOLA, routes![google_keep_desktop_api])
        .mount(GPTHOLA, routes![open_ai_chat])
        .configure(rocket::Config {
            port: 8000,
            ..Default::default()
        })
}

fn chat(message: String) -> Result<Option<String>, Box<dyn std::error::Error>> {
    dotenv().ok();
    let api_token = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set.");
    let client = Client::new(api_token);

    println!("{}", &message);
    
    let req = ChatCompletionRequest::new(
        GPT4.to_string(),
        vec![chat_completion::ChatCompletionMessage {
            role: chat_completion::MessageRole::user,
            content: chat_completion::Content::Text(message),
            name: None,
        }],
    );

    let result = client.chat_completion(req)?;
    let string = &result.choices[0].message.content;
    // println!("Content: {:?}", result.choices[0].message.content);
    // println!("Response Headers: {:?}", result.headers);

    Ok(string.clone())
}