use std::sync::Mutex;
use prometheus::{GaugeVec, IntGaugeVec};

use crate::config::Config;

pub struct AppState {
  pub config: Mutex<Config>,
  pub username: String,
  pub password: String,

  pub metrics: MetricsState,
}

pub struct MetricsState {
  pub p115: P115MetricsState,
  pub t31x: T31XMetricsState,
}

impl MetricsState {
  pub fn new() -> Self {
    Self {
      p115: P115MetricsState::new(),
      t31x: T31XMetricsState::new(),
    }
  }

  pub fn register(&self, registry: &prometheus::Registry) {
    self.p115.register(registry);
    self.t31x.register(registry);
  }
}

pub struct P115MetricsState {
  pub current_power: IntGaugeVec,
  pub today_energy: IntGaugeVec,
  pub today_runtime: IntGaugeVec,
  pub month_energy: IntGaugeVec,
  pub month_runtime: IntGaugeVec,
}

impl P115MetricsState {
  pub fn new() -> Self {
    Self {
      current_power: IntGaugeVec::new(
        prometheus::Opts::new("p115_current_power", "Current power of P115 plugs"),
        &["name", "ip"],
      ).unwrap(),
      today_energy: IntGaugeVec::new(
        prometheus::Opts::new("p115_today_energy", "Today's energy usage of P115 plugs"),
        &["name", "ip"],
      ).unwrap(),
      today_runtime: IntGaugeVec::new(
        prometheus::Opts::new("p115_today_runtime", "Today's runtime of P115 plugs"),
        &["name", "ip"],
      ).unwrap(),
      month_energy: IntGaugeVec::new(
        prometheus::Opts::new("p115_month_energy", "This month's energy usage of P115 plugs"),
        &["name", "ip"],
      ).unwrap(),
      month_runtime: IntGaugeVec::new(
        prometheus::Opts::new("p115_month_runtime", "This month's runtime of P115 plugs"),
        &["name", "ip"],
      ).unwrap(),
    }
  }

  pub fn register(&self, registry: &prometheus::Registry) {
    registry.register(Box::new(self.current_power.clone())).unwrap();
    registry.register(Box::new(self.today_energy.clone())).unwrap();
    registry.register(Box::new(self.today_runtime.clone())).unwrap();
    registry.register(Box::new(self.month_energy.clone())).unwrap();
    registry.register(Box::new(self.month_runtime.clone())).unwrap();
  }
}

pub struct T31XMetricsState {
  pub current_temperature: GaugeVec,
  pub current_humidity: IntGaugeVec,
}

impl T31XMetricsState {
  pub fn new() -> Self {
    Self {
      current_temperature: GaugeVec::new(
        prometheus::Opts::new("t31x_current_temperature", "Current temperature of T31X devices"),
        &["name"],
      ).unwrap(),
      current_humidity: IntGaugeVec::new(
        prometheus::Opts::new("t31x_current_humidity", "Current humidity of T31X devices"),
        &["name"],
      ).unwrap(),
    }
  }

  pub fn register(&self, registry: &prometheus::Registry) {
    registry.register(Box::new(self.current_temperature.clone())).unwrap();
    registry.register(Box::new(self.current_humidity.clone())).unwrap();
  }
}
