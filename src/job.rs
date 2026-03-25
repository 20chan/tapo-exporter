use actix_jobs::Job;
use actix_web::web::Data;
use tapo::{ApiClient, responses::ChildDeviceHubResult};

use crate::state::AppState;

pub struct MyJob {
  pub state: Data<AppState>,
}

impl Job for MyJob {
  fn cron(&self) -> &str {
    "*/10 * * * * *"
  }

  fn run(&mut self) {
    let state = self.state.clone();

    actix_rt::spawn(async move {
      let config = state.config.lock().unwrap();

      for plug in config.plugs.as_ref().unwrap() {
        let device = ApiClient::new(&state.username, &state.password)
            .p115(plug.host.clone())
            .await?;

        let current_power = device.get_current_power().await?.current_power;
        let usage = device.get_energy_usage().await?;

        let label = [plug.name.clone(), plug.host.clone()];
        state.metrics.p115.current_power.with_label_values(&label).set(current_power.try_into().unwrap());
        state.metrics.p115.today_energy.with_label_values(&label).set(usage.today_energy.try_into().unwrap());
        state.metrics.p115.today_runtime.with_label_values(&label).set(usage.today_runtime.try_into().unwrap());
        state.metrics.p115.month_energy.with_label_values(&label).set(usage.month_energy.try_into().unwrap());
        state.metrics.p115.month_runtime.with_label_values(&label).set(usage.month_runtime.try_into().unwrap());
      }

      for hub in config.hubs.as_ref().unwrap() {
          let device = ApiClient::new(&state.username, &state.password)
              .h100(hub.host.clone())
              .await?;

          let child_devices = device.get_child_device_list().await?;
          for child in child_devices {
              match child {
                  ChildDeviceHubResult::T31X(device) => {

                      let label = [device.nickname];
                      state.metrics.t31x.current_temperature.with_label_values(&label).set(device.current_temperature.try_into().unwrap());
                      state.metrics.t31x.current_humidity.with_label_values(&label).set(device.current_humidity.try_into().unwrap());
                  }
                  _ => {}
              }
          }
      }

      Ok::<(), Box<dyn std::error::Error>>(())
    });
  }
}
