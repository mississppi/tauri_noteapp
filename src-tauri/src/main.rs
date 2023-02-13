use rusqlite::{params, NO_PARAMS, Connection, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
    id: u32,
    title: String,
    content: String,
}

#[tauri::command]
fn insert(title:String, content:String) {
  match insert_post(&title, &content) {
    Ok(r) => println!("{}", r),
    Err(e) => println!("{}", e),
  }
}

#[tauri::command]
fn update(id:u32, title:String, content:String) {
  match update_post(&id, &title, &content) {
    Ok(r) => println!("{}", r),
    Err(e) => println!("{}", e),
  }
}

#[tauri::command]
fn delete_all(){
  match delete_all_post() {
    Ok(r) => println!("{}", r),
    Err(e) => println!("{}", e),
  }
}

#[tauri::command]
fn list() -> Vec<Post>{
  let posts = get_posts(); //
  return posts;
}

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

pub fn create_connect() ->Result<Connection, rusqlite::Error> {
  let path = "./ropes.db3";
  let con = Connection::open(&path)?;
  println!("{}", con.is_autocommit());
  Ok(con)
}

pub fn insert_post(title:&String, content:&String) -> Result<usize,rusqlite::Error> {
  let con = create_connect().unwrap();
  return Ok(con.execute(
      "INSERT INTO posts (title, content) VALUES (?1, ?2)",
      params![title, content]
  )?);
}

pub fn update_post(id:&u32, title:&String, content:&String) -> Result<usize, rusqlite::Error>{
  let con = create_connect().unwrap();
  return Ok(con.execute(
      "update posts set title=?, content=? where id=? ",
      params![title, content, id],
  )?);
}

pub fn delete_all_post() -> Result<usize, rusqlite::Error>{
  let con = create_connect().unwrap();
  return Ok(con.execute(
      "delete from posts limit ? ",
      params![10],
  )?);
}

pub fn get_posts() -> Vec<Post>{
  let con = create_connect().unwrap();
  let mut stmt = con.prepare("select * from posts").unwrap();
  let posts = stmt.query_map(params![], |row| {
    Ok(Post {
        id: row.get(0).unwrap(),
        title: row.get(1).unwrap(),
        content: row.get(2).unwrap(),
    })
  }).unwrap();
  let mut articles: Vec<Post> = Vec::new();
  for p in posts {
    articles.push(p.unwrap());
  }
  return articles;
}

fn main() -> Result<()>{
  
  init_db();
  
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      list,
      insert,
      update,
      delete_all,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
  Ok(())
}
