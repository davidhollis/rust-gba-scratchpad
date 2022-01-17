#![no_std]
#![no_main]

use gba::prelude::*;

const B: Color = Color::from_rgb(0, 0, 0);
const W: Color = Color::from_rgb(31, 31, 31);
const U: Color = Color::from_rgb(0, 0, 31);

// Icons
const A_BUTTON_ICON: [Color; 400] = [
  W, W, W, W, W, W, W, B, B, B, B, B, B, W, W, W, W, W, W, W,
  W, W, W, W, W, B, B, U, U, U, U, U, U, B, B, W, W, W, W, W,
  W, W, W, B, B, U, U, U, U, U, U, U, U, U, U, B, B, W, W, W,
  W, W, B, U, U, U, U, U, U, U, U, U, U, U, U, U, U, B, W, W,
  W, W, B, U, U, U, U, U, U, U, U, U, U, U, U, U, U, B, W, W,
  W, B, U, U, U, U, U, U, B, B, B, B, U, U, U, U, U, U, B, W,
  W, B, U, U, U, U, U, U, B, B, B, B, U, U, U, U, U, U, B, W,
  B, U, U, U, U, U, B, B, U, U, U, U, B, B, U, U, U, U, U, B,
  B, U, U, U, U, U, B, B, U, U, U, U, B, B, U, U, U, U, U, B,
  B, U, U, U, U, U, B, B, B, B, B, B, B, B, U, U, U, U, U, B,
  B, U, U, U, U, U, B, B, B, B, B, B, B, B, U, U, U, U, U, B,
  B, U, U, U, U, U, B, B, U, U, U, U, B, B, U, U, U, U, U, B,
  B, U, U, U, U, U, B, B, U, U, U, U, B, B, U, U, U, U, U, B,
  W, B, U, U, U, U, B, B, U, U, U, U, B, B, U, U, U, U, B, W,
  W, B, U, U, U, U, B, B, U, U, U, U, B, B, U, U, U, U, B, W,
  W, W, B, U, U, U, U, U, U, U, U, U, U, U, U, U, U, B, W, W,
  W, W, B, U, U, U, U, U, U, U, U, U, U, U, U, U, U, B, W, W,
  W, W, W, B, B, U, U, U, U, U, U, U, U, U, U, B, B, W, W, W,
  W, W, W, W, W, B, B, U, U, U, U, U, U, B, B, W, W, W, W, W,
  W, W, W, W, W, W, W, B, B, B, B, B, B, W, W, W, W, W, W, W,
];

const B_BUTTON_ICON: [Color; 400] = [
  W, W, W, W, W, W, W, B, B, B, B, B, B, W, W, W, W, W, W, W,
  W, W, W, W, W, B, B, U, U, U, U, U, U, B, B, W, W, W, W, W,
  W, W, W, B, B, U, U, U, U, U, U, U, U, U, U, B, B, W, W, W,
  W, W, B, U, U, U, U, U, U, U, U, U, U, U, U, U, U, B, W, W,
  W, W, B, U, U, U, U, U, U, U, U, U, U, U, U, U, U, B, W, W,
  W, B, U, U, U, U, B, B, B, B, B, B, U, U, U, U, U, U, B, W,
  W, B, U, U, U, U, B, B, B, B, B, B, U, U, U, U, U, U, B, W,
  B, U, U, U, U, U, B, B, U, U, U, U, B, B, U, U, U, U, U, B,
  B, U, U, U, U, U, B, B, U, U, U, U, B, B, U, U, U, U, U, B,
  B, U, U, U, U, U, B, B, B, B, B, B, U, U, U, U, U, U, U, B,
  B, U, U, U, U, U, B, B, B, B, B, B, U, U, U, U, U, U, U, B,
  B, U, U, U, U, U, B, B, U, U, U, U, B, B, U, U, U, U, U, B,
  B, U, U, U, U, U, B, B, U, U, U, U, B, B, U, U, U, U, U, B,
  W, B, U, U, U, U, B, B, B, B, B, B, U, U, U, U, U, U, B, W,
  W, B, U, U, U, U, B, B, B, B, B, B, U, U, U, U, U, U, B, W,
  W, W, B, U, U, U, U, U, U, U, U, U, U, U, U, U, U, B, W, W,
  W, W, B, U, U, U, U, U, U, U, U, U, U, U, U, U, U, B, W, W,
  W, W, W, B, B, U, U, U, U, U, U, U, U, U, U, B, B, W, W, W,
  W, W, W, W, W, B, B, U, U, U, U, U, U, B, B, W, W, W, W, W,
  W, W, W, W, W, W, W, B, B, B, B, B, B, W, W, W, W, W, W, W,
];

