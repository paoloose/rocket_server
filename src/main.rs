use rocket::http::uri::Origin;
use rocket::http::Status;
use rocket::response::Redirect;
use rocket::serde::json::{json, Value};

pub mod ia;
use crate::ia::openAI::chatAI;

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
#[get("/google-keep-desktop/<_platform>/<current_version>?<msg>")]
fn google_keep_desktop_api(_platform: &str, current_version: &str, msg: Option<&str>) -> Result<Value, Status> {
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
    let string = chatAI::chat(msg).unwrap_or(Some("carajo mierda".to_string()));
    
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
}
