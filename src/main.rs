mod utils;

fn main() {
    let mut res = String::new();
    const MX: i32 = 10_000_000;
    println!("Gen start");
    let mut tot: i64 = 0;
    let start_write = std::time::Instant::now();
    for i in 0..MX {
        if !res.is_empty() {
            res.push_str(" ");
        }
        res.push_str(i.to_string().as_str());
        tot += i as i64;
    }
    println!("{:?}", start_write.elapsed());
    let start_read = std::time::Instant::now();
    /*
    for str in res.split(" ") {
        tot -= str.parse::<i64>().unwrap();
    }
    */
    let mut data = res.as_str().as_bytes();
    for _ in 0..MX {
        tot -= utils::io::next_i32(&mut data) as i64;
    }
    assert!(tot == 0);
    println!("{:?}", start_read.elapsed());
}
