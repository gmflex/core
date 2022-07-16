use gmod::lua::{
  State,
  LUA_TUSERDATA, LUA_TTABLE
};
use mongodb::{
  sync::{Client, Database},
  bson::{Bson, Document},
};
use crate::{
  mongo::{raise_error, collection},
  bson
};

#[lua_function]
unsafe fn mongodb_database_index(lua: State) -> i32 {
  let db: &Database = (lua.to_userdata(1) as *const Database).as_ref().unwrap();
  let field = match lua.get_string(2) {
    Some(str) => str.to_string(),
    None => return raise_error(lua, "Invalid field name".to_string())
  };

  match field.as_ref() {
    "name" => {
      #[lua_function]
      unsafe extern "C-unwind" fn get_name(lua: State) -> i32 {
        if lua.lua_type(1) == LUA_TUSERDATA {
          let db: &Database = (lua.to_userdata(1) as *const Database).as_ref().unwrap();
          lua.push_string(db.name());
          1
        } else {
          raise_error(lua, "Invalid database".to_string())
        }
      }
      lua.push_closure(get_name, 0);
    },
    "drop" => {
      #[lua_function]
      unsafe extern "C-unwind" fn drop(lua: State) -> i32 {
        if lua.lua_type(1) == LUA_TUSERDATA {
          let db: &Database = (lua.to_userdata(1) as *const Database).as_ref().unwrap();
          match db.drop(None) {
            Ok(_) => {
              lua.push_boolean(true);
              1
            },
            Err(e) => return raise_error(lua, format!("{}", e))
          }
        } else {
          raise_error(lua, "Invalid database".to_string())
        }
      }
      lua.push_closure(drop, 0);
    },
    "collections" => {
      #[lua_function]
      unsafe extern "C-unwind" fn list_collections(lua: State) -> i32 {
        if lua.lua_type(1) == LUA_TUSERDATA {
          let db: &Database = (lua.to_userdata(1) as *const Database).as_ref().unwrap();
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
          let collections = match db.list_collection_names(filter) {
            Ok(collections) => collections,
            Err(e) => return raise_error(lua, format!("{}", e))
          };

          lua.new_table();
          let mut i = 1isize;
          for collection in collections {
            lua.push_integer(i);
            lua.push_string(&collection);
            lua.set_table(-3);
            i += 1;
          }
          1
        } else {
          raise_error(lua, "Invalid database".to_string())
        }
      }
      lua.push_closure(list_collections, 0);
    },
    "collection" => {
      #[lua_function]
      unsafe extern "C-unwind" fn get_collection(lua: State) -> i32 {
        if lua.lua_type(1) == LUA_TUSERDATA {
          let db: &Database = (lua.to_userdata(1) as *const Database).as_ref().unwrap();
          collection::mongodb_collection_new(db.clone(), lua, None);
          1
        } else {
          raise_error(lua, "Invalid database".to_string())
        }
      }
      lua.push_closure(get_collection, 0);
    },
    "create" => {
      #[lua_function]
      unsafe extern "C-unwind" fn create(lua: State) -> i32 {
        if lua.lua_type(1) == LUA_TUSERDATA {
          let db: &Database = (lua.to_userdata(1) as *const Database).as_ref().unwrap();
          let name = match lua.get_string(2) {
            Some(str) => str.to_string(),
            None => return raise_error(lua, "Invalid collection name".to_string())
          };

          match db.create_collection(&name, None) {
            Ok(_) => {
              lua.push_boolean(true);
            },
            Err(e) => return raise_error(lua, format!("{}", e))
          }
          1
        } else {
          raise_error(lua, "Invalid database".to_string())
        }
      }
      lua.push_closure(create, 0);
    },
    "aggregate" => {
      #[lua_function]
      unsafe extern "C-unwind" fn aggregate(lua: State) -> i32 {
        if lua.lua_type(1) == LUA_TUSERDATA {
          let db: &Database = (lua.to_userdata(1) as *const Database).as_ref().unwrap();
          let pipeline: Vec<Document> = match lua.lua_type(2) {
            LUA_TTABLE => {
              match bson::from_lua(lua, 2) {
                Ok(doc) => match doc.as_array() {
                  Some(arr) => arr.iter().map(|x| x.as_document().unwrap().clone()).collect(),
                  None => return raise_error(lua, "Invalid pipeline".to_string())
                },
                Err(e) => return raise_error(lua, format!("{}", e))
              }
            },
            _ => return raise_error(lua, "Invalid pipeline".to_string())
          };

          let cursor = match db.aggregate(pipeline, None) {
            Ok(cursor) => cursor,
            Err(e) => return raise_error(lua, format!("{}", e))
          };

          let arr = cursor
            .into_iter()
            .map(|doc| Bson::Document(doc.unwrap().clone()))
            .collect::<Vec<Bson>>();
          bson::to_lua(lua, Bson::Array(arr));
          1
        } else {
          raise_error(lua, "Invalid database".to_string())
        }
      }
      lua.push_closure(aggregate, 0);
    },
    "run" => {
      #[lua_function]
      unsafe extern "C-unwind" fn run(lua: State) -> i32 {
        if lua.lua_type(1) == LUA_TUSERDATA {
          let db: &Database = (lua.to_userdata(1) as *const Database).as_ref().unwrap();
          let command: Document = match bson::from_lua(lua, 2) {
            Ok(doc) => match doc.as_document() {
              Some(doc) => doc.clone(),
              None => return raise_error(lua, "Invalid command".to_string())
            },
            Err(e) => return raise_error(lua, format!("{}", e))
          };

          match db.run_command(command, None) {
            Ok(doc) => {
              bson::to_lua(lua, Bson::Document(doc));
            },
            Err(e) => return raise_error(lua, format!("{}", e))
          }

          1
        } else {
          raise_error(lua, "Invalid database".to_string())
        }
      }
      lua.push_closure(run, 0);
    }
    _ => {
      let cols = match db.list_collection_names(None) {
        Ok(cols) => cols,
        Err(e) => return raise_error(lua, format!("{}", e))
      };

      if cols.contains(&field) {
        collection::mongodb_collection_new(db.clone(), lua, Some(&field));
      } else {
        return raise_error(lua, "Invalid field name".to_string());
      }
    }
  }

  1
}

#[lua_function]
unsafe fn mongodb_database_tostring(lua: State) -> i32 {
  let db = (lua.to_userdata(1) as *const Database).as_ref().unwrap();
  lua.push_string(&format!("MongoDB::Database({})", db.name()));
  1
}

pub unsafe fn mongodb_database_new(client: Client, lua: State, name: Option<&str>) {
  let db = match name {
    Some(name) => client.database(name),
    None => client.database(&lua.check_string(2).to_string())
  };
  lua.new_table();

  lua.push_function(mongodb_database_index);
  lua.set_field(-2, lua_string!("__index"));

  lua.push_function(mongodb_database_tostring);
  lua.set_field(-2, lua_string!("__tostring"));

  lua.new_userdata(db, Some(-1));
}