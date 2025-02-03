use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use log::{error, info};
use serde_json::json; // Importing the `json!` macro
use std::env;
use env_logger;
use sys_info; // Importing log macros

async fn disk_space() -> impl Responder {
    match sys_info::disk_info() {
        Ok(disk_info) => {
            let total = disk_info.total;
            let free = disk_info.free;
            let used = total - free; // Calculate used space

            // Calculate percentage used
            let disk_percentage_used = ((used as f64 / total as f64) * 100.0).round() as i32;
            info!(
                "Disk space calculated: total = {}, used = {}, free = {}, used percentage = {}%",
                total, used, free, disk_percentage_used
            );

            HttpResponse::Ok()
                .append_header(("Access-Control-Allow-Origin", "*"))
                .json(json!({
                    "total": total,
                    "used": used,
                    "free": free,
                    "use": format!("{}%", disk_percentage_used),
                }))
        }
        Err(e) => {
            error!("Failed to retrieve disk space information: {}", e);
            HttpResponse::InternalServerError().body("Failed to get disk space information")
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initalise the logger
    env_logger::init();

    // Retrieve the port from command line argument or environment variable
    let port = env::args().nth(1).unwrap_or_else(|| {
        env::var("DISK_USAGE_ENDPOINT_PORT").unwrap_or_else(|_| "5000".to_string())
    });

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(disk_space))
            .route("/disk_space", web::get().to(disk_space))
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}
