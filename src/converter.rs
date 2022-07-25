use std::fmt::Display;

use napi::bindgen_prelude::ToNapiValue;
use napi_derive::napi;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[allow(clippy::enum_clike_unportable_variant)]
#[derive(EnumIter, Debug, PartialEq, Clone, Copy)]
pub enum Unit {
  B = 1,
  KB = 1_000,
  MB = 1_000_000,
  GB = 1_000_000_000,
  TB = 1_000_000_000_000,
  PB = 1_000_000_000_000_000,
  KIB = 1_024,
  MIB = 1_048_576,
  GIB = 1_073_741_824,
  TIB = 1_099_511_627_776,
  PIB = 1_125_899_906_842_624,
}

impl Display for Unit {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Unit::B => write!(f, "B"),
      Unit::KB => write!(f, "KB"),
      Unit::MB => write!(f, "MB"),
      Unit::GB => write!(f, "GB"),
      Unit::TB => write!(f, "TB"),
      Unit::PB => write!(f, "PB"),
      Unit::KIB => write!(f, "KIB"),
      Unit::MIB => write!(f, "MIB"),
      Unit::GIB => write!(f, "GIB"),
      Unit::TIB => write!(f, "TIB"),
      Unit::PIB => write!(f, "PIB"),
    }
  }
}

/// unit enum for stat option
/// B -> KB, 1000 bytes
/// B -> KIB, 1024 bytes
#[napi(object)]
#[derive(Debug)]
pub enum Advance {
  /// 1000
  Kilobase = 1000,
  /// 1024
  Binary = 1024,
}

#[napi(object)]
#[derive(Debug, Clone, Copy)]
pub struct ConvertOption {
  pub advance: Advance,
  /// Decimal point
  pub precision: u8,
}

impl Default for ConvertOption {
  fn default() -> Self {
    ConvertOption {
      advance: Advance::Binary,
      precision: 1,
    }
  }
}

#[napi(object)]
#[derive(Debug)]
pub struct ConvertResult {
  pub value: String,
  pub unit: String,
}

pub trait Converter<T: Into<u64>> {
  fn convert(source: T, option: ConvertOption) -> ConvertResult;
}

impl Converter<u64> for u64 {
  fn convert(source: u64, option: ConvertOption) -> ConvertResult {
    let source = source as f64;
    let advance = option.advance as u64 as f64;
    let precision = option.precision as usize;

    let mut current_advance = Unit::B as u64 as f64;

    while source / current_advance >= advance {
      if current_advance == Unit::PB as u64 as f64 || current_advance == Unit::PIB as u64 as f64 {
        break;
      }
      current_advance *= advance;
    }

    let unit = Unit::iter().find(|u| *u as u64 as f64 == current_advance);

    ConvertResult {
      value: format!("{:.precision$}", source / current_advance),
      unit: unit.unwrap().to_string(),
    }
  }
}
