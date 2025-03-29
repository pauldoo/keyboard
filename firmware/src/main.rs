#![no_std]
#![no_main]

use cortex_m::delay::Delay;
use cortex_m::prelude::*;

use debounce::DebounceState;
use embedded_hal::digital::InputPin;
use key_table::KeyFunction;
use key_table::KEY_MAPPING;
// The macro for our start-up function
use rp_pico::entry;

use frunk::HList;
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

use pac::interrupt;

use rp_pico::hal::Timer;
use usb_device::{class_prelude::*, prelude::*};

use usbd_human_interface_device::device::consumer::ConsumerControl;
use usbd_human_interface_device::device::consumer::ConsumerControlConfig;
use usbd_human_interface_device::device::consumer::MultipleConsumerReport;
use usbd_human_interface_device::device::keyboard::NKROBootKeyboard;
use usbd_human_interface_device::device::keyboard::NKROBootKeyboardConfig;
use usbd_human_interface_device::page::Consumer;
use usbd_human_interface_device::page::Keyboard;
use usbd_human_interface_device::prelude::*;

mod debounce;
mod key_table;

pub(crate) const KEY_ROWS: usize = 6;
pub(crate) const KEY_COLUMNS: usize = 17;

/// Period for calling tick() on the USB HID, and scanning the switch matrix.
const HID_TICK_AND_MATRIX_SCAN_PERIOD_MS: u32 = 1;
/// Period between sending keyboard reports.
const KEYBOARD_REPORT_PERIOD_MS: u32 = 10;
/// Period between sending consumer (media key) reports.
const CONSUMER_REPORT_PERIOD_MS: u32 = 50;

type UsbMultiDev = UsbHidClass<
    'static,
    hal::usb::UsbBus,
    HList!(
        ConsumerControl<'static, hal::usb::UsbBus>,
        NKROBootKeyboard<'static, hal::usb::UsbBus>
    ),
>;

static mut USB_BUS: Option<UsbBusAllocator<hal::usb::UsbBus>> = None;

static mut USB_DEV: Option<UsbDevice<hal::usb::UsbBus>> = None;

static mut MULTI_DEV: Option<UsbMultiDev> = None;

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

    {
        let usb_bus: UsbBusAllocator<hal::usb::UsbBus> =
            UsbBusAllocator::new(hal::usb::UsbBus::new(
                pac.USBCTRL_REGS,
                pac.USBCTRL_DPRAM,
                clocks.usb_clock,
                true,
                &mut pac.RESETS,
            ));

        unsafe {
            USB_BUS = Some(usb_bus);
        }
    }

    {
        let usb_bus = unsafe { USB_BUS.as_ref() }.unwrap();
        let multi = UsbHidClassBuilder::new()
            .add_device(NKROBootKeyboardConfig::default())
            .add_device(ConsumerControlConfig::default())
            .build(usb_bus);
        unsafe {
            MULTI_DEV = Some(multi);
        }
    }

    {
        let usb_bus = unsafe { USB_BUS.as_ref() }.unwrap();
        let usb_dev: UsbDevice<hal::usb::UsbBus> =
            UsbDeviceBuilder::new(usb_bus, UsbVidPid(0x1209, 0x0001))
                .strings(&[StringDescriptors::default().product("Crappy Keyboard")])
                .unwrap()
                .build();
        unsafe {
            USB_DEV = Some(usb_dev);
        }
    }

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

    let mut debounce_states: [[DebounceState; KEY_COLUMNS]; KEY_ROWS] = Default::default();

    let mut hid_tick_and_scan_count_down = timer.count_down();
    hid_tick_and_scan_count_down.start(HID_TICK_AND_MATRIX_SCAN_PERIOD_MS.millis());

    let mut keyboard_count_down = timer.count_down();
    keyboard_count_down.start(KEYBOARD_REPORT_PERIOD_MS.millis());

    let mut consumer_count_down = timer.count_down();
    consumer_count_down.start(CONSUMER_REPORT_PERIOD_MS.millis());

    // Way more than we ever need.
    let mut key_codes_buffer: [Keyboard; 32] = Default::default();
    let mut key_codes_len: usize = 0;
    let mut consumer_codes_buffer: [Consumer; 10] = Default::default();
    let mut consumer_codes_len: usize = 0;

    let mut previous_consumer_report: MultipleConsumerReport = Default::default();

    // Enable the USB interrupt
    unsafe {
        pac::NVIC::unmask(hal::pac::Interrupt::USBCTRL_IRQ);
    };

    loop {
        if hid_tick_and_scan_count_down.wait().is_ok() {
            cortex_m::interrupt::free(|_| {
                let multi = unsafe { MULTI_DEV.as_mut() }.unwrap();
                match multi.tick() {
                    Ok(_) => {}
                    Err(UsbHidError::WouldBlock) => {}
                    Err(UsbHidError::Duplicate) => {}
                    Err(_) => panic!("HID tick failure."),
                }
            });

            (key_codes_len, consumer_codes_len) = scan_keys(
                &mut row_pins,
                &mut column_pins,
                &mut delay,
                &mut debounce_states,
                &mut key_codes_buffer,
                &mut consumer_codes_buffer,
            );

            if key_codes_len == 0 && consumer_codes_len == 0 {
                led_pin.set_low().unwrap()
            } else {
                led_pin.set_high().unwrap()
            }
        }

        if keyboard_count_down.wait().is_ok() {
            cortex_m::interrupt::free(|_| {
                let multi = unsafe { MULTI_DEV.as_mut() }.unwrap();
                let keyboard = multi.device::<NKROBootKeyboard<'_, _>, _>();

                match keyboard.write_report((key_codes_buffer[..key_codes_len]).iter().copied()) {
                    Ok(_) => {}
                    Err(UsbHidError::WouldBlock) => {}
                    Err(UsbHidError::Duplicate) => {}
                    Err(_) => panic!("Keyboard write failure."),
                }
            });
        }

        if consumer_count_down.wait().is_ok() {
            let mut consumer_report = MultipleConsumerReport::default();
            let len = usize::min(consumer_codes_len, consumer_report.codes.len());
            consumer_report.codes[..len]
                .copy_from_slice(&consumer_codes_buffer[..len]);

            cortex_m::interrupt::free(|_| {
                let multi = unsafe { MULTI_DEV.as_mut() }.unwrap();
                let consumer = multi.device::<ConsumerControl<'_, _>, _>();

                match consumer.write_report(&consumer_report) {
                    Ok(_) => {
                        previous_consumer_report = consumer_report;
                    }
                    Err(UsbError::WouldBlock) => {}
                    Err(_) => panic!("Consumer write failure."),
                }
            });
        }

        // Avoid busylooping, we poll the timers at 10KHz.
        delay.delay_us(100);
    }
}

