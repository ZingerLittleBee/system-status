#![deny(clippy::all)]

pub mod return_value;

use napi::bindgen_prelude::{BigInt, ToNapiValue};
use napi::Result;
use napi_derive::napi;
use return_value::{_CPULoad, _FileSystem, _LoadAverage, _Memory, _Network, _SocketStats};
use systemstat::{saturating_sub_bytes, DateTime, Platform, System, Utc};
#[napi]
struct Stat {
  sys: System,
  option: StatOption,
}

/// unit enum for stat option
/// B -> KB, 1000 bytes
/// B -> KIB, 1024 bytes
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
  /// Create stat instance.
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

  /// Set stat option, such as unit and decimal.
  #[napi]
  pub fn set_option(&mut self, option: StatOption) {
    self.option = option;
  }

  /// Get filesystem mount information.
  #[napi]
  pub fn mounts(&self) -> Result<Vec<_FileSystem>> {
    let m = self.sys.mounts()?;
    Ok(m.iter().map(|fs| _FileSystem::from(fs)).collect())
  }

  /// Get a filesystem mount information for the filesystem at a given path.
  #[napi]
  pub fn mount_at(&self, at: String) -> Result<_FileSystem> {
    let m = self.sys.mount_at(at).unwrap();
    Ok(_FileSystem::from(&m))
  }

  fn block_device_statistics(&self) {
    todo!();
  }

  /// Get network intefrace information.
  #[napi]
  pub fn networks(&self) -> Result<Vec<_Network>> {
    let nets = self.sys.networks()?;
    Ok(nets.values().map(|netif| _Network::from(netif)).collect())
  }

  fn battery_life() {
    todo!();
  }

  fn on_ac_power() {
    todo!();
  }

  /// Get memory information.
  #[napi]
  pub fn memory(&self) -> Result<_Memory> {
    let mem = self.sys.memory()?;
    Ok(_Memory {
      free: mem.free.as_u64().into(),
      total: mem.total.as_u64().into(),
      used: saturating_sub_bytes(mem.total, mem.free).as_u64().into(),
    })
  }

  /// Get swap memory information.
  #[napi]
  pub fn swap(&self) -> Result<_Memory> {
    let swap = self.sys.swap()?;
    Ok(_Memory {
      free: swap.free.as_u64().into(),
      total: swap.total.as_u64().into(),
      used: saturating_sub_bytes(swap.total, swap.free).as_u64().into(),
    })
  }

  /// Get load average.
  #[napi]
  pub fn load_average(&self) -> Result<_LoadAverage> {
    let loadavg = self.sys.load_average()?;
    Ok(_LoadAverage {
      one: loadavg.one as f64,
      five: loadavg.five as f64,
      fifteen: loadavg.fifteen as f64,
    })
  }

  /// Get the system uptime.
  #[napi]
  pub fn uptime(&self) -> Result<BigInt> {
    let uptime = self.sys.uptime()?;
    Ok(uptime.as_secs().into())
  }

  /// Get the system boot time.
  #[napi]
  pub fn boot_time(&self) -> Result<DateTime<Utc>> {
    let boot_time = self.sys.boot_time()?;
    Ok(boot_time)
  }

  /// Get CPU load statistics, average over all CPUs (cores).
  #[napi]
  pub fn cpu_load_aggregate(&self) -> Result<_CPULoad> {
    let c = self.sys.cpu_load_aggregate()?.done()?;
    Ok(_CPULoad {
      user: c.user as f64,
      nice: c.nice as f64,
      system: c.system as f64,
      interrupt: c.interrupt as f64,
      idle: c.idle as f64,
      iowait: c.platform.iowait as f64,
    })
  }

  /// Get the current CPU temperature in degrees Celsius.
  /// Depending on the platform, this might be core 0, package, etc.
  #[napi]
  pub fn cpu_temp(&self) -> Result<f64> {
    let cpu_temp = self.sys.cpu_temp()?;
    Ok(cpu_temp as f64)
  }

  /// Get information about the number of sockets in use
  #[napi]
  pub fn socket_stats(&self) -> Result<_SocketStats> {
    let socket_stats = self.sys.socket_stats()?;
    Ok(_SocketStats {
      tcp_sockets_in_use: (socket_stats.tcp_sockets_in_use as u64).into(),
      tcp_sockets_orphaned: (socket_stats.tcp_sockets_orphaned as u64).into(),
      udp_sockets_in_use: (socket_stats.udp_sockets_in_use as u64).into(),
      tcp6_sockets_in_use: (socket_stats.tcp6_sockets_in_use as u64).into(),
      udp6_sockets_in_use: (socket_stats.udp6_sockets_in_use as u64).into(),
    })
  }
}
