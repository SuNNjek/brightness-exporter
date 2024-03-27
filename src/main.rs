extern crate linux_embedded_hal as hal;

mod bh1750;
use crate::bh1750::{BH1750, MeasurementTime, Resolution};

use envconfig::Envconfig;
use hal::{Delay, I2cdev};
use prometheus_exporter::prometheus::register_gauge;

#[derive(Envconfig)]
struct Config {
    #[envconfig(from = "SENSOR_DEVICE", default = "/dev/i2c-1")]
    pub device: String,

    #[envconfig(from = "SENSOR_RESOLUTION", default = "1.0")]
    pub resolution: Resolution,

    #[envconfig(from = "SENSOR_MEASUREMENT_TIME", default = "69")] // Nice
    pub measurement_time: u8,

    #[envconfig(from = "SERVER_PORT", default = "9186")]
    pub port: u16,
}

fn main() {
    let config = Config::init_from_env().unwrap();

    let dev = I2cdev::new(config.device).unwrap();
    let mut bh1750 = BH1750::new(dev, Delay);

    bh1750.reset().unwrap();
    bh1750.set_resolution(config.resolution);
    bh1750.set_measurement_time(MeasurementTime::Custom(config.measurement_time)).unwrap();

    let binding = format!("0.0.0.0:{}", config.port).parse().unwrap();
    let exporter = prometheus_exporter::start(binding).unwrap();

    let temp = register_gauge!("brightness_lux", "The measured brightness in Lux").unwrap();

    loop {
        let _guard = exporter.wait_request();

        temp.set(bh1750.illuminance().unwrap().into());
    }
}