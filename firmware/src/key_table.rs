use crate::{KeyAction, KEY_COLUMNS, KEY_ROWS};

use usbd_hid::descriptor::KeyboardUsage;
use KeyAction::{Letter, Nothing};

pub(crate) const KEY_MAPPING: [[KeyAction; KEY_COLUMNS]; KEY_ROWS] = [
    [
        Letter(KeyboardUsage::KeyboardAa), //
        Letter(KeyboardUsage::KeyboardBb), //
        Letter(KeyboardUsage::KeyboardCc), //
        Nothing,                           //
        Nothing,                           //
        Nothing,                           //
        Nothing,                           //
        Nothing,                           //
        Nothing,                           //
        Nothing,                           //
        Nothing,                           //
        Nothing,                           //
        Nothing,                           //
        Nothing,                           //
        Nothing,                           //
        Nothing,                           //
        Nothing,
    ],
    [
        Nothing, Nothing, Nothing, Nothing, Nothing, Nothing, Nothing, Nothing, Nothing, Nothing,
        Nothing, Nothing, Nothing, Nothing, Nothing, Nothing, Nothing,
    ],
    [
        Nothing, Nothing, Nothing, Nothing, Nothing, Nothing, Nothing, Nothing, Nothing, Nothing,
        Nothing, Nothing, Nothing, Nothing, Nothing, Nothing, Nothing,
    ],
    [
        Nothing, Nothing, Nothing, Nothing, Nothing, Nothing, Nothing, Nothing, Nothing, Nothing,
        Nothing, Nothing, Nothing, Nothing, Nothing, Nothing, Nothing,
    ],
    [
        Nothing, Nothing, Nothing, Nothing, Nothing, Nothing, Nothing, Nothing, Nothing, Nothing,
        Nothing, Nothing, Nothing, Nothing, Nothing, Nothing, Nothing,
    ],
    [
        Nothing, Nothing, Nothing, Nothing, Nothing, Nothing, Nothing, Nothing, Nothing, Nothing,
        Nothing, Nothing, Nothing, Nothing, Nothing, Nothing, Nothing,
    ],
];
