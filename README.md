# Pookie

#### Load cookies from any browser on any platform

Fork of [rookie](https://crates.io/crates/rookie) due to the repository being archived.

## Features 🚀

- Super Fast, Built with `Rust`
- Bypass `Chrome` restriction of file locking and appbound encryption (requires admin rights on `Windows` from v130.x)
- Read session cookies from `Chrome` based browsers! (requires admin rights on `Windows`)
- Wide browsers support
- Cross-platform support for `Windows`, `Linux`, and `macOS`

## Usage ⚙️

## Rust

```shell
cargo add pookie
```

Create `main.rs` with the following

```rust
use pookie::brave;

fn main() {
    let domains = vec!["google.com"];
    let cookies = brave(Some(domains)).unwrap();
    for cookie in cookies {
        println!("{cookie:?}");
    }
}
```

## CLI 💻

You can use pookie as a `CLI` tool which will decrypt the cookies and print it as `JSON`  
See [cli](https://github.com/thewh1teagle/rookie/tree/main/cli) folder

## Contribute 🤝

So far the following platforms are supported:

- **Arc:** `Linux`, `macOS`, `Windows`
- **Brave:** `Linux`, `macOS`, `Windows`
- **Cachy:** `Linux`
- **Chrome:** `Linux`, `macOS`, `Windows`
- **Chromium:** `Linux`, `macOS`, `Windows`
- **Edge:** `Linux`, `macOS`, `Windows`
- **Firefox:** `Linux`, `macOS`, `Windows`
- **Internet Explorer:** `Windows`
- **LibreWolf:** `Linux`, `macOS`, `Windows`
- **Opera:** `Linux`, `macOS`, `Windows`
- **Opera GX:** `macOS`, `Windows`
- **Safari:** `macOS`
- **Vivaldi:** `Linux`, `macOS`, `Windows`
- **Zen:** `Linux`, `macOS`, `Windows`

You are welcome to contribute support for other browsers, or other platforms.

## Support new browsers 🌐

If you have a browser with which the library isn't working with, it may not have been added to the list of supported browsers configs. You can create a pull request (PR) or an issue with the path to the cookies file on your computer, and I will add it.

look at [rookie-rs/config.json](rookie-rs/config.json) to see what configurations is needed.

## Testing Dates (DD/MM/YY) 📅

| Browser   |   Linux    |   macOS    |  Windows   |
| :-------- | :--------: | :--------: | :--------: |
| Arc       | 2024/08/07 | 2024/08/07 | 2024/08/07 |
| Brave     | 2024/10/26 | 2024/10/26 | 2024/10/26 |
| Cachy     | 2024/06/04 |    N/A     |    N/A     |
| Chromium  | 2024/10/26 | 2024/10/26 | 2024/03/16 |
| Chrome    | 2024/10/26 | 2024/10/26 | 2024/03/16 |
| Edge      | 2023/10/01 | 2024/08/07 | 2024/03/16 |
| Firefox   | 2024/10/26 | 2023/11/25 | 2024/03/16 |
| IE        |    N/A     |    N/A     | 2024/03/16 |
| LibreWolf | 2023/10/01 | 2023/11/25 | 2023/10/01 |
| Opera     | 2023/10/01 |     -      | 2023/10/01 |
| Opera GX  |    N/A     |     -      | 2023/10/01 |
| Safari    |    N/A     | 2024/10/26 |    N/A     |
| Vivaldi   | 2023/10/01 | 2023/11/25 | 2023/10/01 |
| Zen       |     -      | 2024/10/26 |     -      |

## Credits 🙌

This project is possible because of prior art:

 - [rookie](https://github.com/thewh1teagle/rookie)
 - [browser_cookie3](https://github.com/borisbabic/browser_cookie3)
