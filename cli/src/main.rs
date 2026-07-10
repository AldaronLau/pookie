use clap::Parser;
use pookie::{any_browser, common::enums::Cookie};
mod browsers_map;
use browsers_map::BROWSERS_MAP;
mod args;
use args::Args;
use pookie::common::format;

fn print_cookies(args: Args, cookies: Vec<Cookie>) {
    match args.format.as_str() {
        "json" => {
            let str = format::json(cookies);
            println!("{str}");
        }
        "netscape" => {
            let data = format::netscape(cookies);
            println!("{}", data);
        }
        _ => {}
    }
}

fn print_version() {
    println!(
        "CLI: {}\nRookie: {}",
        env!("CARGO_PKG_VERSION"),
        pookie::version()
    );
    std::process::exit(0);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    let args = Args::parse();
    if args.version {
        print_version();
    }
    #[allow(unused_assignments)]
    let mut cookies = vec![];
    let args_c = args.clone();
    if args.load {
        cookies = pookie::load(args.domains)?;
    } else if let Some(browser) = args.browser {
        let browser_fn = BROWSERS_MAP.get(&browser).unwrap();
        cookies = browser_fn(args.domains)?;
    } else if let Some(path) = args.path {
        cookies =
            any_browser(path.as_str(), args.domains, args.key_path.as_deref())?;
    } else {
        // Default load from all
        cookies = pookie::load(args.domains)?;
    }
    print_cookies(args_c, cookies);

    Ok(())
}
