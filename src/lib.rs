#![deny(clippy::all)]

use napi_derive::napi;
use systemstat::{saturating_sub_bytes, Platform, System};
#[napi]
struct Stat {
  sys: System,
}

#[napi]
impl Stat {
  #[napi(constructor)]
  pub fn new() -> Self {
    Stat { sys: System::new() }
  }

  #[napi]
  pub fn cpu_load(&self) {
    println!(
      "{:#?}",
      self.sys.cpu_load_aggregate().unwrap().done().unwrap()
    );
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
