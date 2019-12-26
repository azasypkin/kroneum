use crate::device::{Device, DeviceIdentifier};
use actix_files as fs;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use kroneum_api::{
    array::Array,
    beeper::{Note, Tone, NOTE_1_4_DURATION},
    flash::storage_slot::StorageSlot,
};
use serde_derive::{Deserialize, Serialize};

fn beep() -> impl Responder {
    let device = Device::create().unwrap();
    device.beep(1).unwrap();
    HttpResponse::NoContent()
}

fn melody() -> impl Responder {
    let device = Device::create().unwrap();
    device
        .play_melody(Array::<Tone>::from(
            [
                Tone::new(Note::A5 as u8, NOTE_1_4_DURATION),
                Tone::new(Note::ASharp5 as u8, NOTE_1_4_DURATION),
                Tone::new(Note::B5 as u8, NOTE_1_4_DURATION),
                Tone::new(Note::C6 as u8, NOTE_1_4_DURATION),
                Tone::new(Note::CSharp6 as u8, NOTE_1_4_DURATION),
                Tone::new(Note::D6 as u8, NOTE_1_4_DURATION),
                Tone::new(Note::DSharp6 as u8, NOTE_1_4_DURATION),
                Tone::new(Note::E6 as u8, NOTE_1_4_DURATION),
                Tone::new(Note::F6 as u8, NOTE_1_4_DURATION),
                Tone::new(Note::FSharp6 as u8, NOTE_1_4_DURATION),
                Tone::new(Note::G6 as u8, NOTE_1_4_DURATION),
                Tone::new(Note::GSharp6 as u8, NOTE_1_4_DURATION),
                Tone::new(Note::A6 as u8, NOTE_1_4_DURATION),
            ]
            .as_ref(),
        ))
        .unwrap();
    HttpResponse::NoContent()
}

fn echo(info: web::Json<Vec<u8>>) -> impl Responder {
    HttpResponse::Ok().json(Device::create().unwrap().echo(info.as_ref()).unwrap())
}

#[derive(Debug, Serialize, Deserialize)]
struct DeviceInfo {
    identifier: DeviceIdentifier,
    flash: Vec<u8>,
}

fn get_info() -> impl Responder {
    let device = Device::create().unwrap();
    HttpResponse::Ok().json(DeviceInfo {
        identifier: device.get_identifier().unwrap(),
        flash: vec![
            device.read_flash(StorageSlot::One).unwrap(),
            device.read_flash(StorageSlot::Two).unwrap(),
            device.read_flash(StorageSlot::Three).unwrap(),
            device.read_flash(StorageSlot::Four).unwrap(),
            device.read_flash(StorageSlot::Five).unwrap(),
        ],
    })
}

pub fn start(port: u16) -> Result<(), String> {
    let ui_url = format!("127.0.0.1:{}", port);
    let http_server = HttpServer::new(|| {
        App::new()
            .route("/api/beep", web::get().to(beep))
            .route("/api/melody", web::get().to(melody))
            .route("/api/info", web::get().to(get_info))
            .route("/api/echo", web::post().to(echo))
            .service(fs::Files::new("/", "./src/ui/static/dist").index_file("index.html"))
    })
    .bind(&ui_url)
    .or_else(|err| Err(format!("Failed to bind to {}: {:?}", &ui_url, err)))?;

    println!("Running Kroneum Web UI on http://{}", ui_url);

    http_server
        .run()
        .or_else(|err| Err(format!("Failed to run Web Server {:?}", err)))
}
