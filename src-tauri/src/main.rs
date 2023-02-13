use rusqlite::{params, NO_PARAMS, Connection, Result};

fn print_typename<T>(_: T) {
  println!("{}", std::any::type_name::<T>());
}

pub fn init_db() -> Result<()>{
  let path = "./ropes.db";
  let con = Connection::open(&path)?;
  con.execute(
      "create table if not exists posts (
           id integer primary key,
           title text,
           content text
       )",
      NO_PARAMS,
  )?;
  println!("{}", "created");
  Ok(())
}

fn main() -> Result<()>{
  
  init_db();
  
  tauri::Builder::default()
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
  Ok(())
}
