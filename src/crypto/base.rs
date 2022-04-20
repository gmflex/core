use radix64::{STD, FAST, URL_SAFE};
use gmod::lua::State;

// base64.std(string) -> string
#[lua_function]
pub unsafe fn base64_encode(lua: State) -> i32 {
  lua.push_string(&STD.encode(lua.check_binary_string(1)));
  1
}

// base64.fast(string) -> string
#[lua_function]
pub unsafe fn base64_fast_encode(lua: State) -> i32 {
  lua.push_string(&FAST.encode(lua.check_binary_string(1)));
  1
}

// base64.url(string) -> string
#[lua_function]
pub unsafe fn base64_url_encode(lua: State) -> i32 {
  lua.push_string(&URL_SAFE.encode(lua.check_binary_string(1)));
  1
}

#[lua_function]
pub unsafe fn base64_decode(lua: State) -> i32 {
  lua.push_string(&String::from_utf8(STD.decode(lua.check_binary_string(1)).unwrap()).unwrap());
  1
}

#[lua_function]
pub unsafe fn base64_fast_decode(lua: State) -> i32 {
  lua.push_string(&String::from_utf8(FAST.decode(lua.check_binary_string(1)).unwrap()).unwrap());
  1
}

#[lua_function]
pub unsafe fn base64_url_decode(lua: State) -> i32 {
  lua.push_string(&String::from_utf8(URL_SAFE.decode(lua.check_binary_string(1)).unwrap()).unwrap());
  1
}
