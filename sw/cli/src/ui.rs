use crate::device::Device;
use actix_files as fs;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use core::convert::TryFrom;
use kroneum_api::{
    adc::ADCChannel,
    beeper::tone::Tone,
    flash::storage_slot::StorageSlot,
    usb::commands::{KeyModifiers, MediaKey},
};
use serde_derive::Deserialize;

#[derive(Deserialize)]
struct ADCParams {
    channel: u8,
}

#[derive(Deserialize)]
#[serde(remote = "KeyModifiers")]
struct KeyModifiersDef {
    #[serde(rename(deserialize = "leftCtrl"))]
    left_ctrl: bool,
    #[serde(rename(deserialize = "leftShift"))]
    left_shift: bool,
    #[serde(rename(deserialize = "leftAlt"))]
    left_alt: bool,
    #[serde(rename(deserialize = "leftGUI"))]
    left_gui: bool,
    #[serde(rename(deserialize = "rightCtrl"))]
    right_ctrl: bool,
    #[serde(rename(deserialize = "rightShift"))]
    right_shift: bool,
    #[serde(rename(deserialize = "rightAlt"))]
    right_alt: bool,
    #[serde(rename(deserialize = "rightGUI"))]
    right_gui: bool,
}

#[derive(Deserialize)]
struct KeyParams {
    #[serde(rename(deserialize = "keyCode"))]
    key_code: u8,
    delay: u8,
    #[serde(with = "KeyModifiersDef")]
    modifiers: KeyModifiers,
}

#[derive(Deserialize)]
struct MediaKeyParams {
    #[serde(rename(deserialize = "keyCode"))]
    key_code: u8,
    delay: u8,
}

async fn adc(params: web::Path<ADCParams>) -> impl Responder {
    match ADCChannel::try_from(params.channel) {
        Ok(channel) => {
            HttpResponse::Ok().json(Device::create().unwrap().adc_read(channel).unwrap())
        }
        Err(message) => HttpResponse::BadRequest().body(message),
    }
}

async fn beep() -> impl Responder {
    let device = Device::create().unwrap();
    device.beeper_beep(1).unwrap();
    HttpResponse::NoContent()
}

async fn echo(info: web::Json<Vec<u8>>) -> impl Responder {
    HttpResponse::Ok().json(
        Device::create()
            .unwrap()
            .system_echo(info.as_ref())
            .unwrap(),
    )
}

async fn radio_transmit(info: web::Json<Vec<u8>>) -> impl Responder {
    match Device::create().unwrap().radio_transmit(info.as_ref()) {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(message) => HttpResponse::InternalServerError().body(message),
    }
}

async fn radio_receive() -> impl Responder {
    HttpResponse::Ok().json(Device::create().unwrap().radio_receive().unwrap())
}

async fn radio_status() -> impl Responder {
    HttpResponse::Ok().json(Device::create().unwrap().radio_status().unwrap())
}

async fn play(tones: web::Json<Vec<(u8, u8)>>) -> impl Responder {
    let device = Device::create().unwrap();
    device
        .beeper_melody(
            tones
                .iter()
                .map(|(note, duration)| Tone::new(*note, *duration))
                .collect::<Vec<Tone>>()
                .as_ref(),
        )
        .unwrap();
    HttpResponse::NoContent()
}

async fn get_flash() -> impl Responder {
    let device = Device::create().unwrap();
    HttpResponse::Ok().json(vec![
        device.read_flash(StorageSlot::One).unwrap(),
        device.read_flash(StorageSlot::Two).unwrap(),
        device.read_flash(StorageSlot::Three).unwrap(),
        device.read_flash(StorageSlot::Four).unwrap(),
        device.read_flash(StorageSlot::Five).unwrap(),
    ])
}

async fn get_id() -> impl Responder {
    let device = Device::create().unwrap();
    HttpResponse::Ok().json(device.get_identifier())
}

async fn send_key(params: web::Json<KeyParams>) -> impl Responder {
    let device = Device::create().unwrap();
    device
        .keyboard_key(params.modifiers, params.key_code, params.delay)
        .unwrap();
    HttpResponse::NoContent()
}

async fn send_media_key(params: web::Json<MediaKeyParams>) -> impl Responder {
    match MediaKey::try_from(params.key_code) {
        Ok(media_key) => {
            Device::create()
                .unwrap()
                .keyboard_media_key(media_key, params.delay)
                .unwrap();
            HttpResponse::NoContent().finish()
        }
        Err(_) => {
            HttpResponse::BadRequest().body(format!("Not supported media key: {}", params.key_code))
        }
    }
}

#[actix_rt::main]
pub async fn run_server(port: u16) -> Result<(), String> {
    let ui_url = format!("127.0.0.1:{}", port);
    let http_server = HttpServer::new(|| {
        App::new()
            .route("/api/beep", web::get().to(beep))
            .route("/api/play", web::post().to(play))
            .route("/api/flash", web::get().to(get_flash))
            .route("/api/id", web::get().to(get_id))
            .route("/api/echo", web::post().to(echo))
            .route("/api/radio/receive", web::get().to(radio_receive))
            .route("/api/radio/transmit", web::post().to(radio_transmit))
            .route("/api/radio/status", web::get().to(radio_status))
            .route("/api/adc/{channel}", web::get().to(adc))
            .route("/api/key", web::post().to(send_key))
            .route("/api/media_key", web::post().to(send_media_key))
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
