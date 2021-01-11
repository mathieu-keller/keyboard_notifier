use actix_web::HttpResponse;
use reqwest::StatusCode;
use serde::{Serialize, Deserialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct KeyboardDto {
    zone_id: String,
    color: String,
    effect: String,
    pid: String,
    client_name: String,
    message: String,
    name: String,
}

pub async fn send_to_keyboard(color: String, conclusion: &str, repo_name: &str, effect: String) -> HttpResponse {
    let message: String;
    let client = reqwest::Client::new();
    if conclusion.eq("") {
        message = ["Start Github Actions for:", repo_name].join(" ");
    } else {
        message = ["End Github Actions for:", repo_name, "with status:", conclusion].join(" ");
    }
    let url:String;
    match env::var("KEYBOARD_ADDRESS"){
        Err(_) => return HttpResponse::InternalServerError().body("keyboard address is unknown!"),
        Ok(add) => url = [add, "/api/1.0/signals".to_string()].join("")
    }
    let res = client.post(&url)
        .json(&KeyboardDto { zone_id: "19,4".to_string(), client_name: "Rust Server".to_string(), color, effect, pid: "DK5QPID".to_string(), name: "Github Action".to_string(), message })
        .send().await;
    match res {
        Err(e) => {
            HttpResponse::InternalServerError().body(e.to_string())
        }
        Ok(e) => {
            return if e.status() == StatusCode::OK {
                HttpResponse::Ok().finish()
            } else {
                HttpResponse::InternalServerError().finish()
            };
        }
    }
}