const L_BUTTON_ICON: [Color; 400] = [
  W, W, W, W, W, W, W, B, B, B, B, B, B, B, B, B, B, B, B, B,
  W, W, W, W, W, B, B, U, U, U, U, U, U, U, U, U, U, U, U, B,
  W, W, W, B, B, U, U, U, U, U, U, U, U, U, U, U, U, U, U, B,
  W, W, B, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, B,
  W, W, B, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, B,
  W, B, U, U, U, U, B, B, U, U, U, U, U, U, U, U, U, U, U, B,
  W, B, U, U, U, U, B, B, U, U, U, U, U, U, U, U, U, U, U, B,
  B, U, U, U, U, U, B, B, U, U, U, U, U, U, U, U, U, U, U, B,
  B, U, U, U, U, U, B, B, U, U, U, U, U, U, U, U, U, U, U, B,
  B, U, U, U, U, U, B, B, U, U, U, U, U, U, U, U, U, U, U, B,
  B, U, U, U, U, U, B, B, U, U, U, U, U, U, U, U, U, U, U, B,
  B, U, U, U, U, U, B, B, U, U, U, U, U, U, U, U, U, U, U, B,
  B, U, U, U, U, U, B, B, U, U, U, U, U, U, U, U, U, U, U, B,
  B, U, U, U, U, U, B, B, B, B, B, B, B, B, U, U, U, U, U, B,
  B, U, U, U, U, U, B, B, B, B, B, B, B, B, U, U, U, U, U, B,
  B, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, B,
  B, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, B,
  B, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, B,
  B, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, B,
  B, B, B, B, B, B, B, B, B, B, B, B, B, B, B, B, B, B, B, B,
];

const R_BUTTON_ICON: [Color; 400] = [
  B, B, B, B, B, B, B, B, B, B, B, B, B, W, W, W, W, W, W, W,
  B, U, U, U, U, U, U, U, U, U, U, U, U, B, B, W, W, W, W, W,
  B, U, U, U, U, U, U, U, U, U, U, U, U, U, U, B, B, W, W, W,
  B, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, B, W, W,
  B, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, B, W, W,
  B, U, U, U, U, U, B, B, B, B, B, B, U, U, U, U, U, U, B, W,
  B, U, U, U, U, U, B, B, B, B, B, B, U, U, U, U, U, U, B, W,
  B, U, U, U, U, U, B, B, U, U, U, U, B, B, U, U, U, U, U, B,
  B, U, U, U, U, U, B, B, U, U, U, U, B, B, U, U, U, U, U, B,
  B, U, U, U, U, U, B, B, B, B, B, B, U, U, U, U, U, U, U, B,
  B, U, U, U, U, U, B, B, B, B, B, B, U, U, U, U, U, U, U, B,
  B, U, U, U, U, U, B, B, U, U, U, U, B, B, U, U, U, U, U, B,
  B, U, U, U, U, U, B, B, U, U, U, U, B, B, U, U, U, U, U, B,
  B, U, U, U, U, U, B, B, U, U, U, U, B, B, U, U, U, U, U, B,
  B, U, U, U, U, U, B, B, U, U, U, U, B, B, U, U, U, U, U, B,
  B, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, B,
  B, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, B,
  B, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, B,
  B, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, B,
  B, B, B, B, B, B, B, B, B, B, B, B, B, B, B, B, B, B, B, B,
];

