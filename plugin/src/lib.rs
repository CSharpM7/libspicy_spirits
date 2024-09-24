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

pub mod imports;
pub mod vars;
mod spirit_battles;
pub mod spirit_scripts;

#[no_mangle]
pub fn smashline_install() {
    install();
}

pub fn install() {
    crate::spirit_battles::install();
}

#[skyline::main(name = "smashline_nickspirits")]
pub fn main() {
    println!("[spicy_spirits_mod] Mod Loading");
    #[cfg(not(feature = "dev"))]{
        //Added a slight delay for loading the plugin so that the api can create a list of spirit battles
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