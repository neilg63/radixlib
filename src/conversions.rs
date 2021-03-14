use crate::fractions::build_bigint_128;
use crate::fractions::float_to_fraction;
use num::bigint::BigInt;
use num::bigint::Sign;
use num::cast::ToPrimitive;
use num::pow;
use std::char;

#[derive(Debug)]
pub struct RadixValue {
  value: String,
  dec_val: f64,
  base: u32,
  mode: String,
  frac: String,
}

impl RadixValue {
  pub fn new(value: String, dec_val: f64, base: u32, mode: String) -> RadixValue {
    let mode_str = match mode.as_str() {
      "r" => "to",
      _ => "from",
    };

    let (bigrat, _diff) = float_to_fraction(dec_val.clone(), 256);

    let frac = fraction_to_units(
      bigrat.numer().to_i32().unwrap(),
      bigrat.denom().to_i32().unwrap(),
      base,
    );
    RadixValue {
      value: value,
      dec_val: dec_val,
      base: base,
      mode: mode_str.to_string(),
      frac: frac,
    }
  }
}

#[derive(Debug, Clone)]
struct DecimalPart {
  integer: i32,
  decimals: f64,
  radval: String,
  base: u32,
  multiple: f64,
  is_negative: bool,
}

impl DecimalPart {
  pub fn new(dec_num: f64, base: u32) -> DecimalPart {
    let decimals = extract_decimals(dec_num);
    let multiple = calculate_radix_multiple_for_pv(base);
    let large = (decimals * multiple) as i128;
    DecimalPart {
      integer: if dec_num >= 0.0 {
        dec_num.floor() as i32
      } else {
        dec_num.ceil() as i32
      },
      decimals: decimals,
      radval: decimal_to_radix_string(large, base),
      base: base,
      multiple: multiple,
      is_negative: dec_num < 0.0,
    }
  }

  pub fn num_zeroes(&self) -> u64 {
    let div = 1.0 / self.decimals;
    div.log(self.base as f64).ceil() as u64 - 1
  }
}

impl ToString for DecimalPart {
  fn to_string(&self) -> String {
    let mut owned_string = decimal_to_radix(self.integer as i128, self.base, self.is_negative);
    if self.decimals != 0.0_f64 {
      owned_string.push_str(".");
      let zero = if self.base > 36 { "00:" } else { "0" };
      owned_string.push_str(zero.repeat(self.num_zeroes() as usize).as_str());
      owned_string.push_str(self.radval.clone().as_str());
    }
    owned_string
  }
}

pub fn extract_decimals(dec_num: f64) -> f64 {
  let dec_multiplier = pow(10.0_f64, 15);
  let mut remainder = dec_num % (1.0 as f64);
  let (bigrat, diff) = float_to_fraction(remainder, 512);
  let denom = bigrat.denom().to_i32().unwrap();
  if denom < 256 && denom > 2 && diff < 1_f64 / 512_f64 {
    let numer = bigrat.numer().to_i32().unwrap();
    remainder = numer as f64 / denom as f64;
  }
  ((remainder * dec_multiplier).ceil() / dec_multiplier).abs()
}

pub fn decimal_to_radix_pv(num: f64, radix: u32) -> String {
  DecimalPart::new(num, radix).to_string()
}

pub fn decimal_to_radix(num: i128, radix: u32, is_negative: bool) -> String {
  integer_to_radix_string(num, radix as u8, is_negative)
}

pub fn decimal_to_radix_string(large: i128, base: u32) -> String {
  let mut str = decimal_to_radix(large, base, large < 0);
  if base <= 36 {
    str = str.trim_end_matches('0').to_string();
  } else {
    let str2 = str.clone();
    let parts = str2.split(":");
    let num_parts = parts.clone().count();
    if num_parts > 1 {
      if num_parts > 8 {
        let mut items: Vec<&str> = parts.collect();
        //let mut items:Vec<String> = ps.iter().map(|c| c.to_string()).collect().join(":").to_string().resize(8,"00".to_string());
        items.resize(8, "00");
        str = items.join(":").to_string();
      }
      str = str.trim_end_matches(":00").to_string();
    }
  }
  str
}

pub fn integer_to_radix_string(num: i128, radix: u8, is_negative: bool) -> String {
  let bg = build_bigint_128(num);
  let (_, vec_nums) = bg.to_radix_be(radix as u32);
  let mut num_chars: Vec<String> = vec_nums
    .iter()
    .map(|c| integer_to_radix_char(*c, radix))
    .collect();
  if is_negative {
    num_chars.insert(0, "-".to_string());
  }
  let separator: &str = if radix > 36 { ":" } else { "" };
  num_chars.join(separator)
}

