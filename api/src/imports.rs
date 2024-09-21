pub mod imports {
    pub use {
        smash::{
            lib::{
                lua_const::*
            },
            app::{
                *,
                self,
                sv_animcmd::{
                    frame,
                    wait
                },
                lua_bind::*
            },
            hash40,
            lua2cpp::*,
            phx::*
        }
    };
}