use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde_json::json;  // Importing the `json!` macro
use sys_info;
use std::env;

async fn disk_space() -> impl Responder {
    match sys_info::disk_info() {
        Ok(disk_info) => {
            let total = disk_info.total;
            let free = disk_info.free;
            let used = total - free; // Calculate used space

            // Calculate percentage used
            let disk_percentage_used = ((used as f64 / total as f64) * 100.0).round() as i32;

            HttpResponse::Ok()
                .append_header(("Access-Control-Allow-Origin", "*"))
                .json(json!({
                    "total": total,
                    "used": used,
                    "free": free,
                    "use": format!("{}%", disk_percentage_used),
                }))
        }
        Err(_) => HttpResponse::InternalServerError().body("Failed to get disk space information"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Retrieve the port from command line argument or environment variable
    let port = env::args()
        .nth(1)
        .unwrap_or_else(|| env::var("DISK_USAGE_ENDPOINT_PORT").unwrap_or_else(|_| "5000".to_string()));

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(disk_space))
            .route("/disk_space", web::get().to(disk_space))
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}

