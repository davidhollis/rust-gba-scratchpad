#![no_std]
#![no_main]
#![feature(asm)]

use core::cmp::{ max, min };

use gba::prelude::*;
use gbainputs::{ Key, KeyMonitor };
use gbamath::fixed::{ UFixed8, SFixed8 };
use gbamath::geometry::BoundingBox;
use gbamath::Vec2D;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    mode3::dma3_clear_to(Color::from_rgb(31,0,0));
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
    inputs: KeyMonitor,
    state: PlayerState,
    collision_state: u8,
}

impl Player {
    const COLOR: Color = Color::from_rgb(0, 0, 31);
    const CENTER_COLOR: Color = Color::from_rgb(0, 31, 0);
    const ANCHOR_COLOR: Color = Color::from_rgb(31, 31, 0);

    const WALK_SPEED: SFixed8 = SFixed8::constant(2i16);

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
        self.inputs.update();

        match self.state {
            PlayerState::Standing => {
                self.current_velocity = VEC2D_ZERO;

                if self.inputs.is_pressed(Key::LEFT) != self.inputs.is_pressed(Key::RIGHT) {
                    self.state = PlayerState::Walking;
                }
            },
            PlayerState::Walking => {
                if self.inputs.is_pressed(Key::LEFT) == self.inputs.is_pressed(Key::RIGHT) {
                    self.state = PlayerState::Standing;
                    self.current_velocity = VEC2D_ZERO;
                } else if self.inputs.is_pressed(Key::RIGHT) {
                    if self.collision_state & Player::RIGHT_COLLISION_THIS_FRAME > 0 {
                        self.current_velocity.x = SFixed8::ZERO;
                    } else {
                        self.current_velocity.x = Player::WALK_SPEED;
                    }
                } else if self.inputs.is_pressed(Key::LEFT) {
                    if self.collision_state & Player::LEFT_COLLISION_THIS_FRAME > 0 {
                        self.current_velocity.x = SFixed8::ZERO;
                    } else {
                        self.current_velocity.x = -Player::WALK_SPEED;
                    }
                }
            },
        };
        
        self.update_physics();
    }

    fn update_physics(&mut self) {
        self.old_position = self.current_position;
        self.old_velocity = self.current_velocity;
        self.collision_state = (self.collision_state << 1) & Player::LAST_FRAME_STATES;
        self.current_position.saturating_add_signed_assign(self.current_velocity);
        self.collision_box.center = self.current_position.saturating_add_signed(self.box_offset);

        if self.collision_box.left() < LEFT_WALL_X {
            let new_collision_box_center_x = LEFT_WALL_X + self.collision_box.half_size.x;
            self.current_position.x = new_collision_box_center_x.saturating_add_signed(-self.box_offset.x);
            self.collision_box.center.x = new_collision_box_center_x;
            self.current_velocity.x = max(self.current_velocity.x, SFixed8::ZERO);
            self.collision_state |= Player::LEFT_COLLISION_THIS_FRAME;
        } else if self.collision_box.right() > RIGHT_WALL_X {
            let new_collision_box_center_x = RIGHT_WALL_X - self.collision_box.half_size.x;
            self.current_position.x = new_collision_box_center_x.saturating_add_signed(-self.box_offset.x);
            self.collision_box.center.x = new_collision_box_center_x;
            self.current_velocity.x = min(self.current_velocity.x, SFixed8::ZERO);
            self.collision_state |= Player::RIGHT_COLLISION_THIS_FRAME;
        }
    }

    #[inline]
    fn draw(&self) {
        let left: u16 = self.collision_box.left().into();
        let right: u16 = self.collision_box.right().into();
        let top: u16 = self.collision_box.top().into();
        let bottom: u16 = self.collision_box.bottom().into();
        fill_rect(left, right, top, bottom, Player::COLOR);
        let center_x: u16 = self.collision_box.center.x.into();
        let center_y: u16 = self.collision_box.center.y.into();
        mode3::bitmap_xy(center_x.into(), center_y.into()).write(Player::CENTER_COLOR);
        let player_x: u16 = self.current_position.x.into();
        let player_y: u16 = self.current_position.y.into();
        mode3::bitmap_xy(player_x.into(), player_y.into()).write(Player::ANCHOR_COLOR);
    }
}

const BACKGROUND_COLOR: Color = Color::from_rgb(15, 15, 15);
const WALL_COLOR: Color = Color::from_rgb(0, 0, 0);
const VEC2D_ZERO: Vec2D<SFixed8> = Vec2D { x: SFixed8::ZERO, y: SFixed8::ZERO };
const LEFT_WALL_X: UFixed8 = UFixed8::constant(20u16);
const RIGHT_WALL_X: UFixed8 = UFixed8::constant(219u16);
const FLOOR_Y: u16 = 130u16;

#[no_mangle]
fn main() -> ! {
    const SETUP_DISPLAY: DisplayControl = DisplayControl::new().with_display_mode(3).with_display_bg2(true);
    DISPCNT.write(SETUP_DISPLAY);

    let mut player: Player = Player {
        old_position: Vec2D { x: UFixed8::from(100u16), y: FLOOR_Y.into() },
        current_position: Vec2D { x: UFixed8::from(100u16), y: FLOOR_Y.into() },
        old_velocity: Vec2D { x: SFixed8::ZERO, y: SFixed8::ZERO },
        current_velocity: Vec2D { x: SFixed8::ZERO, y: SFixed8::ZERO },
        collision_box: BoundingBox {
            center: Vec2D { x: UFixed8::ZERO, y: UFixed8::ZERO },
            half_size: Vec2D { x: 8u16.into(), y: 16u16.into() },
        },
        box_offset: Vec2D { x: 8i16.into(), y: (-16i16).into() },
        inputs: KeyMonitor::new(),
        state: PlayerState::Standing,
        collision_state: 0,
    };

    mode3::dma3_clear_to(WALL_COLOR);
    loop {
        spin_until_vdraw();
        
        player.update();
        
        spin_until_vblank();
        
        fill_rect(
            u16::from(LEFT_WALL_X), u16::from(RIGHT_WALL_X),
            0u16, FLOOR_Y,
            BACKGROUND_COLOR
        );
        player.draw();
    }
}

#[inline]
fn fill_rect(left: u16, right: u16, top: u16, bottom: u16, color: Color) {
    let raw_color: u16 = color.0;
    let word_count: u16 = right - left + 1;
    for y in top..=bottom {
        unsafe {
            DMA3SAD.write(&raw_color as *const _ as usize);
            DMA3DAD.write(0x0600_0000usize + ((mode3::WIDTH * (y as usize)) + (left as usize)) * 2usize);
            DMA3CNT_L.write(word_count);
            const CTRL: DmaControl = DmaControl::new()
                .with_dest_addr(DestAddrControl::Increment)
                .with_src_addr(SrcAddrControl::Fixed)
                .with_transfer_u32(false)
                .with_start_time(DmaStartTiming::Immediately)
                .with_enabled(true);
            DMA3CNT_H.write(CTRL);
            // Not sure why this is necessary, but here we go
            asm!(
                "
                nop
                nop
                ",
                options(nostack),
            );
        }
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