fn scan_keys(
    row_pins: &mut [&mut Pin<DynPinId, FunctionSio<SioOutput>, PullDown>; KEY_ROWS],
    column_pins: &mut [&mut Pin<DynPinId, FunctionSio<SioInput>, PullDown>; KEY_COLUMNS],
    delay: &mut Delay,
    debounce_states: &mut [[DebounceState; KEY_COLUMNS]; KEY_ROWS],
    key_buffer: &mut [Keyboard],
    consumer_buffer: &mut [Consumer],
) -> (usize, usize) {
    let mut key_out_idx: usize = 0;
    let mut consumer_out_idx: usize = 0;

    assert_eq!(KEY_MAPPING.len(), row_pins.len());
    for (row_idx, row_mapping) in KEY_MAPPING.iter().enumerate() {
        row_pins[row_idx].set_high().unwrap();
        delay.delay_us(1);

        assert_eq!(row_mapping.len(), column_pins.len());
        for (col_idx, function) in row_mapping.iter().enumerate() {
            let input = column_pins[col_idx].is_high().unwrap();
            let is_depressed = debounce_states[row_idx][col_idx].update(input);

            match (function, is_depressed) {
                (_, false) => {}
                (KeyFunction::Nothing, _) => {}
                (KeyFunction::Key(Keyboard::NoEventIndicated), _) => {}
                (KeyFunction::Key(Keyboard::Space), true) => {
                                // The space key is a bit special - as there are two on the board.
                                if !key_buffer[..key_out_idx].contains(&Keyboard::Space) {
                                    key_buffer[key_out_idx] = Keyboard::Space;
                                    key_out_idx += 1;
                                }
                            }
                (KeyFunction::Key(key), true) => {
                                key_buffer[key_out_idx] = *key;
                                key_out_idx += 1;
                            }
                (KeyFunction::Media(consumer), true) => {
                                consumer_buffer[consumer_out_idx] = *consumer;
                                consumer_out_idx += 1;
                            }
                (KeyFunction::MultiKey(keys), true) => {
                    let len = keys.len();
                    key_buffer[key_out_idx..][..len].copy_from_slice(keys);
                    key_out_idx += len;
                },
            }
        }
        row_pins[row_idx].set_low().unwrap();
    }

    (key_out_idx, consumer_out_idx)
}

#[allow(non_snake_case)]
#[interrupt]
fn USBCTRL_IRQ() {
    cortex_m::interrupt::free(|_| {
        let usb_dev = unsafe { USB_DEV.as_mut() }.unwrap();
        let multi = unsafe { MULTI_DEV.as_mut() }.unwrap();

        while usb_dev.poll(&mut [multi]) {
            let keyboard = multi.device::<NKROBootKeyboard<'_, _>, _>();
            match keyboard.read_report() {
                Ok(_leds) => {}
                Err(UsbError::WouldBlock) => {}
                Err(_) => panic!("Keyboard read failure."),
            }
        }
    });
}
