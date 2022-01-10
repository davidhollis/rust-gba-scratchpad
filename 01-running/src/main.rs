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

enum PlayerState {
    Standing,
    Walking,
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
    const COLOR: Color = Color::from_rgb(0, 0, 31);

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

    fn draw(&self) {
        let left: u16 = self.collision_box.left().into();
        let right: u16 = self.collision_box.right().into();
        let top: u16 = self.collision_box.top().into();
        let bottom: u16 = self.collision_box.bottom().into();
        for x in left..=right {
            for y in top..=bottom {
                mode3::bitmap_xy(x.into(), y.into()).write(Player::COLOR);
            }
        }
    }
}

const BACKGROUND_COLOR: Color = Color::from_rgb(31, 31, 31);

#[no_mangle]
fn main() -> ! {
    let mut player: Player = Player {
        old_position: Vec2D { x: UFixed8::ZERO, y: 160u16.into() },
        current_position: Vec2D { x: UFixed8::ZERO, y: UFixed8::ZERO },
        old_velocity: Vec2D { x: SFixed8::ZERO, y: SFixed8::ZERO },
        current_velocity: Vec2D { x: SFixed8::ZERO, y: SFixed8::ZERO },
        collision_box: BoundingBox {
            center: Vec2D { x: 8u16.into(), y: 144u16.into() },
            half_size: Vec2D { x: 8u16.into(), y: 16u16.into() },
        },
        box_offset: Vec2D { x: 8i16.into(), y: (-16i16).into() },
        collision_state: 0,
        inputs: KeyMonitor::new(),
    };

    loop {
        spin_until_vdraw();
        spin_until_vblank();

        player.update();
        mode3::dma3_clear_to(BACKGROUND_COLOR);
        player.draw();
    }
}

// spin_until_* copied from examples. In a real game we'd use interrupts
#[inline]
fn spin_until_vblank() {
  while VCOUNT.read() < 160 {}
}

#[inline]
fn spin_until_vdraw() {
  while VCOUNT.read() >= 160 {}
}