use crate::device::Device;
use actix_files as fs;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

fn beep() -> impl Responder {
    println!("Route `/beep` is called");
    Device::create().unwrap().beep(1).unwrap();
    HttpResponse::Ok().body("beep")
}

pub fn start(port: u16) -> Result<(), String> {
    let ui_url = format!("127.0.0.1:{}", port);
    let http_server = HttpServer::new(|| {
        App::new()
            .route("/api/beep", web::get().to(beep))
            .service(fs::Files::new("/", "./src/ui/static/dist").index_file("index.html"))
    })
    .bind(&ui_url)
    .or_else(|err| Err(format!("Failed to bind to {}: {:?}", &ui_url, err)))?;

    println!("Running Kroneum Web UI on http://{}", ui_url);

    http_server
        .run()
        .or_else(|err| Err(format!("Failed to run Web Server {:?}", err)))
}