pub fn integer_to_radix_char(num: u8, radix: u8) -> String {
  let mut str_val = "".to_string();
  if radix > 10 {
    let mut char_num: u8 = num;
    if radix > 36 {
      if radix >= 100 {
        let hundreds = num / 100;
        str_val.push_str(hundreds.to_string().as_str());
      }
      let tens = num / 10;
      str_val.push_str(tens.to_string().as_str());
      char_num = num - (tens * 10);
    }
    if char_num >= 10 {
      if let Some(n) = char::from_u32(87 + char_num as u32) {
        str_val.push_str(n.to_string().as_str());
      }
    } else {
      str_val.push_str(char_num.to_string().as_str());
    }
  } else {
    str_val = num.to_string();
  }
  str_val
}

pub fn radix_to_decimal(num_string: String, radix: u32) -> i32 {
  if radix <= 36 {
    match i32::from_str_radix(num_string.as_str(), radix) {
      Ok(v) => v,
      _ => 0,
    }
  } else {
    radix_be_to_decimal(num_string, radix)
  }
}

pub fn radix_be_to_decimal(num_string: String, radix: u32) -> i32 {
  let mut str_val = num_string.clone();
  let mut str_sign = "+".to_string();
  if num_string.starts_with("-") {
    str_sign = str_val.remove(0).to_string();
  }
  let sign = match str_sign.as_str() {
    "-" => Sign::Minus,
    _ => Sign::Plus,
  };
  let parts: Vec<&str> = str_val.split(":").collect();
  let nums: Vec<u8> = parts
    .iter()
    .map(|c| u8::from_str_radix(c, 10).ok().unwrap())
    .into_iter()
    .collect();
  if let Some(int_val) = BigInt::from_radix_be(sign, &nums, radix) {
    int_val.to_i32().unwrap()
  } else {
    0
  }
}
pub fn radix_to_decimal_pv(num_string: String, radix: u32) -> f64 {
  let parts = num_string.split(".");
  let mut out: f64 = 0.0;
  if let Some(integer_part) = parts.clone().nth(0) {
    out = radix_to_decimal(integer_part.to_string(), radix) as f64;
  }
  if parts.clone().count() > 1 {
    if let Some(mut frac_part) = parts.clone().nth(1) {
      if radix > 36 {
        frac_part = frac_part.trim_end_matches(":00");
      } else {
        frac_part = frac_part.trim_end_matches('0');
      }
      out += radix_frac_to_float(frac_part.to_string(), radix);
    }
  }
  out
}

pub fn radix_frac_to_float(frac_str: String, radix: u32) -> f64 {
  let mut out = 0.0;
  let mut power: u8 = 1;
  if radix <= 36 {
    for n in frac_str.chars() {
      out += radix_to_decimal(n.to_string(), radix) as f64 / pow(radix, power as usize) as f64;
      power += 1;
    }
  } else {
    for n in frac_str.split(":") {
      out += radix_to_decimal(n.to_string(), radix) as f64 / pow(radix, power as usize) as f64;
      power += 1;
    }
  }
  out
}

pub fn calculate_radix_multiple_for_pv(base: u32) -> f64 {
  let start = if base < 25 {
    20
  } else if base < 40 {
    base * 2 / 3
  } else {
    base * 4 / 7
  };
  let dec_len = start as usize - (base / 2) as usize;
  pow(base as f64, dec_len)
}

pub fn fraction_to_units(numer: i32, denom: i32, radix: u32) -> String {
  let units = numer / denom;
  let mut out: String = "".to_string();
  let is_negative = numer < 0;
  if units > 0 {
    out.push_str(decimal_to_radix(units as i128, radix, is_negative).as_str());
  }
  let remainder = numer % denom;
  if remainder > 0 {
    if units > 0 {
      out.push_str(" ");
    }
    out.push_str(decimal_to_radix(remainder as i128, radix, is_negative).as_str());
    out.push_str("/");
    out.push_str(decimal_to_radix(denom as i128, radix, false).as_str());
  }
  out
}

pub fn convert_radix_fraction_to_radix(num_str: String, radix: u32) -> (f64, String) {
  let parts = num_str.split("/");
  let mut dec_val: f64 = 0.0;
  let mut radix_val: String = "".to_string();
  if parts.clone().count() > 1 {
    let numer_radix = parts.clone().nth(0).unwrap();
    let denom_radix = parts.clone().nth(1).unwrap();
    let numer = radix_to_decimal(numer_radix.to_string(), radix as u32);
    let denom = radix_to_decimal(denom_radix.to_string(), radix as u32);
    dec_val = numer as f64 / denom as f64;
    radix_val = decimal_to_radix_pv(dec_val, radix);
  }
  (dec_val, radix_val)
}
