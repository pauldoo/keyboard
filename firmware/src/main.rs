#![no_std]
#![no_main]

use core::cell::RefCell;
use core::panic::PanicInfo;

use cortex_m::delay::Delay;
use cortex_m::interrupt::Mutex;
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

use rp_pico::hal::gpio::DynPinId;
use rp_pico::hal::gpio::FunctionSio;
use rp_pico::hal::gpio::Pin;
use rp_pico::hal::gpio::PullDown;
use rp_pico::hal::gpio::SioInput;
use rp_pico::hal::gpio::SioOutput;
use rp_pico::hal::i2c::I2C;
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
use static_cell::StaticCell;
use usb_device::{class_prelude::*, prelude::*};

use usbd_human_interface_device::device::consumer::ConsumerControl;
use usbd_human_interface_device::device::consumer::ConsumerControlConfig;
use usbd_human_interface_device::device::consumer::MultipleConsumerReport;
use usbd_human_interface_device::device::keyboard::NKROBootKeyboard;
use usbd_human_interface_device::device::keyboard::NKROBootKeyboardConfig;
use usbd_human_interface_device::device::mouse::WheelMouse;
use usbd_human_interface_device::device::mouse::WheelMouseConfig;
use usbd_human_interface_device::device::mouse::WheelMouseReport;
use usbd_human_interface_device::page::Consumer;
use usbd_human_interface_device::page::Keyboard;
use usbd_human_interface_device::prelude::*;

use fugit::RateExtU32;

use embedded_io::Write;

use crate::key_table::MouseButton;
use crate::key_table::MOUSE_MODIFIER_KEYS;

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

/// Distance mouse must move before space keys become mouse buttons
const MOUSENESS_THRESHOLD: u64 = 5;

type UsbMultiDev = UsbHidClass<
    'static,
    hal::usb::UsbBus,
    HList!(
        WheelMouse<'static, hal::usb::UsbBus>,
        ConsumerControl<'static, hal::usb::UsbBus>,
        NKROBootKeyboard<'static, hal::usb::UsbBus>
    ),
>;

static USB_DEV: Mutex<RefCell<Option<UsbDevice<hal::usb::UsbBus>>>> = Mutex::new(RefCell::new(None));

static MULTI_DEV: Mutex<RefCell<Option<UsbMultiDev>>> = Mutex::new(RefCell::new(None));

type LedPin = Pin<hal::gpio::bank0::Gpio25, FunctionSio<SioOutput>, PullDown>;
static LED_PIN: Mutex<RefCell<Option<LedPin>>> = Mutex::new(RefCell::new(None));

