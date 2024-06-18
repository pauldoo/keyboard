use crate::{KEY_COLUMNS, KEY_ROWS};

use usbd_human_interface_device::page::{Consumer, Keyboard};
use Keyboard::*;

use KeyFunction::*;

pub(crate) enum KeyFunction {
    Nothing,
    Key(Keyboard),
    Media(Consumer),
}

pub(crate) const KEY_MAPPING: [[KeyFunction; KEY_COLUMNS]; KEY_ROWS] = [
    [
        Nothing,           // magic E
        Key(LeftControl),  //
        Key(LeftGUI),      //
        Key(LeftAlt),      //
        Nothing,           // not wired
        Nothing,           // not wired
        Key(Space),        //
        Nothing,           // not wired
        Nothing,           // not wired
        Key(Space),        //
        Nothing,           // not wired
        Key(RightAlt),     //
        Key(RightGUI),     //
        Key(RightControl), //
        Key(LeftArrow),    //
        Key(DownArrow),    //
        Key(RightArrow),   //
    ],
    [
        Media(Consumer::ALTextEditor),             // magic D
        Key(LeftShift),      //
        Key(NonUSBackslash), //
        Key(Z),              //
        Key(X),              //
        Key(C),              //
        Key(V),              //
        Key(B),              //
        Key(N),              //
        Key(M),              //
        Key(Comma),          //
        Key(Dot),            //
        Key(ForwardSlash),   //
        Nothing,             // not wired
        Key(RightShift),     //
        Key(UpArrow),        //
        Key(PageDown),       //
    ],
    [
        Media(Consumer::ALCalculator),         // magic C
        Key(CapsLock),   //
        Nothing,         // not wired
        Key(A),          //
        Key(S),          //
        Key(D),          //
        Key(F),          //
        Key(G),          //
        Key(H),          //
        Key(J),          //
        Key(K),          //
        Key(L),          //
        Key(Semicolon),  //
        Key(Apostrophe), //
        Key(NonUSHash),  //
        Nothing,         // not wired
        Key(PageUp),     //
    ],
    [
        Media(Consumer::PlayPause), // magic B
        Key(Tab),                   //
        Nothing,                    // not wired
        Key(Q),                     //
        Key(W),                     //
        Key(E),                     //
        Key(R),                     //
        Key(T),                     //
        Key(Y),                     //
        Key(U),                     //
        Key(I),                     //
        Key(O),                     //
        Key(P),                     //
        Key(LeftBrace),             //
        Key(RightBrace),            //
        Key(ReturnEnter),           //
        Key(End),                   //
    ],
    [
        Media(Consumer::Mute), // Magic a
        Key(Grave),            //
        Key(Keyboard1),        //
        Key(Keyboard2),        //
        Key(Keyboard3),        //
        Key(Keyboard4),        //
        Key(Keyboard5),        //
        Key(Keyboard6),        //
        Key(Keyboard7),        //
        Key(Keyboard8),        //
        Key(Keyboard9),        //
        Key(Keyboard0),        //
        Key(Minus),            //
        Key(Equal),            //
        Nothing,               // not wired
        Key(DeleteBackspace),  //
        Key(Home),             //
    ],
    [
        Nothing,                          // not wired
        Key(Escape),                      //
        Key(F1),                          //
        Key(F2),                          //
        Key(F3),                          //
        Key(F4),                          //
        Key(F5),                          //
        Key(F6),                          //
        Key(F7),                          //
        Key(F8),                          //
        Key(F9),                          //
        Key(F10),                         //
        Key(F11),                         //
        Key(F12),                         //
        Media(Consumer::VolumeDecrement), //
        Media(Consumer::VolumeIncrement), //
        Key(DeleteForward),               //
    ],
];
