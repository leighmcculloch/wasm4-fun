#[cfg(feature = "buddy-alloc")]
mod alloc;
mod wasm4;
use wasm4::*;

#[rustfmt::skip]
const SMILEY: [u8; 8] = [
    0b11101111,
    0b11000111,
    0b10000011,
    0b00000001,
    0b10101011,
    0b00010001,
    0b10000011,
    0b11010111,
];

static mut POS_X: i32 = 32;
static mut POS_Y: i32 = 142;
static mut VELO_Y: i32 = 0;
static mut ACEL_Y: i32 = 0;

static mut UPDATE_COUNT: usize = 0;

#[no_mangle]
unsafe fn update() {
    UPDATE_COUNT += 1;

    *PALETTE = [0x554994, 0xF675A8, 0xF29393, 0xFFCCB3];

    *DRAW_COLORS = 0x03;
    rect(0, 150, 160, 10);

    *DRAW_COLORS = 0x02;
    text(&UPDATE_COUNT.to_string(), 10, 10);

    let gamepad = *GAMEPAD1;
    if gamepad & BUTTON_LEFT > 0 {
        POS_X -= 1;
    } else if gamepad & BUTTON_RIGHT > 0 {
        POS_X += 1;
    }
    if gamepad & BUTTON_UP > 0 {
        ACEL_Y += 1;
    } else {
        ACEL_Y -= 1;
        if ACEL_Y < 0 {
            ACEL_Y = 0;
        }
    }
    if ACEL_Y == 0 {
        VELO_Y -= 9;
    } else {
        VELO_Y += ACEL_Y;
    }
    if VELO_Y < -9 {
        VELO_Y = -9;
    }
    POS_Y -= VELO_Y;
    if POS_Y > 142 {
        POS_Y = 142;
    }

    *DRAW_COLORS = 0x02;
    blit(&SMILEY, POS_X, POS_Y, 8, 8, BLIT_1BPP);
}
