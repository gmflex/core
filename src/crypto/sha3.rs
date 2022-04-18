use gmod::lua::{State, LUA_TSTRING};
use sha3::{Sha3_224, Sha3_256, Sha3_384, Sha3_512, 
  Keccak224, Keccak256, Keccak256Full, Keccak384, Keccak512, Digest};
use hex;

// sha3_224(string, ?string) -> string
#[lua_function]
pub unsafe fn sha3_224_new(lua: State) -> i32 {
  let mut hasher = Sha3_224::new();
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

// sha3_256(string, ?string) -> string
#[lua_function]
pub unsafe fn sha3_256_new(lua: State) -> i32 {
  let mut hasher = Sha3_256::new();
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

// sha3_384(string, ?string) -> string
#[lua_function]
pub unsafe fn sha3_384_new(lua: State) -> i32 {
  let mut hasher = Sha3_384::new();
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

// sha3_512(string, ?string) -> string
#[lua_function]
pub unsafe fn sha3_512_new(lua: State) -> i32 {
  let mut hasher = Sha3_512::new();
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

// keccak224(string, ?string) -> string
#[lua_function]
pub unsafe fn keccak224_new(lua: State) -> i32 {
  let mut hasher = Keccak224::new();
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

// keccak256(string, ?string) -> string
#[lua_function]
pub unsafe fn keccak256_new(lua: State) -> i32 {
  let mut hasher = Keccak256::new();
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

// keccak256full(string, ?string) -> string
#[lua_function]
pub unsafe fn keccak256full_new(lua: State) -> i32 {
  let mut hasher = Keccak256Full::new();
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

// keccak384(string, ?string) -> string
#[lua_function]
pub unsafe fn keccak384_new(lua: State) -> i32 {
  let mut hasher = Keccak384::new();
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

// keccak512(string, ?string) -> string
#[lua_function]
pub unsafe fn keccak512_new(lua: State) -> i32 {
  let mut hasher = Keccak512::new();
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