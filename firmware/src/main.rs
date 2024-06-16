#![no_std]
#![no_main]

use cortex_m::delay::Delay;
use debounce::DebounceState;
use embedded_hal::digital::InputPin;
use key_table::KEY_MAPPING;
// The macro for our start-up function
use rp_pico::entry;

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

// USB Human Interface Device (HID) Class support
use usbd_hid::descriptor::generator_prelude::*;
use usbd_hid::descriptor::KeyboardReport;
use usbd_hid::descriptor::KeyboardUsage;
use usbd_hid::hid_class::HIDClass;

// The macro for marking our interrupt functions
use rp_pico::hal::pac::interrupt;
use usbd_hid::hid_class::HidCountryCode;
use usbd_hid::hid_class::HidProtocol;
use usbd_hid::hid_class::HidSubClass;
use usbd_hid::hid_class::ProtocolModeConfig;

mod debounce;
mod key_table;

enum KeyAction {
    Nothing,
    Letter(KeyboardUsage),
}

pub(crate) const KEY_ROWS: usize = 6;
pub(crate) const KEY_COLUMNS: usize = 17;

/// The USB Bus Driver (shared with the interrupt).
static mut USB_BUS: Option<UsbBusAllocator<hal::usb::UsbBus>> = None;

/// The USB Device Driver (shared with the interrupt).
static mut USB_DEVICE: Option<UsbDevice<hal::usb::UsbBus>> = None;

/// The USB Human Interface Device Driver (shared with the interrupt).
static mut USB_HID: Option<HIDClass<hal::usb::UsbBus>> = None;

const POLL_PERIOD_MS: u8 = 5;

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

    // The single-cycle I/O block controls our GPIO pins
    let sio = hal::Sio::new(pac.SIO);

    // Set the pins up according to their function on this particular board
    let pins = rp_pico::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );
    let _timer = Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);
    
    {
        // Set up the USB driver
        let usb_bus = UsbBusAllocator::new(hal::usb::UsbBus::new(
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
        let bus_ref = unsafe { USB_BUS.as_ref().unwrap() };
        // Set up the USB HID Class Device driver, providing Keyboard Reports
        let usb_hid = HIDClass::new_with_settings(
            bus_ref,
            KeyboardReport::desc(),
            POLL_PERIOD_MS,
            usbd_hid::hid_class::HidClassSettings {
                subclass: HidSubClass::NoSubClass,
                protocol: HidProtocol::Keyboard,
                config: ProtocolModeConfig::ForceReport,
                locale: HidCountryCode::UK,
            },
        );
        unsafe { USB_HID = Some(usb_hid) }
    }
    {
        let bus_ref = unsafe { USB_BUS.as_ref().unwrap() };
        // Create a USB device with a fake VID and PID
        let usb_dev = UsbDeviceBuilder::new(bus_ref, UsbVidPid(0x16c0, 0x27da))
            .strings(&[StringDescriptors::default()
                .manufacturer("Paul")
                .product("Crappy")
                .serial_number("123")])
            .unwrap()
            .device_class(0)
            .build();
        unsafe {
            // Note (safety): This is safe as interrupts haven't been started yet
            USB_DEVICE = Some(usb_dev);
        }
    }

    // The delay object lets us wait for specified amounts of time (in
    // milliseconds)
    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());


    // Set the LED to be an output
    let mut led_pin = pins.led.into_push_pull_output();

    let row_pins: [&mut Pin<DynPinId, FunctionSio<SioOutput>, PullDown>; KEY_ROWS] = [
        &mut pins.gpio17.into_push_pull_output().into_dyn_pin(),
        &mut pins.gpio18.into_push_pull_output().into_dyn_pin(),
        &mut pins.gpio19.into_push_pull_output().into_dyn_pin(),
        &mut pins.gpio20.into_push_pull_output().into_dyn_pin(),
        &mut pins.gpio21.into_push_pull_output().into_dyn_pin(),
        &mut pins.gpio22.into_push_pull_output().into_dyn_pin(),
    ];

    let column_pins: [&mut Pin<DynPinId, FunctionSio<SioInput>, PullDown>; KEY_COLUMNS] = [
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

    if true {
        unsafe {
            // Enable the USB interrupt
            pac::NVIC::unmask(hal::pac::Interrupt::USBCTRL_IRQ);
        };
    }

    led_pin.set_low().unwrap();
    delay.delay_ms(3_000);

    release_all_keys();

    let mut debounce_states = [[DebounceState::default(); KEY_COLUMNS]; KEY_ROWS];

    let mut previous_report = KeyboardReport::default();

    let mut cycle_count = 0u64;

    loop {
        if false {
            if (cycle_count / 100) % 2 == 0 {
                led_pin.set_high().unwrap();
            } else {
                led_pin.set_low().unwrap();
            }
        }
        cycle_count += 1;

        let mut report: KeyboardReport = KeyboardReport::default();
        let mut something_pressed = false;

        for row_idx in 0..KEY_ROWS {
            row_pins[row_idx].set_high().unwrap();
            delay.delay_us(1);
            for col_idx in 0..KEY_COLUMNS {
                let input = column_pins[col_idx].is_high().unwrap();

                let is_depressed = debounce_states[row_idx][col_idx].update(input);
                if is_depressed {
                    something_pressed = true;
                    enact_action(&KEY_MAPPING[row_idx][col_idx], &mut report);
                }
            }
            row_pins[row_idx].set_low().unwrap();
        }

        if something_pressed {
            led_pin.set_high().unwrap();
        } else {
            led_pin.set_low().unwrap();
        }
        if !report_eq(&previous_report, &report) {
            send_report(&report);
            previous_report = report;
        }

        if false {
            critical_section::with(|_| unsafe {
                let usb_hid = USB_HID.as_mut().unwrap();
            
                let mut buf = [0; 64];
                usb_hid.pull_raw_report(&mut buf);
            });
        }

        delay.delay_ms(POLL_PERIOD_MS.into());
    }
}

fn enact_action(key: &KeyAction, report: &mut KeyboardReport) {
    match key {
        KeyAction::Nothing => {}
        KeyAction::Letter(usage) => {
            for i in 0..report.keycodes.len() {
                if report.keycodes[i] == 0 {
                    report.keycodes[i] = *usage as u8;
                    break;
                }
            }
        }
    }
}

fn report_eq(left: &KeyboardReport, right: &KeyboardReport) -> bool {
    left.modifier == right.modifier
        && left.reserved == right.reserved
        && left.leds == right.leds
        && left.keycodes == right.keycodes
}

fn release_all_keys() {
    send_report(&KeyboardReport::default());
}

fn send_report(report: &KeyboardReport) -> () {
    let _ = critical_section::with(|_| unsafe {
        USB_HID.as_mut().map(|hid| hid.push_input(report))
    })
    .unwrap();
}


/// This function is called whenever the USB Hardware generates an Interrupt
/// Request.
#[allow(non_snake_case)]
#[interrupt]
unsafe fn USBCTRL_IRQ() {
    // Handle USB request
    let usb_dev = USB_DEVICE.as_mut().unwrap();
    let usb_hid = USB_HID.as_mut().unwrap();
    usb_dev.poll(&mut [usb_hid]);
}
