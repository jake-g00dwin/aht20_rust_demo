#![deny(unsafe_code)]
#![no_std]
#![no_main]

extern crate panic_halt;

use nb::block;
use cortex_m_rt::entry;
use stm32f1xx_hal::{
    timer::Timer,
    i2c::{BlockingI2c, DutyCycle, Mode},
    pac,
    prelude::*,
};


use rtt_target::{rtt_init_print, rprintln};

//i2c reserves 0-7 and 120-127 so we need to account for that.
pub const MIN_ADDRESS: u8 = 0x08;
pub const MAX_ADDRESS: u8 = 0x77;

pub const START_TIMEOUT_US: u32 = 1000;
pub const START_RETRIES: u8 = 10;
pub const ADDRESS_TIMEOUT_US: u32 = 1000;
pub const DATA_TIMEOUT_US: u32 = 1000;


#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    rtt_init_print!();
    rprintln!("RTT INITIALIZED\n");

    //take ownership of raw falsh and rcc.
    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();
    let mut alt_func_io = dp.AFIO.constrain();

    //freeze config of clocks and store in var.
    let clocks = if 1 == 1 {
        rcc.cfgr.use_hse(8.MHz()).freeze(&mut flash.acr)
    }
    else {
        rcc.cfgr
            .use_hse(8.MHz())
            .sysclk(48.MHz())
            .pclk1(6.MHz())
            .freeze(&mut flash.acr)
    };


    let mut timer = Timer::syst(cp.SYST, &clocks).counter_hz();
    timer.start(10.Hz()).unwrap();
    
    // Acquire the GPIOB peripheral
    let mut gpiob = dp.GPIOB.split();

    let scl = gpiob.pb6.into_alternate_open_drain(&mut gpiob.crl);
    let sda = gpiob.pb7.into_alternate_open_drain(&mut gpiob.crl);

    let mut i2c = BlockingI2c::i2c1(
        dp.I2C1,
        (scl, sda),
        &mut alt_func_io.mapr,
        Mode::Fast {
            frequency: 400.kHz(),
            duty_cycle: DutyCycle::Ratio16to9,
        },
        clocks,
        START_TIMEOUT_US,
        START_RETRIES,
        ADDRESS_TIMEOUT_US,
        DATA_TIMEOUT_US,
    );


    let mut _write_buffer: [u8; 1] = [0];
    let mut read_buffer: [u8; 1] = [0];
    let mut res: Result<(), stm32f1xx_hal::i2c::Error>;

    rprintln!("Scanning for i2c devices...");
    for address in MIN_ADDRESS..MAX_ADDRESS {
        rprintln!("Calling Address: {:#02x?}", address);

        //Try to read from address, basically just check for ACK bit. 
        res = i2c.read(address, &mut read_buffer);

        match res {
            Ok(()) => rprintln!("Found device!"),
            Err(stm32f1xx_hal::i2c::Error::Acknowledge) => rprintln!("no ACK\n"),
            Err(e) => rprintln!("Other: {:?}\n", e)
        }

        //wait for a tiny bit.
        block!(timer.wait()).unwrap();
    }
 
    //We've ran out of valid 7bit addresses.
    rprintln!("DONE Searching");

    //now endlessly loop
    loop{}
}
