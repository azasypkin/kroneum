use crate::device::Device;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

struct AppState {
    device: Device,
}

fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

pub fn start(device: Device, port: u16) -> Result<(), String> {
    let ui_url = format!("127.0.0.1:{}", port);
    let http_server = HttpServer::new(|| {
        App::new()
            //.data(AppState { device })
            .route("/", web::get().to(index))
    })
    .bind(&ui_url)
    .or_else(|err| Err(format!("Failed to bind to {}: {:?}", &ui_url, err)))?;

    println!("Running Kroneum Web UI on http://{}", ui_url);

    http_server
        .run()
        .or_else(|err| Err(format!("Failed to run Web Server {:?}", err)))
}
