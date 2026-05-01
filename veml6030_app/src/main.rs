#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_nrf::gpio::{Input, Level, Output, OutputDrive, Pull};
use embassy_nrf::twim::{self, Twim};
use embassy_nrf::{bind_interrupts, peripherals};
//use embassy_time::{Duration, Timer};
use static_cell::StaticCell;
use veml6030_driver::builder::VemlBuilder;
use veml6030_driver::regs::{ALSGain, ALSIt};
use {defmt_rtt as _, embassy_nrf as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    TWISPI0 => twim::InterruptHandler<peripherals::TWISPI0>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("VEML6030 Interrupt Example Starting...");
    let p = embassy_nrf::init(Default::default());

    let mut led = Output::new(p.P0_04, Level::High, OutputDrive::Standard);

    let mut int_pin = Input::new(p.P0_14, Pull::None);

    let config = twim::Config::default();
    static RAM_BUFFER: StaticCell<[u8; 16]> = StaticCell::new();
    let i2c = Twim::new(
        p.TWISPI0,
        Irqs,
        p.P0_06,
        p.P0_08,
        config,
        RAM_BUFFER.init([0; 16]),
    );

    let min = 0.0;
    let max = 350.0;
    let mut sensor = match VemlBuilder::new(i2c, 0x48)
        .gain(ALSGain::AlsX2)
        .integration_time(ALSIt::It100ms)
        .interrupt_enabled()
        .thresholds_in_lux(min, max)
        .build()
        .await
    {
        Ok(s) => s,
        Err(e) => {
            error!("Init failed: {:?}", e);
            return;
        }
    };

    let config = sensor.read_reg(0x00).await.unwrap();
    info!("Final Config Register: 0x{:04x}", config);
    info!("Monitoring started. Safety range: {} to {}", min, max);
    led.set_high();

    loop {
        use embassy_futures::select::{select, Either};
        use embassy_time::{Duration, Timer};

        let result = select(
            sensor.wait_interrupt(&mut int_pin),
            Timer::after(Duration::from_secs(2)),
        )
        .await;

        match result {
            Either::First(_) => {
                if let Ok(status) = sensor.read_int_status().await {
                    if status.int_th_high {
                        warn!("INTERRUPT: ALS is above threshold! LED ON.");
                        led.set_low();
                    } else if status.int_th_low {
                        warn!("INTERRUPT: ALS is below threshold! LED OFF.");
                        led.set_high();
                    }
                }
            }
            Either::Second(_) => {
                if let Ok(lux) = sensor.read_lux().await {
                    let status = sensor.read_int_status().await.unwrap_or_default();

                    let pin_level = int_pin.is_low();

                    info!(
                        "Heartbeat - Lux: {}, IntStatus: {:?}, PinLow: {}",
                        lux, status, pin_level
                    );

                    if lux >= min && lux <= max {
                        led.set_high();
                    } else if status.int_th_high || status.int_th_low {
                        warn!("!!! Found interrupt flag in register but select didn't catch it! PinLow: {}", pin_level);
                    }
                }
            }
        }
    }
}
