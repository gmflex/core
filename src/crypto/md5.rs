use gmod::lua::{State, LUA_TSTRING};
use md5::{Md5, Digest};
use hex;

// md5(string, ?string) -> string
#[lua_function]
pub unsafe fn md5_new(lua: State) -> i32 {
  let mut hasher = Md5::new();
  hasher.update(lua.check_binary_string(1));
  if lua.lua_type(2) == LUA_TSTRING {
    hasher.update(b"$");
    hasher.update(lua.check_binary_string(2));
  }

  let hash = hasher.finalize();
  let slice = hash.as_slice();
  
  lua.push_string(&hex::encode(slice));

  1
}