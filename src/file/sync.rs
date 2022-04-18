use std::{
  fs, fs::File, path::{Path, PathBuf},
  ffi::OsStr
};
use gmod::lua::State;
use super::util::build_path;

// file.sync.write(path: string, content: string) -> boolean, string?
#[lua_function]
pub unsafe extern "C-unwind" fn write(lua: State) -> i32 {
  use std::io::Write;

  let path = build_path(lua.check_binary_string(1));
  if path.is_dir() {
    lua.push_boolean(false); // 1
    lua.push_string("File expected"); // 2

    return 2;
  }
  let content = lua.check_binary_string(2);

  let mut f = match File::options()
    .write(true)
    .create(true)
    .truncate(true)
    .open(path) {
      Ok(file) => file,
      Err(e) => {
        lua.push_boolean(false); // 1
        lua.push_string(&e.to_string()); // 2

        return 2;
      }
    };

  match f.write_all(content) {
    Ok(()) => {
      lua.push_boolean(true); // 1
      lua.push_nil(); // 2
    },
    Err(e) => {
      lua.push_boolean(false); // 1
      lua.push_string(&e.to_string()); // 2
    }
  }

  2
}

// file.sync.read(path: string) -> boolean, string
#[lua_function]
pub unsafe extern "C-unwind" fn read(lua: State) -> i32 {
  use std::io::Read;

  let path = build_path(lua.check_binary_string(1));
  if path.is_dir() {
    lua.push_boolean(false); // 1
    lua.push_string("File expected"); // 2

    return 2;
  }

  let mut f = match File::options()
    .read(true)
    .open(path) {
      Ok(file) => file,
      Err(e) => {
        lua.push_boolean(false); // 1
        lua.push_string(&e.to_string()); // 2

        return 2;
      }
    };
  
  let mut buf = String::new();

  match f.read_to_string(&mut buf) {
    Ok(size) => {
      lua.push_boolean(true); // 1
      lua.push_binary_string(buf.as_bytes()); // 2
    },
    Err(e) => {
      lua.push_boolean(false); // 1
      lua.push_string(&e.to_string()); // 2
    }
  }

  2
}

// file.sync.append(path: string, content: string) -> boolean, string?
#[lua_function]
pub unsafe extern "C-unwind" fn append(lua: State) -> i32 {
  use std::io::Write;

  let path = build_path(lua.check_binary_string(1));
  if path.is_dir() {
    lua.push_boolean(false); // 1
    lua.push_string("File expected"); // 2

    return 2;
  }
  let content = lua.check_binary_string(2);
  let mut f = match File::options()
    .append(true)
    .create(true)
    .open(path) {
      Ok(file) => file,
      Err(e) => {
        lua.push_boolean(false); // 1
        lua.push_string(&e.to_string()); // 2

        return 2;
      }
    };
  
  match f.write_all(content) {
    Ok(()) => {
      lua.push_boolean(true); // 1
      lua.push_nil(); // 2
    },
    Err(e) => {
      lua.push_boolean(false); // 1
      lua.push_string(&e.to_string()); // 2
    }
  }

  2
}

// file.sync.exists(path: string) -> boolean, string?
#[lua_function]
pub unsafe extern "C-unwind" fn exists(lua: State) -> i32 {
  let path = build_path(lua.check_binary_string(1));

  match fs::try_exists(path) {
    Ok(exists) => {
      lua.push_boolean(exists); // 1
      lua.push_nil(); // 2
    },
    Err(e) => {
      lua.push_boolean(false); // 1
      lua.push_string(&e.to_string()); // 2
    }
  }
  
  2
}

