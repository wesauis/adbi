use colored::*;


pub fn init() {
  std::panic::set_hook(Box::new(|msg| {
    eprintln!("{} {}", " CRASH ".black().on_red(), msg);
  }));
}

pub fn info(msg: &str) {
  println!("{} {}", " INFO ".black().on_cyan(), msg);
}

pub fn error(msg: &str) {
  println!("{} {}", " ERROR ".black().on_red(), msg);
}

