#![no_std]

use gstd::{};
use pebbles_game_io::*;

static mut PEBBLES_GAME: Option<GameState> = None;

#[no_mangle]
extern "C" fn Init() {
    //You code here 
}

#[no_mangle]
extern "C" fn Handle() {
    //You code here 
}


#[no_mangle]
extern "C" fn State() {
    //You code here 
}