// file.sync.copy(from: string, to: string) -> boolean, string?
#[lua_function]
pub unsafe extern "C-unwind" fn copy(lua: State) -> i32 {
  let from = build_path(lua.check_binary_string(1));
  if from.is_dir() {
    lua.push_boolean(false); // 1
    lua.push_string("File expected"); // 2

    return 2;
  }
  let to = build_path(lua.check_binary_string(2));

  match fs::copy(from, to) {
    Ok(u64) => {
      lua.push_boolean(true); // 1
      lua.push_nil(); // 2
    },
    Err(e) => {
      lua.push_boolean(false); // 1
      lua.push_string(&e.to_string()); // 2
    }
  }

  2
}

// file.sync.rename(old: string, new: string) -> boolean, string?
#[lua_function]
pub unsafe extern "C-unwind" fn rename(lua: State) -> i32 {
  let old = build_path(lua.check_binary_string(1));
  if old.is_dir() {
    lua.push_boolean(false); // 1
    lua.push_string("File expected"); // 2

    return 2;
  }
  let new = build_path(lua.check_binary_string(2));

  match fs::rename(old, new) {
    Ok(()) => {
      lua.push_boolean(true); // 1
      lua.push_nil(); // 2
    },
    Err(e) => {
      lua.push_boolean(false); // 1
      lua.push_string(&e.to_string()); // 2
    }
  }

  2
}

// file.sync.remove_file(path: string) -> boolean, string?
#[lua_function]
pub unsafe extern "C-unwind" fn remove(lua: State) -> i32 {
  let path = build_path(lua.check_binary_string(1));
  if path.is_dir() {
    match fs::remove_dir(path) {
      Ok(()) => {
        lua.push_boolean(true); // 1
        lua.push_nil(); // 2
      },
      Err(e) => {
        lua.push_boolean(false); // 1
        lua.push_string(&e.to_string()); // 2
      }
    }
  } else {
    match fs::remove_file(path) {
      Ok(()) => {
        lua.push_boolean(true); // 1
        lua.push_nil(); // 2
      },
      Err(e) => {
        lua.push_boolean(false); // 1
        lua.push_string(&e.to_string()); // 2
      }
    }
  }

  2
}

// file.sync.create(path: string, recurisve: boolean?) -> boolean, string?
#[lua_function]
pub unsafe extern "C-unwind" fn create(lua: State) -> i32 {
  let path = build_path(lua.check_binary_string(1));
  // TODO: recursive
  if path.to_str().unwrap().ends_with("/") {
    let recursive = lua.check_boolean(2);
    match fs::create_dir(path) {
      Ok(()) => {
        lua.push_boolean(true); // 1
        lua.push_nil(); // 2
      },
      Err(e) => {
        lua.push_boolean(false); // 1
        lua.push_string(&e.to_string()); // 2
      }
    }
  } else {
    match fs::write(path, "") {
      Ok(()) => {
        lua.push_boolean(true);
        lua.push_nil();
      },
      Err(e) => {
        lua.push_boolean(false);
        lua.push_string(&e.to_string());
      }
    }
  }

  2
}

// file.sync.meta(path: string) -> table
#[lua_function]
pub unsafe extern "C-unwind" fn metadata(lua: State) -> i32 {
  let path = build_path(lua.check_binary_string(1));
  let meta = fs::metadata(path).unwrap();

  lua.new_table();

  lua.push_string(if meta.is_dir() {"dir"} else {"file"});
  lua.set_field(-2, lua_string!("type"));

  lua.push_number(meta.len() as f64);
  lua.set_field(-2, lua_string!("len"));

  lua.push_boolean(meta.permissions().readonly());
  lua.set_field(-2, lua_string!("readonly"));

  lua.push_number(meta.modified().unwrap().elapsed().unwrap().as_secs_f64());
  lua.set_field(-2, lua_string!("modified"));

  lua.push_number(meta.accessed().unwrap().elapsed().unwrap().as_secs_f64());
  lua.set_field(-2, lua_string!("accessed"));

  lua.push_number(meta.created().unwrap().elapsed().unwrap().as_secs_f64());
  lua.set_field(-2, lua_string!("created"));

  1
}