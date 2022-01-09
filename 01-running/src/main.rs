#![no_std]
#![no_main]

use gba::prelude::*;
use gbainputs::KeyMonitor;
use gbamath::fixed::{ UFixed8, SFixed8 };
use gbamath::geometry::BoundingBox;
use gbamath::Vec2D;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

struct Player {
    old_position: Vec2D<UFixed8>,
    current_position: Vec2D<UFixed8>,
    old_velocity: Vec2D<SFixed8>,
    current_velocity: Vec2D<SFixed8>,
    collision_box: BoundingBox,
    box_offset: Vec2D<SFixed8>,
    collision_state: u8,
    inputs: KeyMonitor,
}

impl Player {
    // Collision State Masks
    const TOP_COLLISION_LAST_FRAME: u8    = 0b1000_0000;
    const TOP_COLLISION_THIS_FRAME: u8    = 0b0100_0000;
    const BOTTOM_COLLISION_LAST_FRAME: u8 = 0b0010_0000;
    const BOTTOM_COLLISION_THIS_FRAME: u8 = 0b0001_0000;
    const LEFT_COLLISION_LAST_FRAME: u8   = 0b0000_1000;
    const LEFT_COLLISION_THIS_FRAME: u8   = 0b0000_0100;
    const RIGHT_COLLISION_LAST_FRAME: u8  = 0b0000_0010;
    const RIGHT_COLLISION_THIS_FRAME: u8  = 0b0000_0001;
    const LAST_FRAME_STATES: u8 =
        Player::TOP_COLLISION_LAST_FRAME |
        Player::BOTTOM_COLLISION_LAST_FRAME |
        Player::LEFT_COLLISION_LAST_FRAME |
        Player::RIGHT_COLLISION_LAST_FRAME;
    
    fn update(&mut self) {
        self.old_position = self.current_position;
        self.old_velocity = self.current_velocity;
        self.collision_state = (self.collision_state << 1) & Player::LAST_FRAME_STATES;
        self.current_position.saturating_add_signed_assign(self.current_velocity);
        self.collision_box.center = self.current_position.saturating_add_signed(self.box_offset);

        self.inputs.update();
    }
}

#[no_mangle]
fn main() -> ! {
    loop {}
}