#[panic_handler]
fn flash_led(_: &PanicInfo) -> ! {
    let mut on = true;
    loop {
        cortex_m::interrupt::free(|cs| {
            let mut led_pin = LED_PIN.borrow(cs).borrow_mut();
            if let Some(led_pin) = led_pin.as_mut() {
                if on {
                    led_pin.set_high().unwrap();
                } else {
                    led_pin.set_low().unwrap();
                }
            }
        });
        on = !on;
        for _ in 0..10_000_000 {
            core::hint::spin_loop();
        }
    }
}

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
        // Set the LED to be an output
        let led_pin: Pin<hal::gpio::bank0::Gpio25, FunctionSio<SioOutput>, PullDown> = pins.led.into_push_pull_output();
        cortex_m::interrupt::free(|cs| {
            LED_PIN.borrow(cs).replace(Some(led_pin));
        });
    }


    static USB_ALLOC: StaticCell<UsbBusAllocator<hal::usb::UsbBus>> = StaticCell::new();
    let usb_alloc = USB_ALLOC.init(UsbBusAllocator::new(hal::usb::UsbBus::new(
                pac.USBCTRL_REGS,
                pac.USBCTRL_DPRAM,
                clocks.usb_clock,
                true,
                &mut pac.RESETS,
            )));

    {
        let multi = UsbHidClassBuilder::new()
            .add_device(NKROBootKeyboardConfig::default())
            .add_device(ConsumerControlConfig::default())
            .add_device(WheelMouseConfig::default())
            .build(usb_alloc);

        cortex_m::interrupt::free(|cs| {
            MULTI_DEV.borrow(cs).replace(Some(multi));
        });
    }

    {
        let usb_dev: UsbDevice<hal::usb::UsbBus> =
            UsbDeviceBuilder::new(usb_alloc, UsbVidPid(0x1209, 0x0001))
                .strings(&[StringDescriptors::default().product("Crappy Keyboard")])
                .unwrap()
                .build();

        cortex_m::interrupt::free(|cs| {
            USB_DEV.borrow(cs).replace(Some(usb_dev));
        });
    }

    let sda_pin: hal::gpio::Pin<_, hal::gpio::FunctionI2C, _> = pins.gpio26.reconfigure();
    let scl_pin: hal::gpio::Pin<_, hal::gpio::FunctionI2C, _> = pins.gpio27.reconfigure();


    //let sda_pin = pins.gpio26.into_function::<FunctionI2C>();
    //let scl_pin = pins.gpio27.into_function::<FunctionI2C>();
    // Create I2C peripheral - using I2C1 for GP26/GP27
    let mut i2c = I2C::i2c1(
        pac.I2C1,
        sda_pin,
        scl_pin,
        100.kHz(), // I2C clock frequency
        &mut pac.RESETS,
        &clocks.peripheral_clock,
    );

    // The delay object lets us wait for specified amounts of time (in
    // milliseconds)
    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

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

    let mut debounce_states: [[DebounceState; KEY_COLUMNS]; KEY_ROWS] = Default::default();

    let mut hid_tick_and_scan_count_down = timer.count_down();
    hid_tick_and_scan_count_down.start(HID_TICK_AND_MATRIX_SCAN_PERIOD_MS.millis());

    let mut keyboard_count_down = timer.count_down();
    keyboard_count_down.start(KEYBOARD_REPORT_PERIOD_MS.millis());

    let mut consumer_count_down = timer.count_down();
    consumer_count_down.start(CONSUMER_REPORT_PERIOD_MS.millis());


    let mut previous_consumer_report: MultipleConsumerReport = Default::default();

    // Enable the USB interrupt
    unsafe {
        pac::NVIC::unmask(hal::pac::Interrupt::USBCTRL_IRQ);
    };

    let mut buffers: ScanBuffers = Default::default();
    // Distance mouse has moved without a normal key being pressed.
    let mut mouseness: u64 = 0;
    let mut press_counter: u64 = 0;
    let mut mouse_tracker: MouseTracker = Default::default();
    let mut scan_clock: u64 = 0;

    //i2c.write(0x08u8, b"binky");

    loop {
        if hid_tick_and_scan_count_down.wait().is_ok() {
            cortex_m::interrupt::free(|cs| {
                let mut x = MULTI_DEV.borrow(cs).borrow_mut();
                if let Some(multi) = x.as_mut() {
                    match multi.tick() {
                        Ok(_) => {}
                        Err(UsbHidError::WouldBlock) => {}
                        Err(UsbHidError::Duplicate) => {}
                        Err(_) => panic!("HID tick failure."),
                    }
                }                
            });

            scan_clock += 1;
            let press_counter_previous = press_counter;
            scan_keys(
                &mut row_pins,
                &mut column_pins,
                &mut delay,
                &mut debounce_states,
                &mut buffers,
                mouseness >= MOUSENESS_THRESHOLD,
                scan_clock,
                || press_counter += 1
            );

            if press_counter != press_counter_previous && true {
                let mut bytes = [0u8; 12];
                let _ = write!(bytes.as_mut_slice(), "{}", press_counter);
                let len = bytes.iter().take_while(|n| **n != 0u8).count();
                let _ = i2c.write(0x08u8, &bytes[..len]);
            }
        }

        if keyboard_count_down.wait().is_ok() {
            if (scan_clock * u64::from(HID_TICK_AND_MATRIX_SCAN_PERIOD_MS)) >= 500 {
                // Update the mouse only if we have been running for more that 0.5 seconds.
                // The joystick origin is garbage shortly after boot.
                mouse_tracker.update(&mut i2c);
            }
            let mut mouse_report = WheelMouseReport::default();
            mouse_tracker.populate_report(&mut mouse_report);
            buffers.mouse_buttons.iter().for_each(|b|{
                match b {
                    MouseButton::Left => mouse_report.buttons |= 0x1,
                    MouseButton::Right => mouse_report.buttons |= 0x2
                }
            });

            mouseness += u64::try_from(mouse_report.x.abs()).unwrap();
            mouseness += u64::try_from(mouse_report.y.abs()).unwrap();

            if mouseness != 0 {
                if buffers.consumer_codes.is_empty() && buffers.key_codes.iter().all(|k| MOUSE_MODIFIER_KEYS.contains(k)) {
                    // No consumer keys pressed, all keyboard keys are modifiers.
                    // stay in mouse mode
                } else {
                    mouseness = 0;
                }
            }

            cortex_m::interrupt::free(|cs| {
                if let Some(led_pin) = LED_PIN.borrow(cs).borrow_mut().as_mut() {
                    if mouseness >= MOUSENESS_THRESHOLD {
                        led_pin.set_high().unwrap();
                    } else {
                        led_pin.set_low().unwrap();
                    }
                }

                let mut x = MULTI_DEV.borrow(cs).borrow_mut();
                if let Some(multi) = x.as_mut() {

                    let keyboard = multi.device::<NKROBootKeyboard<'_, _>, _>();

                    match keyboard.write_report(buffers.key_codes.iter().copied()) {
                        Ok(_) => {}
                        Err(UsbHidError::WouldBlock) => {}
                        Err(UsbHidError::Duplicate) => {}
                        Err(_) => panic!("Keyboard write failure."),
                    }

                    let mouse = multi.device::<WheelMouse<'_, _>, _>();

                    match mouse.write_report(&mouse_report) {
                        Ok(_) => {
                            mouse_tracker.account_report(&mouse_report);
                        }
                        Err(UsbHidError::WouldBlock) => {}
                        Err(UsbHidError::Duplicate) => {}
                        Err(_) => panic!("Mouse write failure."),
                    }
                }
            });
        }

        if consumer_count_down.wait().is_ok() {
            let mut consumer_report = MultipleConsumerReport::default();
            let len = usize::min(buffers.consumer_codes.len(), consumer_report.codes.len());
            consumer_report.codes[..len]
                .copy_from_slice(&buffers.consumer_codes.as_slice()[..len]);

            cortex_m::interrupt::free(|cs| {
                let mut x = MULTI_DEV.borrow(cs).borrow_mut();
                if let Some(multi) = x.as_mut() {

                    let consumer = multi.device::<ConsumerControl<'_, _>, _>();

                    match consumer.write_report(&consumer_report) {
                        Ok(_) => {
                            previous_consumer_report = consumer_report;
                        }
                        Err(UsbError::WouldBlock) => {}
                        Err(_) => panic!("Consumer write failure."),
                    }
                }

            });
        }

        // Avoid busylooping, we poll the timers at 10KHz.
        delay.delay_us(100);
    }
}

