fn main() {
  tracing_subscriber::fmt::init();
  let cookies = pookie::chrome(None).unwrap();
  for cookie in cookies {
    println!("{:?}", cookie);
  }
}