const UP_BUTTON_ICON: [Color; 400] = [
  B, B, B, B, B, B, B, B, B, B, B, B, B, B, B, B, B, B, B, B,
  B, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, B,
  B, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, B,
  B, U, U, U, U, U, U, U, U, B, B, U, U, U, U, U, U, U, U, B,
  B, U, U, U, U, U, U, U, U, B, B, U, U, U, U, U, U, U, U, B,
  B, U, U, U, U, U, U, U, B, B, B, B, U, U, U, U, U, U, U, B,
  B, U, U, U, U, U, U, U, B, B, B, B, U, U, U, U, U, U, U, B,
  B, U, U, U, U, U, U, B, B, B, B, B, B, U, U, U, U, U, U, B,
  B, U, U, U, U, U, U, B, B, B, B, B, B, U, U, U, U, U, U, B,
  B, U, U, U, U, U, B, B, B, B, B, B, B, B, U, U, U, U, U, B,
  B, U, U, U, U, U, B, B, B, B, B, B, B, B, U, U, U, U, U, B,
  B, U, U, U, U, B, B, B, B, B, B, B, B, B, B, U, U, U, U, B,
  B, U, U, U, U, B, B, B, B, B, B, B, B, B, B, U, U, U, U, B,
  B, U, U, U, B, B, B, B, B, B, B, B, B, B, B, B, U, U, U, B,
  B, U, U, U, B, B, B, B, B, B, B, B, B, B, B, B, U, U, U, B,
  B, U, U, B, B, B, B, B, B, B, B, B, B, B, B, B, B, U, U, B,
  B, U, U, B, B, B, B, B, B, B, B, B, B, B, B, B, B, U, U, B,
  B, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, B,
  B, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, B,
  B, B, B, B, B, B, B, B, B, B, B, B, B, B, B, B, B, B, B, B,
];

const DOWN_BUTTON_ICON: [Color; 400] = [
  B, B, B, B, B, B, B, B, B, B, B, B, B, B, B, B, B, B, B, B,
  B, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, B,
  B, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, B,
  B, U, U, B, B, B, B, B, B, B, B, B, B, B, B, B, B, U, U, B,
  B, U, U, B, B, B, B, B, B, B, B, B, B, B, B, B, B, U, U, B,
  B, U, U, U, B, B, B, B, B, B, B, B, B, B, B, B, U, U, U, B,
  B, U, U, U, B, B, B, B, B, B, B, B, B, B, B, B, U, U, U, B,
  B, U, U, U, U, B, B, B, B, B, B, B, B, B, B, U, U, U, U, B,
  B, U, U, U, U, B, B, B, B, B, B, B, B, B, B, U, U, U, U, B,
  B, U, U, U, U, U, B, B, B, B, B, B, B, B, U, U, U, U, U, B,
  B, U, U, U, U, U, B, B, B, B, B, B, B, B, U, U, U, U, U, B,
  B, U, U, U, U, U, U, B, B, B, B, B, B, U, U, U, U, U, U, B,
  B, U, U, U, U, U, U, B, B, B, B, B, B, U, U, U, U, U, U, B,
  B, U, U, U, U, U, U, U, B, B, B, B, U, U, U, U, U, U, U, B,
  B, U, U, U, U, U, U, U, B, B, B, B, U, U, U, U, U, U, U, B,
  B, U, U, U, U, U, U, U, U, B, B, U, U, U, U, U, U, U, U, B,
  B, U, U, U, U, U, U, U, U, B, B, U, U, U, U, U, U, U, U, B,
  B, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, B,
  B, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, B,
  B, B, B, B, B, B, B, B, B, B, B, B, B, B, B, B, B, B, B, B,
];

