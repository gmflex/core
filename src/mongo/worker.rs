use std::{
  sync::{Arc, Barrier},
  cell::Cell
};
use gmod::lua::{State, LuaReference};

thread_local! {
  static PENDING: Cell<usize> = Cell::new(0);
}

pub struct MongoDBCommand {

}

pub fn send(lua: State, cmd: MongoDBCommand) {
  PENDING.with(|pending| {
    pending.set(pending.get() + 1);
  });

  WORKER_CHANNEL.get()
                .send(cmd)
                .expect("Worker channel hung up - this is a bug with gmsv_reqwest");
  
  unsafe {
    lua.get_global(lua_string!("timer"));
		lua.get_field(-1, lua_string!("Create"));
		lua.push_string("mongodb_worker_response");
		lua.push_integer(0);
		lua.push_integer(0);
		lua.push_function(think);
		lua.call(4, 0);
		lua.pop();
  }
}