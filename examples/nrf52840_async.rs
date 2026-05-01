#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_nrf::twim::{self, Twim};
use embassy_nrf::{bind_interrupts, peripherals};

use embassy_time::{Duration, Timer};
use static_cell::ConstStaticCell;
use veml6030::builder::VemlBuilder;
use {defmt_rtt as _, embassy_nrf as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    TWISPI0 => twim::InterruptHandler<peripherals::TWISPI0>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_nrf::init(Default::default());
    info!("nRF52840 initialized successfully!");

    let config = twim::Config::default();

    static RAM_BUFFER: ConstStaticCell<[u8; 16]> = ConstStaticCell::new([0; 16]);
    let i2c = Twim::new(p.TWISPI0, Irqs, p.P0_06, p.P0_08, config, RAM_BUFFER.take());

    let mut sensor = match VemlBuilder::new(i2c, 0x10)
        .gain(veml6030::regs::ALSGain::AlsX1)
        .integration_time(veml6030::regs::ALSIt::It800ms)
        .build()
        .await
    {
        Ok(s) => s,
        Err(_) => {
            error!("VEML6030 Init failed");
            return;
        }
    };

    if let Ok(id_info) = sensor.device_id().await {
        info!(
            "VEML6030 initialized successfully!   ID: 0x{:x}{:x}",
            id_info.option_code, id_info.device_id
        );
    } else {
        error!("VEML6030 device_id read failed");
    }

    info!("Starting light level monitoring...");

    loop {
        if let Ok(als) = sensor.read_als().await {
            info!("Light Level (ALS): {}", als);
        }
        Timer::after(Duration::from_secs(1)).await;
    }
}
