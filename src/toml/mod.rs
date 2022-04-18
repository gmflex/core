use gmod::lua::{
  State, 
  LUA_TNONE, LUA_TNIL, LUA_TBOOLEAN, LUA_TLIGHTUSERDATA, LUA_TNUMBER,
  LUA_TSTRING, LUA_TTABLE, LUA_TFUNCTION, LUA_TUSERDATA, LUA_TTHREAD
};
use toml;
use toml::Value;

// Leaves on top value converted to LuaValue
unsafe fn value_to_lua(lua: State, obj: Value) {
  match obj {
    Value::String(s) => lua.push_string(&s),
    Value::Integer(i) => lua.push_integer(i as isize),
    Value::Float(f) => lua.push_number(f),
    Value::Boolean(b) => lua.push_boolean(b),
    Value::Datetime(dt) => lua.push_string(&dt.to_string()),
    Value::Array(ar) => {
      lua.new_table();
      
      let mut i = 1isize;
      for item in ar {
        lua.push_integer(i);
        value_to_lua(lua, item);
        lua.set_table(-3);
        i += 1;
      }
    }
    Value::Table(tb) => {
      lua.new_table();

      for (k, v) in tb {
        lua.push_string(&k);
        value_to_lua(lua, v);
        lua.set_table(-3);
      }
    }
  }
}

// toml(string) -> table
#[lua_function]
pub unsafe fn toml_new(lua: State) -> i32 {
  match String::from_utf8_lossy(lua.check_binary_string(1)).to_string().parse::<Value>() {
    Ok(doc) => {
      value_to_lua(lua, doc);

      return 1
    },
    Err(err) => { 
      lua.get_global(lua_string!("ErrorNoHaltWithStack"));
      lua.push_string(&format!("Failed to parse a TOML document: {}", err.to_string()));
      lua.pcall_ignore(1, 0);

      return 0
    }
  }
}

pub unsafe fn init_toml(lua: State, idx: i32) {
  lua.push_function(toml_new);
  lua.set_field(idx, lua_string!("toml"));
  log!("[flex.rs.toml] initialized ✓");
}