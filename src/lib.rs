use wasm_bindgen::prelude::*;
extern crate num;

pub use crate::conversions::convert_radix_fraction_to_radix;
pub use crate::conversions::decimal_to_radix_pv;
pub use crate::conversions::fraction_to_units;
pub use crate::conversions::radix_to_decimal_pv;
pub use crate::fractions::float_to_fraction_parts;

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

#[cfg(test)]
mod tests {
    pub use crate::conversions::decimal_to_radix_pv;
    #[test]
    fn base_12_fraction() {
        assert_eq!(decimal_to_radix_pv(0.5, 12), "0.6".to_string());
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
        assert_eq!(decimal_to_radix_pv(0.5, 36), "0.i".to_string());
    }

    #[test]
    fn base_60_fraction() {
        assert_eq!(decimal_to_radix_pv(62.5, 60), "01:02.30".to_string());
    }
}
