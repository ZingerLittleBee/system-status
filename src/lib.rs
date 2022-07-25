pub mod converter;
pub mod return_value;

use converter::{ConvertOption, ConvertResult, Converter};
use napi::bindgen_prelude::BigInt;
use napi::Result;
use napi_derive::napi;
use return_value::_BatteryLife;
use return_value::{_CPULoad, _FileSystem, _LoadAverage, _Memory, _Network, _SocketStats};
use systemstat::{saturating_sub_bytes, DateTime, Platform, System, Utc};

use crate::return_value::_BlockDeviceStats;
#[napi]
pub struct Stat(System);

#[napi]
impl Stat {
  /// Create stat instance.
  #[napi(constructor)]
  pub fn new() -> Self {
    Stat(System::new())
  }

  /// Get filesystem mount information.
  #[napi]
  pub fn mounts(&self) -> Result<Vec<_FileSystem>> {
    let m = self.0.mounts()?;
    Ok(m.iter().map(_FileSystem::from).collect())
  }

  /// Get a filesystem mount information for the filesystem at a given path.
  #[napi]
  pub fn mount_at(&self, at: String) -> Result<_FileSystem> {
    let m = self.0.mount_at(at)?;
    Ok(_FileSystem::from(&m))
  }

  /// Get block device statistics objects.
  #[napi]
  pub fn block_device_statistics(&self) -> Result<Vec<_BlockDeviceStats>> {
    let b = self.0.block_device_statistics()?;
    Ok(b.values().map(_BlockDeviceStats::from).collect())
  }

  /// Get network intefrace information.
  #[napi]
  pub fn networks(&self) -> Result<Vec<_Network>> {
    let nets = self.0.networks()?;
    Ok(nets.values().map(_Network::from).collect())
  }

  /// Get a battery life information object.
  #[napi]
  pub fn battery_life(&self) -> Result<_BatteryLife> {
    let bat = self.0.battery_life()?;
    Ok(_BatteryLife {
      remaining_capacity: bat.remaining_capacity as f64,
      remaining_time: BigInt::from(bat.remaining_time.as_secs()),
    })
  }

  /// Get whether AC power is plugged in.
  #[napi]
  pub fn is_ac_power(&self) -> Result<bool> {
    let on_ac_power = self.0.on_ac_power()?;
    Ok(on_ac_power)
  }

  /// Get memory information.
  #[napi]
  pub fn memory(&self) -> Result<_Memory> {
    let mem = self.0.memory()?;
    Ok(_Memory {
      free: mem.free.as_u64().into(),
      total: mem.total.as_u64().into(),
      used: saturating_sub_bytes(mem.total, mem.free).as_u64().into(),
    })
  }

  /// Get swap memory information.
  #[napi]
  pub fn swap(&self) -> Result<_Memory> {
    let swap = self.0.swap()?;
    Ok(_Memory {
      free: swap.free.as_u64().into(),
      total: swap.total.as_u64().into(),
      used: saturating_sub_bytes(swap.total, swap.free).as_u64().into(),
    })
  }

  /// Get load average.
  #[napi]
  pub fn load_average(&self) -> Result<_LoadAverage> {
    let loadavg = self.0.load_average()?;
    Ok(_LoadAverage {
      one: loadavg.one as f64,
      five: loadavg.five as f64,
      fifteen: loadavg.fifteen as f64,
    })
  }

  /// Get the system uptime.
  #[napi]
  pub fn uptime(&self) -> Result<BigInt> {
    let uptime = self.0.uptime()?;
    Ok(uptime.as_secs().into())
  }

  /// Get the system boot time.
  #[napi]
  pub fn boot_time(&self) -> Result<DateTime<Utc>> {
    let boot_time = self.0.boot_time()?;
    Ok(boot_time)
  }

  /// Get CPU load statistics, average over all CPUs (cores).
  #[napi]
  pub fn cpu_load_aggregate(&self) -> Result<_CPULoad> {
    let c = self.0.cpu_load_aggregate()?.done()?;
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
    let cpu_temp = self.0.cpu_temp()?;
    Ok(cpu_temp as f64)
  }

  /// Get information about the number of sockets in use
  #[napi]
  pub fn socket_stats(&self) -> Result<_SocketStats> {
    let socket_stats = self.0.socket_stats()?;
    Ok(_SocketStats {
      tcp_sockets_in_use: (socket_stats.tcp_sockets_in_use as u64).into(),
      tcp_sockets_orphaned: (socket_stats.tcp_sockets_orphaned as u64).into(),
      udp_sockets_in_use: (socket_stats.udp_sockets_in_use as u64).into(),
      tcp6_sockets_in_use: (socket_stats.tcp6_sockets_in_use as u64).into(),
      udp6_sockets_in_use: (socket_stats.udp6_sockets_in_use as u64).into(),
    })
  }
}

#[napi]
pub fn format(source: BigInt, option: Option<ConvertOption>) -> Result<ConvertResult> {
  let option = if let Some(value) = option {
    value
  } else {
    Default::default()
  };

  Ok(u64::convert(source.get_u64().1, option))
}

impl Default for Stat {
  fn default() -> Self {
    Self::new()
  }
}
