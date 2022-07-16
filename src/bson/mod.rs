use gmod::lua::State;
use serde_json::Value;
use mongodb::{
  bson,
  bson::{Bson, Document},
  bson::oid::ObjectId
};
use crate::mongo::raise_error;

pub type Result<T> = std::result::Result<T, String>;

pub unsafe fn from_lua(lua: State, idx: i32) -> Result<Bson> {
  let refs = lua.reference();
  lua.get_global(lua_string!("json"));
  lua.get_field(-1, lua_string!("encode"));
  lua.from_reference(refs);
  lua.pcall_ignore(1, 1);
  let jsstr = lua.check_string(-1).to_string();
  lua.dereference(refs);
  lua.pop_n(2);

  let js: Value = match serde_json::from_str(&jsstr) {
    Ok(js) => js,
    Err(e) => return Err(format!("{}", e))
  };

  match js.try_into() {
    Ok(bson) => Ok(bson),
    Err(e) => Err(format!("{}", e))
  }
}

// Leaves on top of stack BSON converted to JSON
pub unsafe fn to_lua(lua: State, obj: Bson) {
  let js: Value = match obj.clone().try_into() {
    Ok(js) => js,
    Err(e) => {
      raise_error(lua, format!("{}", e));
      return;
    }
  };

  lua.get_global(lua_string!("json"));
  lua.get_field(-1, lua_string!("decode"));
  lua.push_string(&js.to_string());
  lua.pcall_ignore(1, 1);
}

#[lua_function]
pub unsafe fn mongodb_bson_objectid(lua: State) -> i32 {
  let oid = ObjectId::new();
  lua.push_string(&oid.to_hex());
  1
}