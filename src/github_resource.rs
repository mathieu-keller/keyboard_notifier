mod keyboard;
use actix_web::{web, HttpResponse};
use serde::{Serialize, Deserialize};
#[derive(Debug, Serialize, Deserialize)]
struct AppDto {
    id: i32,
    slug: String,
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct CheckSuitDto {
    id: i32,
    status: String,
    conclusion: Option<String>,
    app: AppDto,
}

#[derive(Debug, Serialize, Deserialize)]
struct CheckRunDto {
    status: String,
    conclusion: Option<String>,
    name: String,
    check_suite: CheckSuitDto,
}

#[derive(Debug, Serialize, Deserialize)]
struct RepositoryDto {
    name: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CheckSuitActionDto {
    action: String,
    check_suite: CheckSuitDto,
    repository: RepositoryDto,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CheckRunActionDto {
    action: String,
    check_run: CheckRunDto,
    repository: RepositoryDto,
}

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

pub async fn send_check_run(check_run: web::Json<CheckRunActionDto>) -> HttpResponse {
    keyboard::send_to_keyboard("#FFFF00".to_string(), "", &check_run.repository.name, "BLINK".to_string()).await
}

pub async fn send_check_suit(check_suit: web::Json<CheckSuitActionDto>) -> HttpResponse {
    let conclusion: &str;
    match &check_suit.check_suite.conclusion {
        Some(value) => conclusion = value,
        None => conclusion = ""
    }
    let mut color = "#FF0000".to_string();
    if conclusion.eq("success") {
        color = "#00FF00".to_string();
    }
    keyboard::send_to_keyboard(color, conclusion, &check_suit.repository.name, "SET_COLOR".to_string()).await
}
