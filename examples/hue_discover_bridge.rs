extern crate huebridge;
use huebridge::bridge::Bridge;

#[allow(dead_code)]
fn main() {
    let bridge = Bridge::discover().unwrap();
    println!("Hue bridge found: {:?}", bridge);
}