// The result of scanning which keys are pressed.
#[derive(Default)]
struct ScanBuffers {
    // Way more than we ever need.
    key_codes: heapless::Vec<Keyboard, 32>,
    consumer_codes: heapless::Vec<Consumer, 10>,
    mouse_buttons: heapless::Vec<MouseButton, 2>,
}

impl ScanBuffers {
    fn clear(&mut self) {
        self.key_codes.clear();
        self.consumer_codes.clear();
        self.mouse_buttons.clear();
    }
}

#[derive(Default, Clone)]
struct Point2D<T> {
    x: T,
    y: T
}

type ButtonState = bool;

#[derive(Default)]
struct MouseTracker {
    unreported_movement: Point2D<i64>,
    button: ButtonState,
    origin: Option<Point2D<i16>>
}

fn mouse_curve(d: i16) -> i32 {
    if false {
        let d = i32::from(d);
        d*d*d.signum()
    } else {
        i32::from(d) * 500
    }
}

/// Bigger means slower cursor.
static MOUSE_REPORT_SCALE: i64 = 40_000;

impl MouseTracker {
    // Read mouse position from i2c, updating internal measurement of movement.
    fn update(&mut self, i2c: &mut impl embedded_hal::i2c::I2c) {
        if let Some(raw) = read_mouse_raw(i2c) {
            self.button = raw.1;
            if self.origin.is_none() {
                self.origin = Some(raw.0.clone());
            }

            let raw = raw.0;
            let origin = self.origin.as_ref().unwrap();
            self.unreported_movement.x += i64::from(mouse_curve(raw.x - origin.x));
            self.unreported_movement.y += i64::from(mouse_curve(raw.y - origin.y));
        }
    }

