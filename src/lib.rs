#![feature(
    concat_idents,
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

#[macro_use]
extern crate lazy_static;

pub mod imports;
pub mod vars;
mod common;
mod hook;

#[no_mangle]
pub fn smashline_install() {
    install();
}

pub fn install() {
    crate::common::install();
}

#[skyline::main(name = "smashline_nickspirits")]
pub fn main() {
    #[cfg(not(feature = "dev"))]{
        //hook::install();

        #[cfg(feature = "devhook")]
        return;

        install();
    }

    #[cfg(feature = "dev")]
    smashline_install();
}