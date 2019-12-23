use crate::device::Device;
use actix_files as fs;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

fn beep() -> impl Responder {
    let device = Device::create().unwrap();
    device.beep(1).unwrap();
    HttpResponse::NoContent()
}

fn get_info() -> impl Responder {
    let device = Device::create().unwrap();
    HttpResponse::Ok().json(device.get_identifier().unwrap())
}

pub fn start(port: u16) -> Result<(), String> {
    let ui_url = format!("127.0.0.1:{}", port);
    let http_server = HttpServer::new(|| {
        App::new()
            .route("/api/beep", web::get().to(beep))
            .route("/api/info", web::get().to(get_info))
            .service(fs::Files::new("/", "./src/ui/static/dist").index_file("index.html"))
    })
    .bind(&ui_url)
    .or_else(|err| Err(format!("Failed to bind to {}: {:?}", &ui_url, err)))?;

    println!("Running Kroneum Web UI on http://{}", ui_url);

    http_server
        .run()
        .or_else(|err| Err(format!("Failed to run Web Server {:?}", err)))
}
