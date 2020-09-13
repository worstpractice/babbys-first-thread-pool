#![allow(unused)]
use babbys_first_thread_pool::{
  constants::GET,
  preload::HTML,
  threadpool::ThreadPool,
};
use chrono::Utc;
use std::{
  fs::OpenOptions,
  io::{
    prelude::*,
    Result,
  },
  net::{
    TcpListener,
    TcpStream,
  },
};

fn create_timestamp() -> String {
  Utc::now().to_rfc2822() + "\n"
}

fn log_request() {
  let mut file = OpenOptions::new().write(true).append(true).open("production_grade_database.txt").unwrap();

  let timestamp = create_timestamp();

  file.write_all(timestamp.as_bytes()).unwrap();
}

fn send_response(mut connection: TcpStream, response_code: &str) {
  let page = HTML.get(response_code).unwrap();
  connection.write_all(page).unwrap();
  connection.flush().unwrap();
  log_request();
}

fn handle_request(connection: TcpStream, buffer: &[u8]) {
  println!("Request: {:#?}", String::from_utf8_lossy(&buffer[..]));

  let response_code = if buffer.starts_with(GET) { "200" } else { "404" };

  send_response(connection, response_code);
}

fn handle_connection_attempt(mut connection: TcpStream) {
  let mut buffer = [0; 1024];

  if connection.read(&mut buffer).is_ok() {
    handle_request(connection, &buffer);
  }
}

fn main() -> Result<()> {
  let pool = ThreadPool::default();

  let listener = TcpListener::bind("127.0.0.1:7878")?;

  for stream in listener.incoming() {
    if let Ok(attempted_connection) = stream {
      println!("{:#?}", attempted_connection);

      pool.execute(|| {
        handle_connection_attempt(attempted_connection);
      });
    }
  }

  Ok(())
}
