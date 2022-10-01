#![feature(c_unwind)]
#![feature(fs_try_exists)]

#[macro_use] extern crate gmod;
#[macro_use] extern crate magic_static;

use gmod::lua::State;
use fast_log::{
  Config,
  plugin::{
    file_split::RollingType,
    packer::LogPacker,
  },
  consts::LogSize,
};
  
mod utils;
#[cfg(all(not(feature = "toml"), not(feature = "yaml")))]
compile_error!("You must enable at least one of the following features: toml, yaml");

#[cfg(all(feature = "mongo_async", feature = "mongo_sync"))]
compile_error!("You must enable exactly one of the following features: mongo_async, mongo_sync");

//#[cfg(all(feature = "redis_async", feature = "redis_sync"))]
//compile_error!("You must enable exactly one of the following features: redis_async, redis_sync");

#[cfg(any(feature="crypto-sha3", feature="crypto-md5",
feature="crypto-md6", feature="crypto-base64"))] mod crypto;
#[cfg(feature = "toml")] mod toml;
#[cfg(feature = "yaml")] mod yaml;
#[cfg(feature = "fs_sync")] mod fs_sync;
#[cfg(feature = "fs_async")] mod fs_async;
#[cfg(any(feature = "mongo_async", feature = "mongo_sync"))] mod bson;
#[cfg(feature = "mongo_sync")] mod mongo_sync;
#[cfg(feature = "mongo_async")] mod mongo_async;
#[cfg(feature = "redis_sync")] mod redis_sync;
#[cfg(feature = "redis_async")] mod redis_async;

unsafe fn load(lua: State) {
  utils::load(lua);
  #[cfg(feature = "toml")] toml::load(lua);
  #[cfg(feature = "yaml")] yaml::load(lua);
  #[cfg(feature = "fs_sync")] fs_sync::load(lua);
  #[cfg(feature = "fs_async")] fs_async::load(lua);
  #[cfg(feature = "mongo_sync")] mongo_sync::load(lua);
  #[cfg(feature = "mongo_async")] mongo_async::load(lua);
  //#[cfg(feature = "redis_sync")] redis_sync::load(lua);
  //#[cfg(feature = "redis_async")] redis_async::load(lua);
  #[cfg(any(feature="crypto-sha3", feature="crypto-md5",
  feature="crypto-md6", feature="crypto-base64"))] crypto::load(lua);
}

#[gmod13_open]
unsafe fn gmod13_open(lua: State) -> i32 {
  fast_log::init(Config::new()
    .console()
    .file_split("logs/flex/",
               LogSize::MB(5),
               RollingType::All,
               LogPacker {})).unwrap();

  #[cfg(feature = "mongo_async")] mongo_async::worker::init();
  
  lua.new_table(); // base table = -1
  
  load(lua);
  
  lua.set_global(lua_string!("flex"));
  
  info!("flcore loaded");
  0
}

#[gmod13_close]
unsafe fn gmod13_close(lua: State) -> i32 {
  #[cfg(feature = "mongo_async")] mongo_async::worker::shutdown(lua);
  
  info!("flcore unloaded");
  0
}
