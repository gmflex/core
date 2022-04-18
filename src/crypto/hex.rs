use gmod::lua::{State, LUA_TSTRING};
use md5::{Md5, Digest};
use hex;

// hex(string) -> string
#[lua_function]
pub unsafe fn hex_encode(lua: State) -> i32 {
  let msg = lua.check_binary_string(1);
  lua.push_string(&hex::encode(msg));
  1
}

// unhex(string) -> string
#[lua_function]
pub unsafe fn hex_decode(lua: State) -> i32 {
  let shex = lua.check_binary_string(1);
  lua.push_string(&String::from_utf8(hex::decode(shex)
    .expect("Failed to decode Hex"))
    .expect("Failed to convert Hex to String"));
  1
}