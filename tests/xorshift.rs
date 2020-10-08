use raytrace_rs::xorshift::*;

use std::time::SystemTime;
use raytrace_rs::xorshift::*;

#[test]
fn test_next_bounded() {
    let mut rand = XorShift::new(SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos() as u64);
    for _i in 0..100 {
        eprintln!("{}", rand.next_bounded(-1.0, 1.0));
    }
}