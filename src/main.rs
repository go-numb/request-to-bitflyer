extern crate reqwest;

use std::collections::HashMap;
use std::io::Read;

use std::time::{Duration, Instant};
use std::thread::sleep;




fn main() -> Result<(), Box<std::error::Error>> {
    for i in 1..10 {
          let start = Instant::now();

    let mut resp = reqwest::get("https://api.bitflyer.com/v1/getticker?product_code=FX_BTC_JPY")?;
    let mut body = String::new();
    resp.read_to_string(&mut body)?;
    println!("{:#?}", resp);
    println!("{:#?}", body);

          let end = start.elapsed();
  println!("{}.{:03}秒経過", end.as_secs(), end.subsec_nanos() / 1_000_000);

sleep(Duration::from_millis(1));

    }
        Ok(())
}
