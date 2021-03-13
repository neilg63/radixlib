use num::bigint::BigInt;
use num::cast::ToPrimitive;
use num::rational::BigRational;
use num::FromPrimitive;

#[derive(Debug)]
pub struct RationalNumber {
	display: String,
	numer: i32,
	denom: i32,
	value: f64,
	precision: i32,
	difference: f64,
}

impl RationalNumber {
	pub fn new(
		numer: i32,
		denom: i32,
		value: f64,
		precision: i32,
		difference: f64,
	) -> RationalNumber {
		RationalNumber {
			display: fraction_to_units(numer, denom),
			numer: numer,
			denom: denom,
			value: value,
			precision: precision,
			difference: difference,
		}
	}

	pub fn new_from_frac(numer: i32, denom: i32) -> RationalNumber {
		let num: f64 = numer as f64 / denom as f64;
		RationalNumber {
			display: fraction_to_units(numer, denom),
			numer: numer,
			denom: denom,
			value: num,
			precision: 0,
			difference: 0.0,
		}
	}

	pub fn new_from_big_rational(big_rat: BigRational) -> RationalNumber {
		RationalNumber::new_from_frac(
			big_rat.numer().to_i32().unwrap(),
			big_rat.denom().to_i32().unwrap(),
		)
	}
}

#[derive(Debug)]
pub struct RationalNumberSet {
	ratnum: RationalNumber,
	code: u16,
	msg: String,
}

impl RationalNumberSet {
	pub fn new(ratnum: RationalNumber) -> RationalNumberSet {
		RationalNumberSet {
			ratnum: ratnum,
			code: 200,
			msg: "OK".to_string(),
		}
	}

	pub fn new_from_big_rational(bigrat: BigRational) -> RationalNumberSet {
		let ratnum = RationalNumber::new_from_big_rational(bigrat);
		RationalNumberSet {
			ratnum: ratnum,
			code: 200,
			msg: "OK".to_string(),
		}
	}
}

pub fn fraction_string_to_big_rational(number: String) -> BigRational {
	let parts: Vec<&str> = number.as_str().clone().split("/").collect();
	let mut numer: i32 = 0;
	let mut denom: i32 = 0;
	if parts.len() == 2 {
		numer = parts.get(0).unwrap().parse::<i32>().ok().unwrap();
		denom = parts.get(1).unwrap().parse::<i32>().ok().unwrap();
	}
	build_big_rational(numer, denom)
}

pub fn float_to_fraction(num: f64, precision: i32) -> (BigRational, f64) {
	let mut numer: i32 = num as i32;
	let mut demon: i32 = 1;
	let max = precision + 1;
	let max_dec: f64 = 1.0_f64 / max as f64;
	let mut difference: f64 = 0.0;

	for i in 1..max {
		if let Some((n, diff)) = is_divisable(num, i, max_dec) {
			numer = (n as f64 * num).round() as i32;
			demon = n as i32;
			difference = diff;
			break;
		}
	}
	(build_big_rational(numer, demon), difference)
}

pub fn is_divisable(num: f64, i: i32, tolerance: f64) -> Option<(i32, f64)> {
	let diff = num * i as f64 % 1.0;
	if diff <= tolerance {
		return Some((i, diff));
	} else if diff >= (1.0 - tolerance) {
		return Some((i, 1.0 - diff));
	} else {
		return None;
	}
}

pub fn fraction_to_units(numer: i32, denom: i32) -> String {
	let units = numer / denom;
	let mut out: String = "".to_string();
	if units > 0 {
		out.push_str(units.to_string().as_str());
	}
	let remainder = numer % denom;
	if remainder > 0 {
		if units > 0 {
			out.push_str(" ");
		}
		out.push_str(remainder.to_string().as_str());
		out.push_str("/");
		out.push_str(denom.to_string().as_str());
	}
	out
}

pub fn float_to_fraction_parts(dec_val: f64, precision: i32) -> (i32, i32, f64) {
	let (bigrat, diff) = float_to_fraction(dec_val, precision);
	let numer = bigrat.numer().to_i32().unwrap();
	let denom = bigrat.denom().to_i32().unwrap();
	(numer, denom, diff)
}

pub fn build_big_rational(numer: i32, denom: i32) -> BigRational {
	BigRational::new(build_bigint(numer), build_bigint(denom)).reduced()
}

pub fn build_bigint(integer: i32) -> BigInt {
	BigInt::from_i32(integer).unwrap()
}
