use napi::bindgen_prelude::BigInt;
use napi::bindgen_prelude::ToNapiValue;
use napi_derive::napi;
use systemstat::BlockDeviceStats;
use systemstat::Filesystem;
use systemstat::Network;
use systemstat::NetworkAddrs;

#[napi(object, js_name = "CPULoad")]
pub struct _CPULoad {
  pub user: f64,
  pub nice: f64,
  pub system: f64,
  pub interrupt: f64,
  pub idle: f64,
  pub iowait: f64,
}

#[napi(object, js_name = "FileSystem")]
pub struct _FileSystem {
  /// Used file nodes in filesystem
  pub files: BigInt,
  /// Total file nodes in filesystem
  pub files_total: BigInt,
  /// Free nodes available to non-superuser
  pub files_avail: BigInt,
  /// Free bytes in filesystem
  pub free: BigInt,
  /// Free bytes available to non-superuser
  pub avail: BigInt,
  /// Total bytes in filesystem
  pub total: BigInt,
  /// Maximum filename length
  pub name_max: BigInt,
  pub fs_type: String,
  pub fs_mounted_from: String,
  pub fs_mounted_on: String,
}

impl From<&Filesystem> for _FileSystem {
  fn from(fs: &Filesystem) -> Self {
    _FileSystem {
      files: BigInt::from(fs.files as u64),
      files_total: BigInt::from(fs.files_total as u64),
      files_avail: BigInt::from(fs.files_avail as u64),
      free: BigInt::from(fs.free.as_u64()),
      avail: BigInt::from(fs.avail.as_u64()),
      total: BigInt::from(fs.total.as_u64()),
      name_max: BigInt::from(fs.name_max as u64),
      fs_type: fs.fs_type.as_str().into(),
      fs_mounted_from: fs.fs_mounted_from.as_str().into(),
      fs_mounted_on: fs.fs_mounted_on.as_str().into(),
    }
  }
}

#[napi(js_name = "AddrType")]
pub enum _AddrType {
  Ipv4,
  IPv6,
}

#[napi(object, js_name = "NetworkAddrs")]
pub struct _NetworkAddrs {
  pub addr: String,
  pub netmask: String,
  pub addr_type: _AddrType,
}

impl From<&NetworkAddrs> for _NetworkAddrs {
  fn from(network_addrs: &NetworkAddrs) -> Self {
    let (addr, addr_type) = match network_addrs.addr {
      systemstat::IpAddr::V4(ip) => (ip.to_string(), _AddrType::Ipv4),
      systemstat::IpAddr::V6(ip) => (ip.to_string(), _AddrType::IPv6),
      _ => ("".to_string(), _AddrType::Ipv4),
    };
    let netmask = match network_addrs.netmask {
      systemstat::IpAddr::V4(ip) => ip.to_string(),
      systemstat::IpAddr::V6(ip) => ip.to_string(),
      _ => "".to_string(),
    };
    _NetworkAddrs {
      addr,
      netmask,
      addr_type,
    }
  }
}

#[napi(object, js_name = "Network")]
pub struct _Network {
  pub name: String,
  pub addrs: Vec<_NetworkAddrs>,
}

impl From<&Network> for _Network {
  fn from(network: &Network) -> Self {
    let addrs: Vec<_NetworkAddrs> = network
      .addrs
      .iter()
      .map(|addr| _NetworkAddrs::from(addr))
      .collect();
    _Network {
      name: String::from(network.name.as_str()),
      addrs,
    }
  }
}

#[napi(object, js_name = "Memory")]
pub struct _Memory {
  pub free: BigInt,
  pub total: BigInt,
  pub used: BigInt,
}

#[napi(object, js_name = "LoadAverage")]
pub struct _LoadAverage {
  pub one: f64,
  pub five: f64,
  pub fifteen: f64,
}

#[napi(object, js_name = "SocketStats")]
pub struct _SocketStats {
  pub tcp_sockets_in_use: BigInt,
  pub tcp_sockets_orphaned: BigInt,
  pub udp_sockets_in_use: BigInt,
  pub tcp6_sockets_in_use: BigInt,
  pub udp6_sockets_in_use: BigInt,
}

#[napi(object, js_name = "BlockDeviceStats")]
pub struct _BlockDeviceStats {
  pub name: String,
  pub read_ios: String,
  pub read_merges: String,
  pub read_sectors: String,
  pub read_ticks: String,
  pub write_ios: String,
  pub write_merges: String,
  pub write_sectors: String,
  pub write_ticks: String,
  pub in_flight: String,
  pub io_ticks: String,
  pub time_in_queue: String,
}

impl From<&BlockDeviceStats> for _BlockDeviceStats {
  fn from(b: &BlockDeviceStats) -> Self {
    _BlockDeviceStats {
      name: b.name.as_str().into(),
      read_ios: b.read_ios.to_string(),
      read_merges: b.read_merges.to_string(),
      read_sectors: b.read_sectors.to_string(),
      read_ticks: b.read_ticks.to_string(),
      write_ios: b.write_ios.to_string(),
      write_merges: b.write_merges.to_string(),
      write_sectors: b.write_sectors.to_string(),
      write_ticks: b.write_ticks.to_string(),
      in_flight: b.in_flight.to_string(),
      io_ticks: b.io_ticks.to_string(),
      time_in_queue: b.time_in_queue.to_string(),
    }
  }
}

#[napi(object, js_name = "BatteryLife")]
pub struct _BatteryLife {
  pub remaining_capacity: String,
  pub remaining_time: String,
}
