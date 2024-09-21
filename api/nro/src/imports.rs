pub mod imports_agent {
    pub use {
        smash::{
            lib::{
                L2CValue,
                L2CAgent,
                lua_const::*
            },
            app::{
                *,
                self,
                lua_bind::*,
            },
            hash40,
            lua2cpp::*,
            phx::*
        },
        smash_script::{
            *,
            macros::*
        },
        smashline::{
            *,
            Main,
        },
        
        sharpsmashlinesuite::{
            *,
            util::{
                *,
                self
            },
            ext::*,
            //getvar::*
        },
    };
    pub unsafe extern "C" fn empty_status(agent: &mut L2CAgentBase) -> L2CValue {
        0.into()
    }
}
pub mod imports_status {
    pub use {
        crate::imports::imports_agent::*,
        smashline::{
            *,
            Init,
            Pre,
            Main,
            Exec,
            End,
        },
    };
}