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

    Ok(())
}

async fn health() -> HttpResponse {
    HttpResponse::Ok().body("ok")
}
