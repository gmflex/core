use gmod::lua::State;

pub mod md5;
pub mod md6;
pub mod sha3;
pub mod hex;
pub mod base;

pub unsafe fn init_base(lua: State, idx: i32) {
  lua.new_table(); // base = {...} -1
  
  lua.push_function(base::base64_encode);
  lua.set_field(idx, lua_string!("encode"));

  lua.push_function(base::base64_fast_encode);
  lua.set_field(idx, lua_string!("fast_encode"));

  lua.push_function(base::base64_url_encode);
  lua.set_field(idx, lua_string!("url_encode"));

  lua.push_function(base::base64_decode);
  lua.set_field(idx, lua_string!("decode"));

  lua.push_function(base::base64_fast_decode);
  lua.set_field(idx, lua_string!("fast_decode"));

  lua.push_function(base::base64_url_decode);
  lua.set_field(idx, lua_string!("url_decode"));

  lua.set_field(idx, lua_string!("base64"));
  log!("[flex.rs.crypto.base64] initialized ✓");
}

pub unsafe fn init_hex(lua: State, idx: i32) {
  lua.push_function(hex::hex_encode);
  lua.set_field(idx, lua_string!("hex"));

  lua.push_function(hex::hex_decode);
  lua.set_field(idx, lua_string!("unhex"));
  log!("[flex.rs.crypto.hex] initialized ✓");
}

pub unsafe fn init_md5(lua: State, idx: i32) {
  lua.push_function(md5::md5_new);
  lua.set_field(idx, lua_string!("md5"));
  log!("[flex.rs.crypto.md5] initialized ✓");
}

pub unsafe fn init_md6(lua: State, idx: i32) {
  lua.push_function(md6::md6_new);
  lua.set_field(idx, lua_string!("md6"));
  log!("[flex.rs.crypto.md6] initialized ✓");
}

pub unsafe fn init_sha3(lua: State, idx: i32) {
  lua.push_function(sha3::sha3_224_new);
  lua.set_field(idx, lua_string!("sha3_224"));

  lua.push_function(sha3::sha3_256_new);
  lua.set_field(idx, lua_string!("sha3_256"));

  lua.push_function(sha3::sha3_384_new);
  lua.set_field(idx, lua_string!("sha3_384"));

  lua.push_function(sha3::sha3_512_new);
  lua.set_field(idx, lua_string!("sha3_512"));

  lua.push_function(sha3::keccak224_new);
  lua.set_field(idx, lua_string!("keccak224"));

  lua.push_function(sha3::keccak256_new);
  lua.set_field(idx, lua_string!("keccak256"));

  lua.push_function(sha3::keccak256_new);
  lua.set_field(idx, lua_string!("keccak256full"));

  lua.push_function(sha3::keccak384_new);
  lua.set_field(idx, lua_string!("keccak384"));

  lua.push_function(sha3::keccak512_new);
  lua.set_field(idx, lua_string!("keccak512"));
  log!("[flex.rs.crypto.sha3] initialized ✓");
}