#![no_main]
#![no_std]

extern crate alloc;
use alloc::format;

risc0_zkvm::guest::entry!(main);

use rlua::Lua;
use risc0_zkvm::guest::env;

pub fn main() {
    let (x, y): (u64, u64) = (env::read(), env::read());

    Lua::new().context(|lua| {
        let res: u64 = lua.load(&format!(
            r#"
                return {} + {}
            "#, x, y)).eval::<u64>().unwrap();

        env::commit(&res);
    });
}

