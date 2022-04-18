use std::{
  env, path::{Path, PathBuf}
};

pub fn build_path(path: &[u8]) -> PathBuf {
  env::current_dir()
    .unwrap()
    .join("garrysmod")
    .join(Path::new(&String::from_utf8(path.to_vec()).unwrap()))
}