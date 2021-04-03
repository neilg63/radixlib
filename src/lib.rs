use wasm_bindgen::prelude::*;
extern crate meval;
extern crate num;
pub use crate::conversions::convert_radix_fraction_to_radix;
pub use crate::conversions::decimal_to_radix_pv;
pub use crate::conversions::fraction_to_units;
pub use crate::conversions::radix_to_decimal_pv;
pub use crate::fractions::float_to_fraction_parts;
pub use crate::fractions::RationalNumber;

mod conversions;
mod fractions;

#[wasm_bindgen]
pub struct NumString {
  num: f64,
  text: String,
}

#[wasm_bindgen]
impl NumString {
  pub fn new(num: f64, text: String) -> NumString {
    NumString {
      num: num,
      text: text,
    }
  }

  pub fn as_float(&self) -> f64 {
    self.num
  }

  pub fn as_string(&self) -> String {
    self.text.as_str().to_string()
  }
}

const MAX32: i32 = 2147483647;

#[wasm_bindgen]
pub struct Fraction {
  numer: i32,
  denom: i32,
  diff: f64,
}

#[wasm_bindgen]
impl Fraction {
  pub fn new(numer: i32, denom: i32, diff: f64) -> Fraction {
    Fraction {
      numer: numer,
      denom: denom,
      diff: diff,
    }
  }

  pub fn numerator(&self) -> i32 {
    if self.numer >= MAX32 {
      0
    } else {
      self.numer as i32
    }
  }

  pub fn denominator(&self) -> i32 {
    if self.denom >= MAX32 || self.denom <= (0 - MAX32) {
      0
    } else {
      self.denom as i32
    }
  }

  pub fn difference(&self) -> f64 {
    self.diff
  }
}

#[wasm_bindgen]
pub fn decimal_to_radix(large: f64, base: u32) -> String {
  decimal_to_radix_pv(large, base)
}

#[wasm_bindgen]
pub fn radix_to_decimal(rad_val: String, base: u32) -> f64 {
  radix_to_decimal_pv(rad_val, base)
}

#[wasm_bindgen]
pub fn fraction_to_unit(numer: i32, denom: i32, base: u32) -> String {
  fraction_to_units(numer, denom, base)
}

#[wasm_bindgen]
pub fn radix_fraction_to_radix(num_string: String, base: u32) -> NumString {
  let (num, text) = convert_radix_fraction_to_radix(num_string, base);
  NumString {
    num: num,
    text: text,
  }
}

#[wasm_bindgen]
pub fn float_to_fraction(dec_val: f64, precision: i32) -> Fraction {
  let (numer, denom, diff) = float_to_fraction_parts(dec_val, precision);
  Fraction::new(numer, denom, diff)
}

#[wasm_bindgen]
pub fn expr_to_f64(expr: String) -> f64 {
  let result: meval::Expr = expr.as_str().parse().unwrap();
  result.eval().ok().unwrap()
}

#[wasm_bindgen]
pub fn expr_to_radix(expr: String, base: u32) -> String {
  let num = expr_to_f64(expr);
  decimal_to_radix(num, base)
}

#[wasm_bindgen]
pub fn frac_expr_to_big_rational(expr: String) -> RationalNumber {
  let components: Vec<i32> = expr
    .split("/")
    .into_iter()
    .map(|item| item.trim().parse::<i32>().unwrap())
    .collect();
  let numer = components[0];
  let denom = components[1];
  RationalNumber::new_from_frac(numer, denom)
}

#[cfg(test)]
mod tests {
  pub use crate::conversions::decimal_to_radix_pv;
  pub use crate::expr_to_f64;
  pub use crate::frac_expr_to_big_rational;
  pub use crate::fractions::float_to_fraction_parts;
  pub use crate::fractions::RationalNumber;
  pub use crate::expr_to_radix;

  #[test]
  fn base_2_fraction() {
    assert_eq!(decimal_to_radix_pv(1.5, 2), "1.1".to_string());
  }

  #[test]
  fn base_6_fraction() {
    assert_eq!(decimal_to_radix_pv(2.166666666, 6), "2.1".to_string());
  }

  #[test]
  fn base_12_fraction() {
    assert_eq!(decimal_to_radix_pv(0.5, 12), "0.6".to_string());
  }

  #[test]
  fn base_12_float_fraction() {
    // test approximation
    assert_eq!(
      decimal_to_radix_pv(0.33333333333 as f64, 12),
      "0.4".to_string()
    );
  }

  #[test]
  fn base_12_float_fraction_approx() {
    // test approximation
    assert_eq!(
      decimal_to_radix_pv(0.111111111 as f64, 12),
      "0.14".to_string()
    );
  }

  #[test]
  fn base_16_fraction() {
    assert_eq!(decimal_to_radix_pv(0.125, 16), "0.2".to_string());
  }

  #[test]
  fn base_20_fraction() {
    assert_eq!(decimal_to_radix_pv(26.75, 20), "16.f".to_string());
  }

  #[test]
  fn base_36_fraction() {
    assert_eq!(decimal_to_radix_pv(67.333333333333, 36), "1v.c".to_string());
  }

  #[test]
  fn base_36_float_fraction() {
    assert_eq!(
      decimal_to_radix_pv(4.0 / 23.0, 36).starts_with("0.69e34p"),
      true
    );
  }

  #[test]
  fn base_60_fraction() {
    assert_eq!(decimal_to_radix_pv(62.5, 60), "01:02.30".to_string());
  }
  #[test]
  fn base_12_negative() {
    assert_eq!(decimal_to_radix_pv(-0.75, 12), "-0.9".to_string());
  }

  #[test]
  fn base_10_negative_rational_fraction() {
    assert_eq!(float_to_fraction_parts(-1.75, 4096), (-7, 4, 0.0));
  }

  #[test]
  fn text_expr_to_f64() {
    assert_eq!(expr_to_f64("(24 / 2) + 5 * 7".to_string()), 47f64);
  }

  #[test]
  fn text_expr_to_f64_frac() {
    assert_eq!(expr_to_f64("1 / 7".to_string()), (1f64 / 7f64));
  }

  #[test]
  fn text_expr_to_f64_with_power() {
    assert_eq!(expr_to_f64("4 ^ 0.5".to_string()), 2f64);
  }

  #[test]
  fn text_expr_to_radix_with_power_12() {
    assert_eq!(expr_to_radix("12 ^ 8".to_string(), 12), "100000000");
  }

  #[test]
  fn text_expr_to_f64_with_decimal() {
    assert_eq!(expr_to_f64("4 + 0.5".to_string()), 4.5f64);
  }

  #[test]
  fn text_expr_to_rational_number() {
    assert_eq!(
      frac_expr_to_big_rational("3 / 2".to_string()),
      RationalNumber::new_from_frac(3, 2)
    );
  }

  #[test]
  fn text_expr_to_radix() {
    assert_eq!(expr_to_radix("1 / 3".to_string(), 12), "0.4".to_string());
  }

  #[test]
  fn text_expr_to_radix_2() {
    let result = expr_to_radix("1 / 7".to_string(), 12);
    println!("{}", result);
    assert_eq!(result.starts_with("0.186a35186a35"), true);
  }
}
