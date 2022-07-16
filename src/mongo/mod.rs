use gmod::lua::State;
use mongodb::sync::Client;
use crate::bson;

mod collection;
mod database;
mod client;

pub fn raise_error(lua: State, msg: String) -> i32 {
  unsafe {
    lua.get_global(lua_string!("ErrorNoHaltWithStack"));
    lua.push_string(&msg);
    lua.pcall_ignore(1, 0);
    lua.push_boolean(false);
  }
  1
}


pub fn init_mongo(lua: State, idx: i32) {
  unsafe {
    lua.push_function(client::mongodb_client);
    lua.set_field(idx, lua_string!("client"));

    lua.push_function(bson::mongodb_bson_objectid);
    lua.set_field(idx, lua_string!("objectid"));
  }
}