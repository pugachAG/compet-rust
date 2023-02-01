use super::includes::*;

pub fn check(std_io: &mut Io, driver_io: &mut Io) {
    let t = 1usize;
    output! { driver_io => t; }
    for _tc in 0..t {
    }
    output! { std_io => "OK"; }
}
