#![no_main]

use libfuzzer_sys::fuzz_target;
use poly::{Poly, new_from_slice};
use z2z::Z2z;
use num::Zero;

fuzz_target!(|data: (&[u8], &[u8])| {
    let p0 = new_from_slice(data.0);
    let p1 = new_from_slice(data.1);
    if !p0.is_zero() {
        assert_eq!((p0.clone() * p1.clone()) / p0.clone(), (p1.clone(), Poly::<Z2z>::zero()));
    }
});
