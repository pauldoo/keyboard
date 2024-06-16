#![no_std]
#![no_main]

use cortex_m::delay::Delay;
use cortex_m::prelude::*;

use debounce::DebounceState;
use embedded_hal::digital::InputPin;
use key_table::KEY_MAPPING;
// The macro for our start-up function
use rp_pico::entry;

use fugit::ExtU32;

// GPIO traits
use embedded_hal::digital::OutputPin;

// Ensure we halt the program on panic (if we don't mention this crate it won't
// be linked)
use panic_halt as _;

use rp_pico::hal::gpio::DynPinId;
use rp_pico::hal::gpio::FunctionSio;
use rp_pico::hal::gpio::Pin;
use rp_pico::hal::gpio::PullDown;
use rp_pico::hal::gpio::SioInput;
use rp_pico::hal::gpio::SioOutput;
// Pull in any important traits
use rp_pico::hal::prelude::*;

// A shorter alias for the Peripheral Access Crate, which provides low-level
// register access
use rp_pico::hal::pac;

// A shorter alias for the Hardware Abstraction Layer, which provides
// higher-level drivers.
use rp_pico::hal;

use rp_pico::hal::Timer;
use usb_device::{class_prelude::*, prelude::*};

use usbd_human_interface_device::prelude::*;
use usbd_human_interface_device::device::keyboard::NKROBootKeyboardConfig;
use usbd_human_interface_device::page::Keyboard;

mod debounce;
mod key_table;

pub(crate) const KEY_ROWS: usize = 6;
pub(crate) const KEY_COLUMNS: usize = 17;

/// Period for polling for USB I/O.
const TICK_PERIOD_MS: u32 = 1;
/// Period between scans for key presses.
const SCAN_PERIOD_MS: u32 = 5;

