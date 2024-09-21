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
mod fighter;
mod spirit_battles;
pub mod spirit_scripts;

#[no_mangle]
pub fn smashline_install() {
    install();
}

pub fn install() {
    crate::fighter::install();
}

#[skyline::main(name = "smashline_nickspirits")]
pub fn main() {
    println!("[spicy_spirits_mod] Mod Loading");
    #[cfg(not(feature = "dev"))]{
        std::thread::Builder::new()
            .stack_size(0x40_0000)
            .spawn(|| {
                crate::spirit_battles::install();
            })
            .unwrap()
        .join();

        #[cfg(feature = "devhook")]
        return;

        install();
    }

    #[cfg(feature = "dev")]
    smashline_install();

    println!("[spicy_spirits_mod] Mod Loaded");
}