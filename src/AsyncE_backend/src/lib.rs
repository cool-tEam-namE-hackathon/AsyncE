#![allow(non_snake_case)]

pub mod globals;
pub mod group;
pub mod user;

use getrandom::register_custom_getrandom;

fn custom_getrandom(_buf: &mut [u8]) -> Result<(), getrandom::Error> {
    // TODO get some randomness
    return Ok(());
}

register_custom_getrandom!(custom_getrandom);
