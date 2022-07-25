use napi::bindgen_prelude::BigInt;
use napi::bindgen_prelude::ToNapiValue;
use napi_derive::napi;
use systemstat::BlockDeviceStats;
use systemstat::Filesystem;
use systemstat::Network;
use systemstat::NetworkAddrs;
use systemstat::NetworkStats;

#[napi(object, js_name = "CPULoad")]
pub struct _CPULoad {
  pub user: f64,
  pub nice: f64,
  pub system: f64,
  pub interrupt: f64,
  pub idle: f64,
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
    let addrs: Vec<_NetworkAddrs> = network.addrs.iter().map(_NetworkAddrs::from).collect();
    _Network {
      name: String::from(network.name.as_str()),
      addrs,
    }
  }
}

#[napi(object, js_name = "NetworkStats")]
pub struct _NetworkStats {
  pub rx_bytes: BigInt,
  pub tx_bytes: BigInt,
  pub rx_packets: BigInt,
  pub tx_packets: BigInt,
  pub rx_errors: BigInt,
  pub tx_errors: BigInt,
}

impl From<NetworkStats> for _NetworkStats {
  fn from(net_stats: NetworkStats) -> Self {
    _NetworkStats {
      rx_bytes: BigInt::from(net_stats.rx_bytes.as_u64()),
      tx_bytes: BigInt::from(net_stats.tx_bytes.as_u64()),
      rx_packets: BigInt::from(net_stats.rx_packets as u64),
      tx_packets: BigInt::from(net_stats.tx_packets as u64),
      rx_errors: BigInt::from(net_stats.rx_errors as u64),
      tx_errors: BigInt::from(net_stats.tx_errors as u64),
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
  pub read_ios: BigInt,
  pub read_merges: BigInt,
  pub read_sectors: BigInt,
  pub read_ticks: BigInt,
  pub write_ios: BigInt,
  pub write_merges: BigInt,
  pub write_sectors: BigInt,
  pub write_ticks: BigInt,
  pub in_flight: BigInt,
  pub io_ticks: BigInt,
  pub time_in_queue: BigInt,
}

impl From<&BlockDeviceStats> for _BlockDeviceStats {
  fn from(b: &BlockDeviceStats) -> Self {
    _BlockDeviceStats {
      name: b.name.as_str().into(),
      read_ios: BigInt::from(b.read_ios as u64),
      read_merges: BigInt::from(b.read_merges as u64),
      read_sectors: BigInt::from(b.read_sectors as u64),
      read_ticks: BigInt::from(b.read_ticks as u64),
      write_ios: BigInt::from(b.write_ios as u64),
      write_merges: BigInt::from(b.write_merges as u64),
      write_sectors: BigInt::from(b.write_sectors as u64),
      write_ticks: BigInt::from(b.write_ticks as u64),
      in_flight: BigInt::from(b.in_flight as u64),
      io_ticks: BigInt::from(b.io_ticks as u64),
      time_in_queue: BigInt::from(b.time_in_queue as u64),
    }
  }
}

#[napi(object, js_name = "BatteryLife")]
pub struct _BatteryLife {
  pub remaining_capacity: f64,
  pub remaining_time: BigInt,
}