const LEFT_BUTTON_ICON: [Color; 400] = [
  B, B, B, B, B, B, B, B, B, B, B, B, B, B, B, B, B, B, B, B,
  B, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, B,
  B, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, B,
  B, U, U, U, U, U, U, U, U, U, U, U, U, U, U, B, B, U, U, B,
  B, U, U, U, U, U, U, U, U, U, U, U, U, B, B, B, B, U, U, B,
  B, U, U, U, U, U, U, U, U, U, U, B, B, B, B, B, B, U, U, B,
  B, U, U, U, U, U, U, U, U, B, B, B, B, B, B, B, B, U, U, B,
  B, U, U, U, U, U, U, B, B, B, B, B, B, B, B, B, B, U, U, B,
  B, U, U, U, U, B, B, B, B, B, B, B, B, B, B, B, B, U, U, B,
  B, U, U, B, B, B, B, B, B, B, B, B, B, B, B, B, B, U, U, B,
  B, U, U, B, B, B, B, B, B, B, B, B, B, B, B, B, B, U, U, B,
  B, U, U, U, U, B, B, B, B, B, B, B, B, B, B, B, B, U, U, B,
  B, U, U, U, U, U, U, B, B, B, B, B, B, B, B, B, B, U, U, B,
  B, U, U, U, U, U, U, U, U, B, B, B, B, B, B, B, B, U, U, B,
  B, U, U, U, U, U, U, U, U, U, U, B, B, B, B, B, B, U, U, B,
  B, U, U, U, U, U, U, U, U, U, U, U, U, B, B, B, B, U, U, B,
  B, U, U, U, U, U, U, U, U, U, U, U, U, U, U, B, B, U, U, B,
  B, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, B,
  B, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, B,
  B, B, B, B, B, B, B, B, B, B, B, B, B, B, B, B, B, B, B, B,
];

const RIGHT_BUTTON_ICON: [Color; 400] = [
  B, B, B, B, B, B, B, B, B, B, B, B, B, B, B, B, B, B, B, B,
  B, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, B,
  B, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, B,
  B, U, U, B, B, U, U, U, U, U, U, U, U, U, U, U, U, U, U, B,
  B, U, U, B, B, B, B, U, U, U, U, U, U, U, U, U, U, U, U, B,
  B, U, U, B, B, B, B, B, B, U, U, U, U, U, U, U, U, U, U, B,
  B, U, U, B, B, B, B, B, B, B, B, U, U, U, U, U, U, U, U, B,
  B, U, U, B, B, B, B, B, B, B, B, B, B, U, U, U, U, U, U, B,
  B, U, U, B, B, B, B, B, B, B, B, B, B, B, B, U, U, U, U, B,
  B, U, U, B, B, B, B, B, B, B, B, B, B, B, B, B, B, U, U, B,
  B, U, U, B, B, B, B, B, B, B, B, B, B, B, B, B, B, U, U, B,
  B, U, U, B, B, B, B, B, B, B, B, B, B, B, B, U, U, U, U, B,
  B, U, U, B, B, B, B, B, B, B, B, B, B, U, U, U, U, U, U, B,
  B, U, U, B, B, B, B, B, B, B, B, U, U, U, U, U, U, U, U, B,
  B, U, U, B, B, B, B, B, B, U, U, U, U, U, U, U, U, U, U, B,
  B, U, U, B, B, B, B, U, U, U, U, U, U, U, U, U, U, U, U, B,
  B, U, U, B, B, U, U, U, U, U, U, U, U, U, U, U, U, U, U, B,
  B, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, B,
  B, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, B,
  B, B, B, B, B, B, B, B, B, B, B, B, B, B, B, B, B, B, B, B,
];

