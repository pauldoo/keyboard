use crate::{KEY_COLUMNS, KEY_ROWS};

use usbd_human_interface_device::page::Keyboard;
use Keyboard::*;

pub(crate) const KEY_MAPPING: [[Keyboard; KEY_COLUMNS]; KEY_ROWS] = [
    [
        NoEventIndicated, // magic E
        LeftControl,      //
        LeftGUI,          //
        LeftAlt,          //
        NoEventIndicated, // not wired
        NoEventIndicated, // not wired
        Space,            //
        NoEventIndicated, // not wired
        NoEventIndicated, // not wired
        Space,            //
        NoEventIndicated, // not wired
        RightAlt,         //
        RightGUI,         //
        RightControl,     //
        LeftArrow,        //
        DownArrow,        //
        RightArrow,       //
    ],
    [
        NoEventIndicated, // magic D
        LeftShift,        //
        NonUSBackslash,   //
        Z,                //
        X,                //
        C,                //
        V,                //
        B,                //
        N,                //
        M,                //
        Comma,            //
        Dot,              //
        ForwardSlash,     //
        NoEventIndicated, // not wired
        RightShift,       //
        UpArrow,          //
        PageDown,         //
    ],
    [
        NoEventIndicated, // magic C
        CapsLock,         //
        NoEventIndicated, // not wired
        A,                //
        S,                //
        D,                //
        F,                //
        G,                //
        H,                //
        J,                //
        K,                //
        L,                //
        Semicolon,        //
        Apostrophe,       //
        NonUSHash,        //
        NoEventIndicated, // not wired
        PageUp,           //
    ],
    [
        NoEventIndicated, // magic B
        Tab,              //
        NoEventIndicated, // not wired
        Q,                //
        W,                //
        E,                //
        R,                //
        T,                //
        Y,                //
        U,                //
        I,                //
        O,                //
        P,                //
        LeftBrace,        //
        RightBrace,       //
        ReturnEnter,      //
        End,              //
    ],
    [
        Mute,             // Magic a
        Grave,            //
        Keyboard1,        //
        Keyboard2,        //
        Keyboard3,        //
        Keyboard4,        //
        Keyboard5,        //
        Keyboard6,        //
        Keyboard7,        //
        Keyboard8,        //
        Keyboard9,        //
        Keyboard0,        //
        Minus,            //
        Equal,            //
        NoEventIndicated, // not wired
        DeleteBackspace,  //
        Home,             //
    ],
    [
        NoEventIndicated, // not wired
        Escape,           //
        F1,               //
        F2,               //
        F3,               //
        F4,               //
        F5,               //
        F6,               //
        F7,               //
        F8,               //
        F9,               //
        F10,              //
        F11,              //
        F12,              //
        VolumeDown,       //
        VolumeUp,         //
        DeleteForward,    //
    ],
];
