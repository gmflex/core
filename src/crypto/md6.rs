use gmod::lua::{State, LUA_TSTRING};
use md6::Md6;
use hex;

// md6(number, string, ?string) -> string
#[lua_function]
pub unsafe fn md6_new(lua: State) -> i32 {
  let size = lua.check_integer(1);
  
  if size < 1 || size > 512 {
    lua.get_global(lua_string!("ErrorNoHaltWithStack"));
    lua.push_string("Hash output length must be between 1 and 512");
    lua.pcall_ignore(1, 0);

    return 0;
  }

  let mut slice = Vec::with_capacity(size.unsigned_abs());
  let mut hasher = Md6::new(size as i32).unwrap();
  hasher.update(lua.check_binary_string(1));
  if lua.lua_type(2) == LUA_TSTRING {
    hasher.update(b"$");
    hasher.update(lua.check_binary_string(2));
  }
  hasher.finalise(&mut slice);

  lua.push_string(&hex::encode(slice));

  1
}