const START_BUTTON_ICON: [Color; 400] = [
  W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W,
  W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W,
  W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W,
  W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W,
  W, W, W, B, B, B, B, B, B, B, B, B, B, B, B, B, B, W, W, W,
  W, W, B, U, U, U, U, U, U, U, U, U, U, U, U, U, U, B, W, W,
  W, B, U, U, U, B, B, B, U, U, U, B, B, B, B, B, U, U, B, W,
  W, B, U, U, B, U, U, U, B, U, U, U, U, B, U, U, U, U, B, W,
  B, U, U, U, B, U, U, U, U, U, U, U, U, B, U, U, U, U, U, B,
  B, U, U, U, U, B, B, B, U, U, U, U, U, B, U, U, U, U, U, B,
  B, U, U, U, U, U, U, U, B, U, U, U, U, B, U, U, U, U, U, B,
  B, U, U, U, U, U, U, U, B, U, U, U, U, B, U, U, U, U, U, B,
  W, B, U, U, B, U, U, U, B, U, U, U, U, B, U, U, U, U, B, W,
  W, B, U, U, U, B, B, B, U, U, U, U, U, B, U, U, U, U, B, W,
  W, W, B, U, U, U, U, U, U, U, U, U, U, U, U, U, U, B, W, W,
  W, W, W, B, B, B, B, B, B, B, B, B, B, B, B, B, B, W, W, W,
  W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W,
  W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W,
  W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W,
  W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W,
];

const SELECT_BUTTON_ICON: [Color; 400] = [
  W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W,
  W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W,
  W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W,
  W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W,
  W, W, W, B, B, B, B, B, B, B, B, B, B, B, B, B, B, W, W, W,
  W, W, B, U, U, U, U, U, U, U, U, U, U, U, U, U, U, B, W, W,
  W, B, U, U, U, B, B, B, U, U, U, B, U, U, U, U, U, U, B, W,
  W, B, U, U, B, U, U, U, B, U, U, B, U, U, U, U, U, U, B, W,
  B, U, U, U, B, U, U, U, U, U, U, B, U, U, U, U, U, U, U, B,
  B, U, U, U, U, B, B, B, U, U, U, B, U, U, U, U, U, U, U, B,
  B, U, U, U, U, U, U, U, B, U, U, B, U, U, U, U, U, U, U, B,
  B, U, U, U, U, U, U, U, B, U, U, B, U, U, U, U, U, U, U, B,
  W, B, U, U, B, U, U, U, B, U, U, B, U, U, U, U, U, U, B, W,
  W, B, U, U, U, B, B, B, U, U, U, B, B, B, B, B, U, U, B, W,
  W, W, B, U, U, U, U, U, U, U, U, U, U, U, U, U, U, B, W, W,
  W, W, W, B, B, B, B, B, B, B, B, B, B, B, B, B, B, W, W, W,
  W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W,
  W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W,
  W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W,
  W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W,
];

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
  loop {}
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

#[inline]
fn draw_icon(x: usize, y: usize, width: usize, icon: &[Color], enabled: bool) {
  for (idx, color) in icon.iter().enumerate() {
    let dx = idx % width;
    let dy = idx / width;
    let modified_color =
      if *color == U && !enabled {
        W
      } else {
        *color
      };
    mode3::bitmap_xy(x + dx, y + dy).write(modified_color);
  }
}

#[no_mangle]
fn main() -> ! {
  const SETUP_DISPLAY: DisplayControl = DisplayControl::new().with_display_mode(3).with_display_bg2(true);
  DISPCNT.write(SETUP_DISPLAY);

  mode3::dma3_clear_to(W);

  let mut keys: Keys = KEYINPUT.read().into();

  loop {
    spin_until_vdraw();
    spin_until_vblank();
    
    draw_icon(20,  20,  20, &L_BUTTON_ICON, keys.l());
    draw_icon(200, 20,  20, &R_BUTTON_ICON, keys.r());
    draw_icon(170, 60,  20, &A_BUTTON_ICON, keys.a());
    draw_icon(150, 80,  20, &B_BUTTON_ICON, keys.b());
    draw_icon(60,  50,  20, &UP_BUTTON_ICON, keys.up());
    draw_icon(40,  70,  20, &LEFT_BUTTON_ICON, keys.left());
    draw_icon(80,  70,  20, &RIGHT_BUTTON_ICON, keys.right());
    draw_icon(60,  90,  20, &DOWN_BUTTON_ICON, keys.down());
    draw_icon(60,  114, 20, &START_BUTTON_ICON, keys.start());
    draw_icon(60,  132, 20, &SELECT_BUTTON_ICON, keys.select());
    
    // read our keys for this frame
    keys = KEYINPUT.read().into();
  }
}
