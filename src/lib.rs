#![feature(c_unwind)]
#![feature(const_fn_trait_bound)]
#![feature(path_try_exists)]

#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

#[macro_use] extern crate paris;
#[macro_use] extern crate gmod;
use gmod::lua::State;
use paris::Logger;

mod crypto;
mod toml;
mod yaml;
mod file;
//mod bson;

#[gmod13_open]
unsafe fn gmod13_open(lua: State) -> i32 {
  let log = Logger::new();

  log!("[flex.rs] initializing...");
  lua.new_table(); // base table = -1

  // crypto
  lua.new_table(); // crypto = {...} -2
  crypto::init_hex(lua, -2);
  crypto::init_md5(lua, -2);
  crypto::init_md6(lua, -2);
  crypto::init_sha3(lua, -2);
  lua.set_field(-2, lua_string!("crypto"));
  // crypto end

  // toml
  toml::init_toml(lua, -2); // flex.toml(string) -> table
  // toml end

  // yaml
  yaml::init_yaml(lua, -2); // flex.yaml(string) -> table
  // yaml end

  // bson
  //lua.new_table(); // bson = {...} -2
  //lua.push_function(bson::bson_new);
  //lua.set_field(-2, lua_string!("test"));
  //lua.set_field(-2, lua_string!("bson"));
  // bson end

  // file.sync
  lua.new_table(); // file = {...}; -2
  file::init_sync(lua, -2);
  lua.set_field(-2, lua_string!("file"));

  lua.set_global(lua_string!("flex"));
  log!("[flex.rs] initialized successfully");
  1
}

#[gmod13_close]
unsafe fn gmod13_close(lua: State) -> i32 {
  let mut log = Logger::new();
  
  log.log("[flex.rs] deinitializing...");

  log.log("[flex.rs] deinitialized successfully");

  0
}