use crate::{KEY_COLUMNS, KEY_ROWS};

use usbd_human_interface_device::page::Keyboard;
use Keyboard::NoEventIndicated;

pub(crate) const KEY_MAPPING: [[Keyboard; KEY_COLUMNS]; KEY_ROWS] = [
    [
        Keyboard::A, //
        Keyboard::B, //
        Keyboard::C, //
        NoEventIndicated,              //
        NoEventIndicated,              //
        NoEventIndicated,              //
        NoEventIndicated,              //
        NoEventIndicated,              //
        NoEventIndicated,              //
        NoEventIndicated,              //
        NoEventIndicated,              //
        NoEventIndicated,              //
        NoEventIndicated,              //
        NoEventIndicated,              //
        NoEventIndicated,              //
        NoEventIndicated,              //
        NoEventIndicated,
    ],
    [
        NoEventIndicated, NoEventIndicated, NoEventIndicated, NoEventIndicated, NoEventIndicated, NoEventIndicated, NoEventIndicated, NoEventIndicated, NoEventIndicated, NoEventIndicated, NoEventIndicated, NoEventIndicated, NoEventIndicated, NoEventIndicated, NoEventIndicated,
        NoEventIndicated, NoEventIndicated,
    ],
    [
        NoEventIndicated, NoEventIndicated, NoEventIndicated, NoEventIndicated, NoEventIndicated, NoEventIndicated, NoEventIndicated, NoEventIndicated, NoEventIndicated, NoEventIndicated, NoEventIndicated, NoEventIndicated, NoEventIndicated, NoEventIndicated, NoEventIndicated,
        NoEventIndicated, NoEventIndicated,
    ],
    [
        NoEventIndicated, NoEventIndicated, NoEventIndicated, NoEventIndicated, NoEventIndicated, NoEventIndicated, NoEventIndicated, NoEventIndicated, NoEventIndicated, NoEventIndicated, NoEventIndicated, NoEventIndicated, NoEventIndicated, NoEventIndicated, NoEventIndicated,
        NoEventIndicated, NoEventIndicated,
    ],
    [
        NoEventIndicated, NoEventIndicated, NoEventIndicated, NoEventIndicated, NoEventIndicated, NoEventIndicated, NoEventIndicated, NoEventIndicated, NoEventIndicated, NoEventIndicated, NoEventIndicated, NoEventIndicated, NoEventIndicated, NoEventIndicated, NoEventIndicated,
        NoEventIndicated, NoEventIndicated,
    ],
    [
        NoEventIndicated, NoEventIndicated, NoEventIndicated, NoEventIndicated, NoEventIndicated, NoEventIndicated, NoEventIndicated, NoEventIndicated, NoEventIndicated, NoEventIndicated, NoEventIndicated, NoEventIndicated, NoEventIndicated, NoEventIndicated, NoEventIndicated,
        NoEventIndicated, NoEventIndicated,
    ],
];
