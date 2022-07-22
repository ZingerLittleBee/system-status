#![deny(clippy::all)]

pub mod return_value;

use napi::bindgen_prelude::{BigInt, ToNapiValue};
use napi_derive::napi;
use return_value::{_CPULoad, _FileSystem, _Network};
use systemstat::{saturating_sub_bytes, Platform, System};
#[napi]
struct Stat {
  sys: System,
  option: StatOption,
}

#[napi]
pub enum Unit {
  B,
  KB,
  MB,
  GB,
  TB,
  PB,
  KIB,
  MIB,
  GIB,
  TIB,
  PIB,
}

#[napi(object)]
pub struct StatOption {
  pub unit: Unit,
  /// Number of decimal places to be retained
  pub decimal: u8,
}

impl Default for StatOption {
  fn default() -> Self {
    StatOption {
      unit: Unit::B,
      decimal: 2,
    }
  }
}

#[napi]
impl Stat {
  #[napi(constructor)]
  pub fn new(option: Option<StatOption>) -> Self {
    let real_option: StatOption = if option.is_some() {
      option.unwrap()
    } else {
      Default::default()
    };
    Stat {
      sys: System::new(),
      option: real_option,
    }
  }

  #[napi]
  pub fn set_option(&mut self, option: StatOption) {
    self.option = option;
  }

  #[napi]
  pub fn mounts(&self) -> Vec<_FileSystem> {
    let m = self.sys.mounts().unwrap();
    m.iter().map(|fs| _FileSystem::from(fs)).collect()
  }

  #[napi]
  pub fn mount_at(&self, at: String) -> _FileSystem {
    let m = self.sys.mount_at(at).unwrap();
    _FileSystem::from(&m)
  }

  #[napi]
  pub fn block_device_statistics(&self) {
    todo!();
  }

  #[napi]
  pub fn networks(&self) -> Vec<_Network> {
    let nets = self.sys.networks().unwrap();
    nets.values().map(|netif| _Network::from(netif)).collect()
  }

  #[napi]
  pub fn cpu_load_aggregate(&self) -> _CPULoad {
    let c = self.sys.cpu_load_aggregate().unwrap().done().unwrap();
    _CPULoad {
      user: c.user as f64,
      nice: c.nice as f64,
      system: c.system as f64,
      interrupt: c.interrupt as f64,
      idle: c.idle as f64,
      iowait: c.platform.iowait as f64,
    }
  }

  #[napi]
  pub fn load_average(&self) {
    match self.sys.load_average() {
      Ok(loadavg) => println!(
        "\nLoad average: {} {} {}",
        loadavg.one, loadavg.five, loadavg.fifteen
      ),
      Err(x) => println!("\nLoad average: error: {}", x),
    }
  }

  #[napi]
  pub fn socket_stats(&self) {
    println!("{:#?}", self.sys.socket_stats().unwrap());
  }

  #[napi]
  pub fn uptime(&self) {
    println!("{:#?}", self.sys.uptime().unwrap());
  }
  #[napi]
  pub fn swap(&self) {
    match self.sys.swap() {
      Ok(swap) => println!(
        "\nSwap: {} used / {} ({} bytes) total ({:?})",
        saturating_sub_bytes(swap.total, swap.free),
        swap.total,
        swap.total.as_u64(),
        swap.platform_swap
      ),
      Err(x) => println!("\nSwap: error: {}", x),
    }
  }
}
