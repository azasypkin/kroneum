use crate::device::{Device, DeviceIdentifier};
use actix_files as fs;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use core::convert::TryFrom;
use kroneum_api::{
    adc::ADCChannel, array::Array, beeper::tone::Tone, flash::storage_slot::StorageSlot,
    usb::commands::RadioCommand,
};
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize)]
struct ADCParams {
    channel: u8,
}

async fn adc(params: web::Path<ADCParams>) -> impl Responder {
    match ADCChannel::try_from(params.channel) {
        Ok(channel) => {
            HttpResponse::Ok().json(Device::create().unwrap().read_adc(channel).unwrap())
        }
        Err(message) => HttpResponse::BadRequest().body(message),
    }
}

async fn beep() -> impl Responder {
    let device = Device::create().unwrap();
    device.beep(1).unwrap();
    HttpResponse::NoContent()
}

async fn echo(info: web::Json<Vec<u8>>) -> impl Responder {
    HttpResponse::Ok().json(Device::create().unwrap().echo(info.as_ref()).unwrap())
}

async fn radio_transmit(info: web::Json<Vec<u8>>) -> impl Responder {
    HttpResponse::Ok().json(
        Device::create()
            .unwrap()
            .radio(RadioCommand::Transmit(Array::from(info.as_ref())))
            .unwrap(),
    )
}

async fn radio_receive() -> impl Responder {
    HttpResponse::Ok().json(
        Device::create()
            .unwrap()
            .radio(RadioCommand::Receive)
            .unwrap(),
    )
}

async fn radio_status() -> impl Responder {
    HttpResponse::Ok().json(
        Device::create()
            .unwrap()
            .radio(RadioCommand::Status)
            .unwrap(),
    )
}

async fn play(tones: web::Json<Vec<(u8, u8)>>) -> impl Responder {
    let device = Device::create().unwrap();
    device
        .play_melody(Array::<Tone>::from(
            tones
                .iter()
                .map(|(note, duration)| Tone::new(*note, *duration))
                .collect::<Vec<Tone>>()
                .as_ref(),
        ))
        .unwrap();
    HttpResponse::NoContent()
}

#[derive(Debug, Serialize, Deserialize)]
struct DeviceInfo {
    identifier: DeviceIdentifier,
    flash: Vec<u8>,
}

async fn get_info() -> impl Responder {
    let device = Device::create().unwrap();
    HttpResponse::Ok().json(DeviceInfo {
        identifier: device.get_identifier(),
        flash: vec![
            device.read_flash(StorageSlot::One).unwrap(),
            device.read_flash(StorageSlot::Two).unwrap(),
            device.read_flash(StorageSlot::Three).unwrap(),
            device.read_flash(StorageSlot::Four).unwrap(),
            device.read_flash(StorageSlot::Five).unwrap(),
        ],
    })
}

#[actix_rt::main]
pub async fn run_server(port: u16) -> Result<(), String> {
    let ui_url = format!("127.0.0.1:{}", port);
    let http_server = HttpServer::new(|| {
        App::new()
            .route("/api/beep", web::get().to(beep))
            .route("/api/play", web::post().to(play))
            .route("/api/info", web::get().to(get_info))
            .route("/api/echo", web::post().to(echo))
            .route("/api/radio/receive", web::get().to(radio_receive))
            .route("/api/radio/transmit", web::post().to(radio_transmit))
            .route("/api/radio/status", web::get().to(radio_status))
            .route("/api/adc/{channel}", web::get().to(adc))
            .service(fs::Files::new("/", "./src/ui/static/dist").index_file("index.html"))
    })
    .bind(&ui_url)
    .or_else(|err| Err(format!("Failed to bind to {}: {:?}", &ui_url, err)))?;

    println!("Running Kroneum Web UI on http://{}", ui_url);

    http_server
        .run()
        .await
        .or_else(|err| Err(format!("Failed to run Web Server {:?}", err)))
}
