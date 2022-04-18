use gmod::lua::State;
pub mod util;
pub mod sync;

pub unsafe fn init_sync(lua: State, idx: i32) {
  lua.push_function(sync::write);
  lua.set_field(idx, lua_string!("sync_write"));
  
  lua.push_function(sync::read);
  lua.set_field(idx, lua_string!("sync_read"));

  lua.push_function(sync::append);
  lua.set_field(idx, lua_string!("sync_append"));

  lua.push_function(sync::exists);
  lua.set_field(idx, lua_string!("sync_exists"));

  lua.push_function(sync::copy);
  lua.set_field(idx, lua_string!("sync_copy"));

  lua.push_function(sync::rename);
  lua.set_field(idx, lua_string!("sync_rename"));

  lua.push_function(sync::remove);
  lua.set_field(idx, lua_string!("sync_remove"));

  lua.push_function(sync::create);
  lua.set_field(idx, lua_string!("sync_create"));

  lua.push_function(sync::metadata);
  lua.set_field(idx, lua_string!("sync_meta"));

  log!("[flex.rs.file.sync] initialized ✓");
}