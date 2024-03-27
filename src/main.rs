extern crate linux_embedded_hal as hal;

mod bh1750;
use crate::bh1750::{BH1750, MeasurementTime, Resolution};

use hal::{Delay, I2cdev};
use prometheus_exporter::prometheus::register_gauge;

fn main() {
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let mut bh1750 = BH1750::new(dev, Delay);

    bh1750.reset().unwrap();
    bh1750.set_resolution(Resolution::Lx0_5);
    bh1750.set_measurement_time(MeasurementTime::Custom(254)).unwrap();

    let binding = "0.0.0.0:9186".parse().unwrap();
    let exporter = prometheus_exporter::start(binding).unwrap();

    let temp = register_gauge!("brightness_lux", "The measured brightness in Lux").unwrap();

    loop {
        let _guard = exporter.wait_request();

        temp.set(bh1750.illuminance().unwrap().into());
    }
}