#![feature(
    proc_macro_hygiene
)]
#![allow(
    non_snake_case,
    unused
)]
#![allow(warnings)]
#![deny(
    deprecated
)]

mod imports;
mod system;
pub mod util;

#[skyline::main(name = "libspicy_spirits")]
pub fn main() {
    println!("[spicy_spirits_nro] Plugin Loading");
    system::install();
    println!("[spicy_spirits_nro] Plugin Loaded");
}