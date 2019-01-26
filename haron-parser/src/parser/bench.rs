extern crate test;

use super::parse_string;

#[bench]
fn name(b: &mut test::Bencher) {
    let input = "\r\n\r\nDAT 1, 1  \n\nMOV 1\n\n\nSPL ,1\n".into();

    b.iter(|| parse_string(input))
}
