//#![cfg_attr(not(test), no_std)]
#![no_std]
#![no_main]

extern crate alloc;
extern crate panic_halt;

use cortex_m_rt::entry;
use embedded_alloc::Heap;

#[global_allocator]
static HEAP: Heap = Heap::empty();

//use cortex_m::delay::Delay;

use stm32f1xx_hal::{
    i2c::{BlockingI2c, Mode},
    pac,
    prelude::*,
};


use rtt_target::{rtt_init_print, rprintln};

use sensor_lib_aht20 as aht20;

//i2c reserves 0-7 and 120-127 so we need to account for that.
pub const MIN_ADDRESS: u8 = 0x08;
pub const MAX_ADDRESS: u8 = 0x77;

pub const START_TIMEOUT_US: u32 = 1000;
pub const START_RETRIES: u8 = 10;
pub const ADDRESS_TIMEOUT_US: u32 = 1000;
pub const DATA_TIMEOUT_US: u32 = 1000;


fn init_heap() {
    use core::mem::MaybeUninit;
    const HEAP_SIZE: usize = 1024;
    static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
    unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }
}



#[entry]
fn main() -> ! {
    init_heap();
    rtt_init_print!();
    //rprintln!("RTT INITIALIZED\n");

    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();


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


    //let mut delay = delay::Delay::new(cp.SYST, clocks);
    //let mut timer = Timer::syst(cp.SYST, &clocks.clone()).counter_hz();
    //timer.start(10.Hz()).unwrap();
    
    let mut delay = cp.SYST.delay(&clocks);
    
    // Acquire the GPIOB peripheral
    let mut gpiob = dp.GPIOB.split();

    let scl = gpiob.pb6.into_alternate_open_drain(&mut gpiob.crl);
    let sda = gpiob.pb7.into_alternate_open_drain(&mut gpiob.crl);

    let i2c = BlockingI2c::i2c1(
        dp.I2C1,
        (scl, sda),
        &mut alt_func_io.mapr,
        Mode::Standard {
            frequency: 100.kHz(),
            //duty_cycle: DutyCycle::Ratio16to9,
        },
        clocks,
        START_TIMEOUT_US,
        START_RETRIES,
        ADDRESS_TIMEOUT_US,
        DATA_TIMEOUT_US,
    );
   
    //rprintln!("Create instance of AHT20 sensor module");
    let mut sensor_instance = aht20::Sensor::new(i2c, aht20::SENSOR_ADDR);
    
    //rprintln!("Aquired aht20 sensor instance.");

    let mut inited_sensor = sensor_instance.init(&mut delay).unwrap();
    //let sensor_status = inited_sensor.get_status().unwrap();    
    //now try to get some messurements
    let mut sd = inited_sensor.read_sensor(&mut delay).unwrap(); 

    //check the CRC of the data.
    //rprintln!("crc8: {:?}", sd.crc_8_maxim());
    //rprintln!("is_crc_good()-->{:?} ", sd.is_crc_good());
    
    loop {
        rprintln!("Humidity: {}, Temp(C): {}", sd.calculate_humidity(), sd.calculate_temperature());
        delay.delay_ms(1000 as u16);
        sd = inited_sensor.read_sensor(&mut delay).unwrap();
    }

    //now endlessly loop
    //loop{}
}
