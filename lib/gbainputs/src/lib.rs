#![no_std]

use voladdress::{ Safe, VolAddress };

const KEY_REGISTER: VolAddress<u16, Safe, ()> = unsafe { VolAddress::new(0x0400_0130) };
const KEY_MASK: u16 = 0b11_1111_1111u16;

#[derive(PartialEq, Eq, Clone, Copy)]
#[repr(u16)]
pub enum Key {
    A      = 0b00_0000_0001u16,
    B      = 0b00_0000_0010u16,
    SELECT = 0b00_0000_0100u16,
    START  = 0b00_0000_1000u16,
    RIGHT  = 0b00_0001_0000u16,
    LEFT   = 0b00_0010_0000u16,
    UP     = 0b00_0100_0000u16,
    DOWN   = 0b00_1000_0000u16,
    R      = 0b01_0000_0000u16,
    L      = 0b10_0000_0000u16,
}

pub struct KeyMonitor {
    previous: u16,
    current: u16,
}

impl KeyMonitor {
    pub fn new() -> KeyMonitor {
        KeyMonitor {
            previous: 0,
            current: 0,
        }
    }

    #[inline]
    pub fn is_pressed(&self, key: Key) -> bool {
        self.current & (key as u16) != 0
    }

    #[inline]
    pub fn was_pressed(&self, key: Key) -> bool {
        self.previous & (key as u16) != 0
    }

    #[inline]
    pub fn is_released(&self, key: Key) -> bool {
        self.current & (key as u16) == 0
    }

    #[inline]
    pub fn was_released(&self, key: Key) -> bool {
        self.previous & (key as u16) == 0
    }

    #[inline]
    pub fn just_pressed(&self, key: Key) -> bool {
        self.was_released(key) && self.is_pressed(key)
    }

    #[inline]
    pub fn just_released(&self, key: Key) -> bool {
        self.was_pressed(key) && self.is_released(key)
    }

    #[inline]
    pub fn held(&self, key: Key) -> bool {
        self.was_pressed(key) && self.is_pressed(key)
    }

    #[inline]
    pub fn update(&mut self) {
        self.previous = self.current;
        self.current = KEY_REGISTER.read() ^ KEY_MASK;
    }
}