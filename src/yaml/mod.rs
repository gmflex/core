use gmod::lua::State;
use yaml_rust::{Yaml, YamlLoader};

unsafe fn value_to_lua(lua: State, obj: Yaml) {
  match obj {
    yaml_rust::Yaml::Real(rl) => lua.push_number(rl.parse::<f64>().unwrap()),
    yaml_rust::Yaml::Integer(r) => lua.push_integer(r as isize),
    yaml_rust::Yaml::String(s) => lua.push_string(&s),
    yaml_rust::Yaml::Boolean(b) => lua.push_boolean(b),
    yaml_rust::Yaml::Alias(s) => lua.push_integer(s as isize),
    yaml_rust::Yaml::Null => lua.push_nil(),
    yaml_rust::Yaml::BadValue => lua.push_nil(),
    yaml_rust::Yaml::Array(ar) => {
      lua.new_table();

      let mut i = 1isize;
      for item in ar {
        lua.push_integer(i);
        value_to_lua(lua, item);
        lua.set_table(-3);
        i += 1;
      }
    }
    yaml_rust::Yaml::Hash(hs) => {
      lua.new_table();

      for (k, v) in hs {
        value_to_lua(lua, k);
        value_to_lua(lua, v);
        lua.set_table(-3);
      }
    }
  }
}

// yaml(string) -> table
#[lua_function]
pub unsafe fn yaml_new(lua: State) -> i32 {
  match YamlLoader::load_from_str(&String::from_utf8_lossy(lua.check_binary_string(1))) {
    Ok(doc) => {
      value_to_lua(lua, yaml_rust::Yaml::Array(doc));

      return 1
    },
    Err(err) => {
      lua.get_global(lua_string!("ErrorNoHaltWithStack"));
      lua.push_string(&format!("Failed to parse a YAML document: {}", err.to_string()));
      lua.pcall_ignore(1, 0);

      return 0
    }
  }
}

pub unsafe fn init_yaml(lua: State, idx: i32) {
  lua.push_function(yaml_new);
  lua.set_field(idx, lua_string!("yaml"));
  log!("[flex.rs.yaml] initialized ✓");
}