use gmod::lua::{
  State,
  LUA_TUSERDATA, LUA_TTABLE
};
use mongodb::{
  sync::{Client, Database, Collection as MongoCollection},
  error::Error as MongoError,
  bson::{Bson, Document},
};
use crate::{
  mongo::raise_error,
  bson
};

type Collection = MongoCollection<Document>;

#[lua_function]
unsafe fn mongodb_collection_index(lua: State) -> i32 {
  let field = match lua.get_string(2) {
    Some(str) => str,
    None => return raise_error(lua, "Invalid field name".to_string())
  };

  match field.as_ref() {
    "name" => {
      #[lua_function]
      unsafe extern "C-unwind" fn name(lua: State) -> i32 {
        if lua.lua_type(1) == LUA_TUSERDATA {
          let col: &Collection = (lua.to_userdata(1) as *const Collection).as_ref().unwrap();
          lua.push_string(col.name());
          1
        } else {
          raise_error(lua, "Invalid collection".to_string())
        }
      }
      lua.push_closure(name, 0);
    },
    "namespace" => {
      #[lua_function]
      unsafe extern "C-unwind" fn namespace(lua: State) -> i32 {
        if lua.lua_type(1) == LUA_TUSERDATA {
          let col: &Collection = (lua.to_userdata(1) as *const Collection).as_ref().unwrap();
          let ns = col.namespace();
          lua.push_string(&format!("{}.{}", ns.db, ns.coll));
          1
        } else {
          raise_error(lua, "Invalid collection".to_string())
        }
      }
      lua.push_closure(namespace, 0);
    },
    "count" => {
      #[lua_function]
      unsafe extern "C-unwind" fn count(lua: State) -> i32 {
        if lua.lua_type(1) == LUA_TUSERDATA {
          let col: &Collection = (lua.to_userdata(1) as *const Collection).as_ref().unwrap();
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

          match col.count_documents(filter, None) {
            Ok(count) => lua.push_number(count as f64),
            Err(e) => return raise_error(lua, format!("{}", e))
          }

          1
        } else {
          raise_error(lua, "Invalid collection".to_string())
        }
      }
      lua.push_closure(count, 0);
    },
    "delete" => {
      #[lua_function]
      unsafe extern "C-unwind" fn delete(lua: State) -> i32 {
        if lua.lua_type(1) == LUA_TUSERDATA {
          let col: &Collection = (lua.to_userdata(1) as *const Collection).as_ref().unwrap();
          let query: Document = match lua.lua_type(2) {
            LUA_TTABLE => {
              match bson::from_lua(lua, 2) {
                Ok(doc) => match doc.as_document() {
                  Some(doc) => doc.clone(),
                  None => return raise_error(lua, "Invalid query".to_string())
                },
                Err(e) => return raise_error(lua, format!("{}", e))
              }
            },
            _ => return raise_error(lua, "No query".to_string())
          };

          match col.delete_one(query, None) {
            Ok(dels) => lua.push_number(dels.deleted_count as f64),
            Err(e) => return raise_error(lua, format!("{}", e))
          }

          1
        } else {
          raise_error(lua, "Invalid collection".to_string())
        }
      }
      lua.push_closure(delete, 0);
    },
    "delete_many" => {
      #[lua_function]
      unsafe extern "C-unwind" fn delete_many(lua: State) -> i32 {
        if lua.lua_type(1) == LUA_TUSERDATA {
          let col: &Collection = (lua.to_userdata(1) as *const Collection).as_ref().unwrap();
          let query: Document = match lua.lua_type(2) {
            LUA_TTABLE => {
              match bson::from_lua(lua, 2) {
                Ok(doc) => match doc.as_document() {
                  Some(doc) => doc.clone(),
                  None => return raise_error(lua, "Invalid query".to_string())
                },
                Err(e) => return raise_error(lua, format!("{}", e))
              }
            },
            _ => return raise_error(lua, "No query".to_string())
          };

          match col.delete_many(query, None) {
            Ok(dels) => lua.push_number(dels.deleted_count as f64),
            Err(e) => return raise_error(lua, format!("{}", e))
          }

          1
        } else {
          raise_error(lua, "Invalid collection".to_string())
        }
      }
      lua.push_closure(delete_many, 0);
    },
    "distinct" => {
      #[lua_function]
      unsafe extern "C-unwind" fn distinct(lua: State) -> i32 {
        if lua.lua_type(1) == LUA_TUSERDATA {
          let col: &Collection = (lua.to_userdata(1) as *const Collection).as_ref().unwrap();
          let filter: Option<Document> = match lua.lua_type(3) {
            LUA_TTABLE => {
              match bson::from_lua(lua, 3) {
                Ok(doc) => match doc.as_document() {
                  Some(doc) => Some(doc.clone()),
                  None => return raise_error(lua, "Invalid filter".to_string())
                },
                Err(e) => return raise_error(lua, format!("{}", e))
              }
            },
            _ => None
          };
          let field: String = match lua.get_string(2) {
            Some(str) => str.to_string(),
            None => return raise_error(lua, "Invalid field name".to_string())
          };

          match col.distinct(field, filter, None) {
            Ok(dist) => {
              bson::to_lua(lua, Bson::Array(dist));
              1
            },
            Err(e) => return raise_error(lua, format!("{}", e))
          }
        } else {
          raise_error(lua, "Invalid collection".to_string())
        }
      }
      lua.push_closure(distinct, 0);
    },
    "drop" => {
      #[lua_function]
      unsafe extern "C-unwind" fn drop(lua: State) -> i32 {
        if lua.lua_type(1) == LUA_TUSERDATA {
          let col: &Collection = (lua.to_userdata(1) as *const Collection).as_ref().unwrap();
          match col.drop(None) {
            Ok(_) => lua.push_boolean(true),
            Err(e) => return raise_error(lua, format!("{}", e))
          }

          1
        } else {
          raise_error(lua, "Invalid collection".to_string())
        }
      }
      lua.push_closure(drop, 0);
    },
    "find" => {
      #[lua_function]
      unsafe extern "C-unwind" fn find(lua: State) -> i32 {
        if lua.lua_type(1) == LUA_TUSERDATA {
          let col: &Collection = (lua.to_userdata(1) as *const Collection).as_ref().unwrap();
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

          let cursor = match col.find(filter, None) {
            Ok(cursor) => cursor,
            Err(e) => return raise_error(lua, format!("{}", e))
          };
          // ! РАБОТАЕТ >>>>
          let arr = cursor
            .into_iter()
            .map(|doc| Bson::Document(doc.unwrap().clone()))
            .collect::<Vec<Bson>>();
          bson::to_lua(lua, Bson::Array(arr));
          1
        } else {
          raise_error(lua, "Invalid collection".to_string())
        }
      }
      lua.push_closure(find, 0);
    },
    "aggregate" => {
      #[lua_function]
      unsafe extern "C-unwind" fn aggregate(lua: State) -> i32 {
        if lua.lua_type(1) == LUA_TUSERDATA {
          let col: &Collection = (lua.to_userdata(1) as *const Collection).as_ref().unwrap();
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
            _ => return raise_error(lua, "No pipeline".to_string())
          };

          let cursor = match col.aggregate(pipeline, None) {
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
          raise_error(lua, "Invalid collection".to_string())
        }
      }
      lua.push_closure(aggregate, 0);
    },
    "find_one" => {
      #[lua_function]
      unsafe extern "C-unwind" fn find_one(lua: State) -> i32 {
        if lua.lua_type(1) == LUA_TUSERDATA {
          let col: &Collection = (lua.to_userdata(1) as *const Collection).as_ref().unwrap();
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

          match col.find_one(filter, None) {
            Ok(doc) => {
              if let Some(doc) = doc {
                bson::to_lua(lua, Bson::Document(doc.clone()));
              } else {
                lua.push_boolean(false);
              }
              1
            },
            Err(e) => return raise_error(lua, format!("{}", e))
          }
        } else {
          raise_error(lua, "Invalid collection".to_string())
        }
      }
      lua.push_closure(find_one, 0);
    },
    "find_one_and_delete" => {
      #[lua_function]
      unsafe extern "C-unwind" fn find_one_and_delete(lua: State) -> i32 {
        if lua.lua_type(1) == LUA_TUSERDATA {
          let col: &Collection = (lua.to_userdata(1) as *const Collection).as_ref().unwrap();
          let filter: Document = match lua.lua_type(2) {
            LUA_TTABLE => {
              match bson::from_lua(lua, 2) {
                Ok(doc) => match doc.as_document() {
                  Some(doc) => doc.clone(),
                  None => return raise_error(lua, "Invalid filter".to_string())
                },
                Err(e) => return raise_error(lua, format!("{}", e))
              }
            },
            _ => return raise_error(lua, "No filter".to_string())
          };

          match col.find_one_and_delete(filter, None) {
            Ok(doc) => {
              if let Some(doc) = doc {
                bson::to_lua(lua, Bson::Document(doc.clone()));
              } else {
                lua.push_boolean(false);
              }
              1
            },
            Err(e) => return raise_error(lua, format!("{}", e))
          }
        } else {
          raise_error(lua, "Invalid collection".to_string())
        }
      }
      lua.push_closure(find_one_and_delete, 0);
    },
    "find_one_and_replace" => {
      #[lua_function]
      unsafe extern "C-unwind" fn find_one_and_replace(lua: State) -> i32 {
        if lua.lua_type(1) == LUA_TUSERDATA {
          let col: &Collection = (lua.to_userdata(1) as *const Collection).as_ref().unwrap();
          let replacement: Document = match lua.lua_type(3) {
            LUA_TTABLE => {
              match bson::from_lua(lua, 3) {
                Ok(doc) => match doc.as_document() {
                  Some(doc) => doc.clone(),
                  None => return raise_error(lua, "Invalid replacement".to_string())
                },
                Err(e) => return raise_error(lua, format!("{}", e))
              }
            },
            _ => return raise_error(lua, "No replacement".to_string())
          };
          let filter: Document = match lua.lua_type(2) {
            LUA_TTABLE => {
              match bson::from_lua(lua, 2) {
                Ok(doc) => match doc.as_document() {
                  Some(doc) => doc.clone(),
                  None => return raise_error(lua, "Invalid filter".to_string())
                },
                Err(e) => return raise_error(lua, format!("{}", e))
              }
            },
            _ => return raise_error(lua, "No filter".to_string())
          };

          match col.find_one_and_replace(filter, replacement, None) {
            Ok(doc) => {
              if let Some(doc) = doc {
                bson::to_lua(lua, Bson::Document(doc.clone()));
              } else {
                lua.push_boolean(false);
              }
              1
            },
            Err(e) => return raise_error(lua, format!("{}", e))
          }
        } else {
          raise_error(lua, "Invalid collection".to_string())
        }
      }
      lua.push_closure(find_one_and_replace, 0);
    },
    "find_one_and_update" => {
      #[lua_function]
      unsafe extern "C-unwind" fn find_one_and_update(lua: State) -> i32 {
        if lua.lua_type(1) == LUA_TUSERDATA {
          let col: &Collection = (lua.to_userdata(1) as *const Collection).as_ref().unwrap();
          let update: Document = match lua.lua_type(3) {
            LUA_TTABLE => {
              match bson::from_lua(lua, 2) {
                Ok(doc) => match doc.as_document() {
                  Some(doc) => doc.clone(),
                  None => return raise_error(lua, "Invalid update".to_string())
                },
                Err(e) => return raise_error(lua, format!("{}", e))
              }
            },
            _ => return raise_error(lua, "No update".to_string())
          };
          let filter: Document = match lua.lua_type(2) {
            LUA_TTABLE => {
              match bson::from_lua(lua, 2) {
                Ok(doc) => match doc.as_document() {
                  Some(doc) => doc.clone(),
                  None => return raise_error(lua, "Invalid filter".to_string())
                },
                Err(e) => return raise_error(lua, format!("{}", e))
              }
            },
            _ => return raise_error(lua, "No filter".to_string())
          };

          match col.find_one_and_update(filter, update, None) {
            Ok(doc) => {
              if let Some(doc) = doc {
                bson::to_lua(lua, Bson::Document(doc.clone()));
              } else {
                lua.push_boolean(false);
              }
              1
            },
            Err(e) => return raise_error(lua, format!("{}", e))
          }
        } else {
          raise_error(lua, "Invalid collection".to_string())
        }
      }
      lua.push_closure(find_one_and_update, 0);
    },
    "insert" => {
      #[lua_function]
      unsafe extern "C-unwind" fn insert(lua: State) -> i32 {
        if lua.lua_type(1) == LUA_TUSERDATA {
          let col: &Collection = (lua.to_userdata(1) as *const Collection).as_ref().unwrap();
          let doc: Document = match lua.lua_type(2) {
            LUA_TTABLE => {
              match bson::from_lua(lua, 2) {
                Ok(doc) => match doc.as_document() {
                  Some(doc) => doc.clone(),
                  None => return raise_error(lua, "Invalid document".to_string())
                },
                Err(e) => return raise_error(lua, format!("{}", e))
              }
            },
            _ => return raise_error(lua, "No document".to_string())
          };

          match col.insert_one(doc, None) {
            Ok(doc) => {
              lua.push_boolean(true);
              1
            },
            Err(e) => return raise_error(lua, format!("{}", e))
          }
        } else {
          raise_error(lua, "Invalid collection".to_string())
        } 
      }
      lua.push_closure(insert, 0);
    },
    "insert_many" => {
      #[lua_function]
      unsafe extern "C-unwind" fn insert_many(lua: State) -> i32 {
        if lua.lua_type(1) == LUA_TUSERDATA {
          let col: &Collection = (lua.to_userdata(1) as *const Collection).as_ref().unwrap();
          let docs: Vec<Document> = match lua.lua_type(2) {
            LUA_TTABLE => {
              match bson::from_lua(lua, 2) {
                Ok(doc) => match doc.as_array() {
                  Some(doc) => doc.iter().map(|x| x.as_document().unwrap().clone()).collect(),
                  None => return raise_error(lua, "Invalid documents".to_string())
                },
                Err(e) => return raise_error(lua, format!("{}", e))
              }
            },
            _ => return raise_error(lua, "No documents".to_string())
          };

          match col.insert_many(docs, None) {
            Ok(doc) => {
              lua.push_boolean(true);
              1
            },
            Err(e) => return raise_error(lua, format!("{}", e))
          }
        } else {
          raise_error(lua, "Invalid collection".to_string())
        }
      }
      lua.push_closure(insert_many, 0);
    },
    "replace" => {
      #[lua_function]
      unsafe extern "C-unwind" fn replace(lua: State) -> i32 {
        if lua.lua_type(1) == LUA_TUSERDATA {
          let col: &Collection = (lua.to_userdata(1) as *const Collection).as_ref().unwrap();
          let replacement = match lua.lua_type(3) {
            LUA_TTABLE => {
              match bson::from_lua(lua, 3) {
                Ok(doc) => match doc.as_document() {
                  Some(doc) => doc.clone(),
                  None => return raise_error(lua, "Invalid replacement".to_string())
                },
                Err(e) => return raise_error(lua, format!("{}", e))
              }
            },
            _ => return raise_error(lua, "No replacement".to_string())
          };
          let query = match lua.lua_type(2) {
            LUA_TTABLE => {
              match bson::from_lua(lua, 2) {
                Ok(doc) => match doc.as_document() {
                  Some(doc) => doc.clone(),
                  None => return raise_error(lua, "Invalid query".to_string())
                },
                Err(e) => return raise_error(lua, format!("{}", e))
              }
            },
            _ => return raise_error(lua, "No query".to_string())
          };

          match col.replace_one(query, replacement, None) {
            Ok(rep) => {
              let mut doc = Document::new();
              doc.insert("matched", i64::try_from(rep.matched_count).expect("Failed to convert u64 to i64"));
              doc.insert("modified", i64::try_from(rep.modified_count).expect("Failed to convert u64 to i64"));
              if let Some(ids) = rep.upserted_id {
                doc.insert("upserted_id", ids);
              }
              bson::to_lua(lua, Bson::Document(doc));
              1
            },
            Err(e) => return raise_error(lua, format!("{}", e))
          }
        } else {
          raise_error(lua, "Invalid collection".to_string())
        }
      }
      lua.push_closure(replace, 0);
    },
    "update" => {
      #[lua_function]
      unsafe extern "C-unwind" fn update(lua: State) -> i32 {
        if lua.lua_type(1) == LUA_TUSERDATA {
          let col: &Collection = (lua.to_userdata(1) as *const Collection).as_ref().unwrap();
          let update = match lua.lua_type(3) {
            LUA_TTABLE => {
              match bson::from_lua(lua, 3) {
                Ok(doc) => match doc.as_document() {
                  Some(doc) => doc.clone(),
                  None => return raise_error(lua, "Invalid update".to_string())
                },
                Err(e) => return raise_error(lua, format!("{}", e))
              }
            },
            _ => return raise_error(lua, "No update".to_string())
          };
          let query = match lua.lua_type(2) {
            LUA_TTABLE => {
              match bson::from_lua(lua, 2) {
                Ok(doc) => match doc.as_document() {
                  Some(doc) => doc.clone(),
                  None => return raise_error(lua, "Invalid query".to_string())
                },
                Err(e) => return raise_error(lua, format!("{}", e))
              }
            },
            _ => return raise_error(lua, "No query".to_string())
          };

          match col.update_one(query, update, None) {
            Ok(rep) => {
              let mut doc = Document::new();
              doc.insert("matched", i64::try_from(rep.matched_count).expect("Failed to convert u64 to i64"));
              doc.insert("modified", i64::try_from(rep.modified_count).expect("Failed to convert u64 to i64"));
              if let Some(ids) = rep.upserted_id {
                doc.insert("upserted_id", ids);
              }
              bson::to_lua(lua, Bson::Document(doc));
              1
            },
            Err(e) => return raise_error(lua, format!("{}", e))
          }
        } else {
          raise_error(lua, "Invalid collection".to_string())
        }
      }
      lua.push_closure(update, 0);
    },
    "update_many" => {
      #[lua_function]
      unsafe extern "C-unwind" fn update_many(lua: State) -> i32 {
        if lua.lua_type(1) == LUA_TUSERDATA {
          let col: &Collection = (lua.to_userdata(1) as *const Collection).as_ref().unwrap();
          let update = match lua.lua_type(3) {
            LUA_TTABLE => {
              match bson::from_lua(lua, 3) {
                Ok(doc) => match doc.as_document() {
                  Some(doc) => doc.clone(),
                  None => return raise_error(lua, "Invalid update".to_string())
                },
                Err(e) => return raise_error(lua, format!("{}", e))
              }
            },
            _ => return raise_error(lua, "No update".to_string())
          };
          let query = match lua.lua_type(2) {
            LUA_TTABLE => {
              match bson::from_lua(lua, 2) {
                Ok(doc) => match doc.as_document() {
                  Some(doc) => doc.clone(),
                  None => return raise_error(lua, "Invalid query".to_string())
                },
                Err(e) => return raise_error(lua, format!("{}", e))
              }
            },
            _ => return raise_error(lua, "No query".to_string())
          };

          match col.update_many(query, update, None) {
            Ok(rep) => {
              let mut doc = Document::new();
              doc.insert("matched", i64::try_from(rep.matched_count).expect("Failed to convert u64 to i64"));
              doc.insert("modified", i64::try_from(rep.modified_count).expect("Failed to convert u64 to i64"));
              if let Some(ids) = rep.upserted_id {
                doc.insert("upserted_id", ids);
              }
              bson::to_lua(lua, Bson::Document(doc));
              1
            },
            Err(e) => return raise_error(lua, format!("{}", e))
          }
        } else {
          raise_error(lua, "Invalid collection".to_string())
        }
      }
      lua.push_closure(update_many, 0);
    },
    "create_index" => {
      #[lua_function]
      unsafe extern "C-unwind" fn create_index(lua: State) -> i32 {
        if lua.lua_type(1) == LUA_TUSERDATA {
          let col: &Collection = (lua.to_userdata(1) as *const Collection).as_ref().unwrap();
          let keys = match lua.lua_type(2) {
            LUA_TTABLE => {
              match bson::from_lua(lua, 2) {
                Ok(doc) => match doc.as_document() {
                  Some(doc) => Some(doc.clone()),
                  None => return raise_error(lua, "Invalid keys".to_string())
                },
                Err(e) => return raise_error(lua, format!("{}", e))
              }
            },
            _ => return raise_error(lua, "No keys".to_string())
          };
          /*let options = match lua.lua_type(3) {
            LUA_TTABLE => {
              match bson::from_lua(lua, 3) {
                Ok(doc) => match doc.as_document() {
                  Some(doc) => Some(doc.clone()),
                  None => return raise_error(lua, "Invalid options".to_string())
                },
                Err(e) => return raise_error(lua, format!("{}", e))
              }
            },
            _ => return raise_error(lua, "No options".to_string())
          };*/
          raise_error(lua, "Not implemented".to_string())
        } else {
          raise_error(lua, "Invalid collection".to_string())
        }
      }
    }
    _ => return raise_error(lua, "No field name".to_string())
  }
  1
}

#[lua_function]
unsafe fn mongodb_collection_tostring(lua: State) -> i32 {
  let col = (lua.to_userdata(1) as *const Collection).as_ref().unwrap();
  lua.push_string(&format!("MongoDB::Collection({})", col.name()));
  1
}

pub unsafe fn mongodb_collection_new(db: Database, lua: State, name: Option<&str>) {
  let col: Collection = match name {
    Some(name) => db.collection(name),
    None => db.collection(&lua.check_string(2).to_string())
  };
  lua.new_table();

  lua.push_function(mongodb_collection_index);
  lua.set_field(-2, lua_string!("__index"));

  lua.push_function(mongodb_collection_tostring);
  lua.set_field(-2, lua_string!("__tostring"));

  lua.new_userdata(col, Some(-1));
}