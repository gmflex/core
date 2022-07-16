use gmod::lua::{State, LUA_TUSERDATA, LUA_TTABLE};
use mongodb::sync::Client;
use crate::mongo::database;
use crate::mongo::raise_error;
use mongodb::bson::Document;
use crate::bson;

#[lua_function]
unsafe fn mongodb_client_index(lua: State) -> i32 {
  let client: &Client = (lua.to_userdata(1) as *const Client).as_ref().unwrap();
  let field = match lua.get_string(2) {
    Some(str) => str.to_string(),
    None => return raise_error(lua, "Invalid field name".to_string())
  };

  match field.as_ref() {
    "databases" => {
      #[lua_function]
      unsafe extern "C-unwind" fn list_databases(lua: State) -> i32 {
        if lua.lua_type(1) == LUA_TUSERDATA {
          let client: &Client = (lua.to_userdata(1) as *const Client).as_ref().unwrap();
          let filter: Option<Document> = match lua.lua_type(2) {
            LUA_TTABLE => {
              match bson::from_lua(lua, 2) {
                Ok(doc) => match doc.as_document() {
                  Some(doc) => Some(doc.clone()),
                  None => return raise_error(lua, "Invalid filter".to_string())
                },
                Err(e) => return raise_error(lua, format!("{}", e))
              }
            },
            _ => None
          };
          let dbs = match client.list_database_names(filter, None) {
            Ok(dbs) => dbs,
            Err(e) => return raise_error(lua, format!("{}", e))
          };
        
          lua.new_table();
          let mut i = 1isize;
          for db in dbs {
            lua.push_integer(i);
            lua.push_string(&db);
            lua.set_table(-3);
            i += 1;
          }
          1
        } else {
          raise_error(lua, "Invalid client".to_string())
        }
      }
      lua.push_closure(list_databases, 0);
    },
    "database" => {
      #[lua_function]
      unsafe extern "C-unwind" fn get_database(lua: State) -> i32 {
        if lua.lua_type(1) == LUA_TUSERDATA {
          let client: &Client = (lua.to_userdata(1) as *const Client).as_ref().unwrap();
          database::mongodb_database_new(client.clone(), lua, None);
          1
        } else {
          raise_error(lua, "Invalid client".to_string())
        }
      }
      lua.push_closure(get_database, 1);
    },
    _ => {
      let dbs = match client.list_database_names(None, None) {
        Ok(dbs) => dbs,
        Err(e) => return raise_error(lua, format!("{}", e))
      };

      if dbs.contains(&field) {
        database::mongodb_database_new(client.clone(), lua, Some(&field));
      } else {
        return raise_error(lua, "Invalid field name".to_string());
      }
    }
  }

  1
}

#[lua_function]
unsafe fn mongodb_client_tostring(lua: State) -> i32 {
  lua.push_string("MongoDB::Client");
  1
}

#[lua_function]
pub unsafe fn mongodb_client(lua: State) -> i32 {
  let uri = lua.check_string(1).to_string();
  let client = match Client::with_uri_str(&uri) {
    Ok(client) => client,
    Err(e) => return raise_error(lua, format!("{}", e))
  };

  lua.new_table();

  lua.push_function(mongodb_client_index);
  lua.set_field(-2, lua_string!("__index"));

  lua.push_function(mongodb_client_tostring);
  lua.set_field(-2, lua_string!("__tostring"));

  lua.new_userdata(client, Some(-1));

  1
}