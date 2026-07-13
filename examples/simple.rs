fn main() {
    tracing_subscriber::fmt::init();
    
    let cookies = pookie::firefox(None).unwrap();

    for cookie in cookies {
        println!("{cookie:?}");
    }
}