/// Entry point to our bare-metal application.
///
/// The `#[entry]` macro ensures the Cortex-M start-up code calls this function
/// as soon as all global variables are initialised.
///
/// The function configures the RP2040 peripherals, then blinks the LED in an
/// infinite loop.
#[entry]
fn main() -> ! {
    // Grab our singleton objects
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    // Set up the watchdog driver - needed by the clock setup code
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    // Configure the clocks
    //
    // The default is to generate a 125 MHz system clock
    let clocks = hal::clocks::init_clocks_and_plls(
        rp_pico::XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let timer = Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);

    // The single-cycle I/O block controls our GPIO pins
    let sio = hal::Sio::new(pac.SIO);

    // Set the pins up according to their function on this particular board
    let pins = rp_pico::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // Set up the USB driver
    let usb_bus = UsbBusAllocator::new(hal::usb::UsbBus::new(
        pac.USBCTRL_REGS,
        pac.USBCTRL_DPRAM,
        clocks.usb_clock,
        true,
        &mut pac.RESETS,
    ));

    let mut keyboard = UsbHidClassBuilder::new()
        .add_device(NKROBootKeyboardConfig::default())
        .build(&usb_bus);

    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x1209, 0x0001))
        .strings(&[StringDescriptors::default().product("Crappy Keyboard")])
        .unwrap()
        .build();

    // The delay object lets us wait for specified amounts of time (in
    // milliseconds)
    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    // Set the LED to be an output
    let mut led_pin = pins.led.into_push_pull_output();

    let mut row_pins: [&mut Pin<DynPinId, FunctionSio<SioOutput>, PullDown>; KEY_ROWS] = [
        &mut pins.gpio17.into_push_pull_output().into_dyn_pin(),
        &mut pins.gpio18.into_push_pull_output().into_dyn_pin(),
        &mut pins.gpio19.into_push_pull_output().into_dyn_pin(),
        &mut pins.gpio20.into_push_pull_output().into_dyn_pin(),
        &mut pins.gpio21.into_push_pull_output().into_dyn_pin(),
        &mut pins.gpio22.into_push_pull_output().into_dyn_pin(),
    ];

    let mut column_pins: [&mut Pin<DynPinId, FunctionSio<SioInput>, PullDown>; KEY_COLUMNS] = [
        &mut pins.gpio0.into_pull_down_input().into_dyn_pin(),
        &mut pins.gpio1.into_pull_down_input().into_dyn_pin(),
        &mut pins.gpio2.into_pull_down_input().into_dyn_pin(),
        &mut pins.gpio3.into_pull_down_input().into_dyn_pin(),
        &mut pins.gpio4.into_pull_down_input().into_dyn_pin(),
        &mut pins.gpio5.into_pull_down_input().into_dyn_pin(),
        &mut pins.gpio6.into_pull_down_input().into_dyn_pin(),
        &mut pins.gpio7.into_pull_down_input().into_dyn_pin(),
        &mut pins.gpio8.into_pull_down_input().into_dyn_pin(),
        &mut pins.gpio9.into_pull_down_input().into_dyn_pin(),
        &mut pins.gpio10.into_pull_down_input().into_dyn_pin(),
        &mut pins.gpio11.into_pull_down_input().into_dyn_pin(),
        &mut pins.gpio12.into_pull_down_input().into_dyn_pin(),
        &mut pins.gpio13.into_pull_down_input().into_dyn_pin(),
        &mut pins.gpio14.into_pull_down_input().into_dyn_pin(),
        &mut pins.gpio15.into_pull_down_input().into_dyn_pin(),
        &mut pins.gpio16.into_pull_down_input().into_dyn_pin(),
    ];

    led_pin.set_low().unwrap();

    let mut debounce_states: [[DebounceState; KEY_COLUMNS]; KEY_ROWS] = [[DebounceState::default(); KEY_COLUMNS]; KEY_ROWS];

    let mut tick_count_down = timer.count_down();
    tick_count_down.start(TICK_PERIOD_MS.millis());

    let mut scan_count_down = timer.count_down();
    scan_count_down.start(SCAN_PERIOD_MS.millis());

    // Way more than we ever need.
    let mut report_buffer: [Keyboard; 200] = [Keyboard::NoEventIndicated; 200];

    loop {
        if tick_count_down.wait().is_ok() {
            let _ = keyboard.tick();
        }

        if scan_count_down.wait().is_ok() {
            let report: &[Keyboard] = scan_keys(
                &mut row_pins, &mut column_pins, &mut delay, &mut debounce_states, &mut report_buffer);
            
            if report.is_empty() {
                led_pin.set_low().unwrap()
            } else {
                led_pin.set_high().unwrap()
            }

            let _ = keyboard.device().write_report(report.iter().copied());
        }

        if usb_dev.poll(&mut [&mut keyboard]) {
            let _ = keyboard.device().read_report().map(|_leds| {});
        }
    }
}

fn scan_keys<'a>(
    row_pins: &mut [&mut Pin<DynPinId, FunctionSio<SioOutput>, PullDown>; KEY_ROWS],
    column_pins: &mut [&mut Pin<DynPinId, FunctionSio<SioInput>, PullDown>; KEY_COLUMNS],
    delay: &mut Delay,
    debounce_states: &mut [[DebounceState; KEY_COLUMNS]; KEY_ROWS],
    buffer: &'a mut [Keyboard]
) -> &'a [Keyboard] {

    let mut out_idx: usize = 0;

    assert_eq!(KEY_MAPPING.len(), row_pins.len());
    for (row_idx, row_mapping) in KEY_MAPPING.iter().enumerate() {
        row_pins[row_idx].set_high().unwrap();
        delay.delay_us(1);

        assert_eq!(row_mapping.len(), column_pins.len());
        for (col_idx, key) in row_mapping.iter().enumerate() {
            if *key != Keyboard::NoEventIndicated {
                let input = column_pins[col_idx].is_high().unwrap();

                let is_depressed = debounce_states[row_idx][col_idx].update(input);
                if is_depressed {
                    buffer[out_idx] = *key;
                    out_idx += 1;
                }
            }
        }
    }

    &buffer[..out_idx]
}
