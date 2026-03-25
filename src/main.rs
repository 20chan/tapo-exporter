extern crate dotenv;

pub mod config;
pub mod state;
pub mod job;

use actix_jobs::{Scheduler, run_forever};
use actix_web::{App, HttpResponse, HttpServer, web};
use actix_web_prom::PrometheusMetricsBuilder;
use dotenv::dotenv;
use std::sync::Mutex;
use std::env;

use crate::{config::Config, job::MyJob, state::{AppState, MetricsState}};

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let config_str = std::fs::read_to_string("config.toml")?;
    let config: Config = toml::from_str(&config_str)?;

    let username = env::var("TAPO_USERNAME").unwrap();
    let password = env::var("TAPO_PASSWORD").unwrap();

    let state = web::Data::new(AppState {
        config: Mutex::new(config),
        username: username.clone(),
        password: password.clone(),
        metrics: MetricsState::new(),
    });

    let prometheus = PrometheusMetricsBuilder::new("tapo_exporter")
        .endpoint("/metrics")
        .build()
        .unwrap();
    state.metrics.register(&prometheus.registry);

    let mut scheduler = Scheduler::new();
    let job = MyJob { state: state.clone() };
    scheduler.add(Box::new(job));

    run_forever(scheduler);

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .wrap(prometheus.clone())
            .service(web::resource("/health").to(health))
    })
        .bind("127.0.0.1:9190")?
        .run()
        .await?;

    /*
    for plug in config.plugs.unwrap_or_default() {
        let device = ApiClient::new(&username, &password)
            .p115(plug.host)
            .await?;

        let current_power = device.get_current_power().await?.current_power;
        let usage = device.get_energy_usage().await?;
        println!("plug {}: power={}, usage: current_power={}, today_energy={}, today_runtime={}", plug.name, current_power, usage.current_power.unwrap_or(0), usage.today_energy, usage.today_runtime);
    }

    for hub in config.hubs.unwrap_or_default() {
        let device = ApiClient::new(&username, &password)
            .h100(hub.host)
            .await?;

        let child_devices = device.get_child_device_list().await?;
        for child in child_devices {
            match child {
                ChildDeviceHubResult::T31X(device) => {
                    println!("hub {}: child device: {} ({}), status: {:?}, temperature: {} °C, humidity: {} %, nickname: {}, low battery: {}", hub.name, device.nickname, device.device_id, device.status, device.current_temperature, device.current_humidity, device.nickname, device.at_low_battery)
                }
                _ => {}
            }
        }
    }
    */


    Ok(())
}

async fn health() -> HttpResponse {
    HttpResponse::Ok().body("ok")
}
