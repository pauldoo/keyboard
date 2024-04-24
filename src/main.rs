#![no_std]
#![no_main]

use cortex_m::delay::Delay;
// The macro for our start-up function
use rp_pico::entry;

// GPIO traits
use embedded_hal::digital::OutputPin;

// Ensure we halt the program on panic (if we don't mention this crate it won't
// be linked)
use panic_halt as _;

// Pull in any important traits
use rp_pico::hal::prelude::*;

// A shorter alias for the Peripheral Access Crate, which provides low-level
// register access
use rp_pico::hal::pac;

// A shorter alias for the Hardware Abstraction Layer, which provides
// higher-level drivers.
use rp_pico::hal;

use usb_device::{class_prelude::*, prelude::*};

// USB Human Interface Device (HID) Class support
use usbd_hid::descriptor::generator_prelude::*;
use usbd_hid::descriptor::KeyboardReport;
use usbd_hid::descriptor::KeyboardUsage;
use usbd_hid::descriptor::MouseReport;
use usbd_hid::hid_class::HIDClass;

// The macro for marking our interrupt functions
use rp_pico::hal::pac::interrupt;

/// The USB Bus Driver (shared with the interrupt).
static mut USB_BUS: Option<UsbBusAllocator<hal::usb::UsbBus>> = None;

/// The USB Device Driver (shared with the interrupt).
static mut USB_DEVICE: Option<UsbDevice<hal::usb::UsbBus>> = None;

/// The USB Human Interface Device Driver (shared with the interrupt).
static mut USB_HID: Option<HIDClass<hal::usb::UsbBus>> = None;


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
        // Set up the USB HID Class Device driver, providing Mouse Reports
        let usb_hid = HIDClass::new(bus_ref, KeyboardReport::desc(), 20);
        unsafe {
            USB_HID = Some(usb_hid)
        }
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

    // The single-cycle I/O block controls our GPIO pins
    let sio = hal::Sio::new(pac.SIO);

    // Set the pins up according to their function on this particular board
    let pins = rp_pico::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // Set the LED to be an output
    let mut led_pin = pins.led.into_push_pull_output();

    unsafe {
        // Enable the USB interrupt
        pac::NVIC::unmask(hal::pac::Interrupt::USBCTRL_IRQ);
    };


    // Blink for a bit
    loop {
        led_pin.set_high().unwrap();
        send_key(KeyboardUsage::KeyboardPp, &mut delay);
        delay.delay_ms(500);

        led_pin.set_low().unwrap();
        send_key(KeyboardUsage::KeyboardQq, &mut delay);
        delay.delay_ms(500);
    }
}

fn send_key(key: KeyboardUsage, delay: &mut Delay) {
    // Press
    let mut report: KeyboardReport = KeyboardReport::default();
    report.keycodes[0] = key as u8;
    send_report(&report);
    
    // Wait
    delay.delay_ms(50);

    // Release
    send_report(&KeyboardReport::default());
}

fn send_report(report: &KeyboardReport) {
    critical_section::with(|_| unsafe {
        USB_HID.as_mut().map(|hid| 
            hid.push_input(
            report
        ))
    }).unwrap();
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
