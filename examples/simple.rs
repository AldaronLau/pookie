fn main() {
    env_logger::init();

    let cookies = pookie::chrome(Some(vec!["google.com".to_string()])).unwrap();

    for cookie in cookies {
        println!("{cookie:?}");
    }
}
