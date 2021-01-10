use crate::constants::RESPONSE_CODES;
use lazy_static::lazy_static;
use std::{
  collections::HashMap,
  fs,
};

lazy_static! {
  pub static ref HTML: HashMap<&'static str, Vec<u8>> = {
    let mut map = HashMap::with_capacity(2);

    #[allow(unused_doc_comments)]
    ///   _________________
    /// < HERE BE DRAGONS! >
    ///   ----------------
    ///       \                    / \  //\
    ///        \    |\___/|      /   \//  \\
    ///             /0  0  \__  /    //  | \ \
    ///            /     /  \/_/    //   |  \  \
    ///            @_^_@'/   \/_   //    |   \   \
    ///            //_^_/     \/_ //     |    \    \
    ///         ( //) |        \///      |     \     \
    ///       ( / /) _|_ /   )  //       |      \     _\
    ///     ( // /) '/,_ _ _/  ( ; -.    |    _ _\.-~        .-~~~^-.
    ///   (( / / )) ,-{        _      `-.|.-~-.           .~         `.
    ///  (( // / ))  '/\      /                 ~-. _ .-~      .-~^-.  \
    ///  (( /// ))      `.   {            }                   /      \  \
    ///   (( / ))     .----~-.\        \-'                 .~         \  `. \^-.
    ///              ///.----..>        \             _ -~             `.  ^-`  ^-_
    ///                ///-._ _ _ _ _ _ _}^ - - - - ~                     ~-- ,.-~
    ///                                                                   /.-~

    #[allow(clippy::needless_range_loop)]
    for i in 0..RESPONSE_CODES.len() {
      let (response_code, response_message) = RESPONSE_CODES[i];

      let filename: String = response_code.to_owned() + ".html";

      let page: String = fs::read_to_string(filename).unwrap();

      let response: Vec<u8> = format!(
        "HTTP/1.1 {code} {message}\r\nContent-Length: {length}\r\n\r\n{content}",
        code = response_code,
        message = response_message,
        length = page.len(),
        content = page,
      )
      .into_bytes();

      map.insert(response_code, response);
    }

    map
  };
}