    fn populate_report(&self, report: &mut WheelMouseReport) {
        let rx = (self.unreported_movement.x / MOUSE_REPORT_SCALE).clamp(i8::MIN.into(), i8::MAX.into());
        let ry = (self.unreported_movement.y / MOUSE_REPORT_SCALE).clamp(i8::MIN.into(), i8::MAX.into());

        report.x = i8::try_from(rx).unwrap();
        report.y = i8::try_from(ry).unwrap();
        report.buttons = if self.button { 0x1 } else { 0x0 };
    }

    fn account_report(&mut self, report: &WheelMouseReport) {
        self.unreported_movement.x -= i64::from(report.x) * MOUSE_REPORT_SCALE;
        self.unreported_movement.y -= i64::from(report.y) * MOUSE_REPORT_SCALE;
    }
}

// The raw reading from the Arduino about joystick position and button state.
fn read_mouse_raw(i2c: &mut impl embedded_hal::i2c::I2c) -> Option<(Point2D<i16>, ButtonState)> {
    let mut mouse_buffer: [u8; 5] = [0u8; 5];

    match i2c.read(0x08u8, mouse_buffer.as_mut_slice()) {
        Ok(_) => {
            Some((
                Point2D::<i16>{
                  x: -i16::try_from(u16::from_be_bytes(mouse_buffer[2..4].try_into().unwrap())).unwrap(),
                  y: i16::try_from(u16::from_be_bytes(mouse_buffer[0..2].try_into().unwrap())).unwrap()
                },
                mouse_buffer[4] == 1u8
            ))
        },
        Err(_) => None,
    }
}

fn scan_keys<F: FnMut()>(
    row_pins: &mut [&mut Pin<DynPinId, FunctionSio<SioOutput>, PullDown>; KEY_ROWS],
    column_pins: &mut [&mut Pin<DynPinId, FunctionSio<SioInput>, PullDown>; KEY_COLUMNS],
    delay: &mut Delay,
    debounce_states: &mut [[DebounceState; KEY_COLUMNS]; KEY_ROWS],
    buffers: &mut ScanBuffers,
    mouseish: bool,
    scan_clock: u64,
    mut press_action: F
) -> () {
    buffers.clear();

    assert_eq!(KEY_MAPPING.len(), row_pins.len());
    for (row_idx, row_mapping) in KEY_MAPPING.iter().enumerate() {
        row_pins[row_idx].set_high().unwrap();
        delay.delay_us(1);

        assert_eq!(row_mapping.len(), column_pins.len());
        for (col_idx, function) in row_mapping.iter().enumerate() {
            let input = column_pins[col_idx].is_high().unwrap();
            let is_depressed = debounce_states[row_idx][col_idx].update(input, scan_clock, &mut press_action);

            match (function, is_depressed) {
                (_, false) => {}
                (KeyFunction::Nothing, _) => {}
                (KeyFunction::Key(Keyboard::NoEventIndicated), _) => {}
                (KeyFunction::Key(key), true) => {
                                if !buffers.key_codes.contains(key) {
                                    buffers.key_codes.push(*key).unwrap();
                                }
                            }
                (KeyFunction::Media(consumer), true) => {
                                buffers.consumer_codes.push(*consumer).unwrap();
                            }
                (KeyFunction::MultiKey(keys), true) => {
                    keys.iter().for_each(|k| {
                        if !buffers.key_codes.contains(k) {
                            buffers.key_codes.push(*k).unwrap();
                        }
                    });
                },
                (KeyFunction::Dual(key, mouse_button), true) => {
                    if mouseish {
                        buffers.mouse_buttons.push(*mouse_button).unwrap();
                    } else {
                        if !buffers.key_codes.contains(key) {
                            buffers.key_codes.push(*key).unwrap();
                        }
                    }
                }
            }
        }
        row_pins[row_idx].set_low().unwrap();
    }
}

#[allow(non_snake_case)]
#[interrupt]
fn USBCTRL_IRQ() {
    cortex_m::interrupt::free(|cs| {
        let mut usb_dev = USB_DEV.borrow(cs).borrow_mut();
        let mut multi = MULTI_DEV.borrow(cs).borrow_mut();
        if let Some(usb_dev) = usb_dev.as_mut() &&
            let Some(multi) = multi.as_mut() {

            while usb_dev.poll(&mut [multi]) {
                let keyboard = multi.device::<NKROBootKeyboard<'_, _>, _>();
                match keyboard.read_report() {
                    Ok(_leds) => {}
                    Err(UsbError::WouldBlock) => {}
                    Err(_) => panic!("Keyboard read failure."),
                }
            }
        }
    });
}
