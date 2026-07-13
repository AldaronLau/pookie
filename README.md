# Pookie

> Fork of [rookie](https://crates.io/crates/rookie) due to the repository being
archived.

[![tests](https://github.com/AldaronLau/pookie/actions/workflows/ci.yml/badge.svg)](https://github.com/AldaronLau/pookie/actions/workflows/ci.yml)
[![GitHub commit activity](https://img.shields.io/github/commit-activity/y/AldaronLau/pookie)](https://github.com/AldaronLau/pookie)
[![GitHub contributors](https://img.shields.io/github/contributors/AldaronLau/pookie)](https://github.com/AldaronLau/pookie/graphs/contributors)  
[![Crates.io](https://img.shields.io/crates/v/pookie)](https://crates.io/crates/pookie)
[![Crates.io](https://img.shields.io/crates/d/pookie)](https://crates.io/crates/pookie)
[![Crates.io (recent)](https://img.shields.io/crates/dr/pookie)](https://crates.io/crates/pookie)  
[![Crates.io](https://img.shields.io/crates/l/pookie)](https://github.com/search?q=repo%3AAldaronLau%2Fpookie+path%3A**%2FLICENSE*&type=code)
[![Docs.rs](https://docs.rs/as_repr/badge.svg)](https://docs.rs/pookie/)

Load cookies from any browser on any platform

Check out the [documentation] for examples.

### Features

 - Cross-platform loading of cookies for many browsers
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
 - Bypass `Chrome` restriction of file locking and appbound encryption (requires
   admin rights on `Windows` from v130.x)

## Support new browsers

If you have a browser with which the library isn't working with, it may not have
been added to the list of supported browsers configs. You can create a PR or an
issue with the path to the cookies file on your computer, and I will add it.

Look at
[pookie/config.json](https://github.com/AldaronLau/pookie/blob/v0/config.json)
to see what configuration is needed.

## Testing Dates (DD/MM/YY)

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

## MSRV

The current MSRV is Rust 1.95.

Any future MSRV updates will follow the [Ardaku MSRV guidelines].

## License

Copyright © 2023-2024 thewh1teagle and others
Copyright © 2026 The Pookie crate Contributor(s)

Dual licensed under the:

 - MIT License, ([LICENSE\_MIT] or <https://mit-license.org/>)

And any of:

 - Apache License, Version 2.0, ([LICENSE\_APACHE] or
   <https://www.apache.org/licenses/LICENSE-2.0>)
 - Boost Software License, Version 1.0, ([LICENSE\_BOOST] or
   <https://www.boost.org/LICENSE_1_0.txt>)
 - Zlib License, ([LICENSE\_ZLIB] or <https://opensource.org/licenses/Zlib>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

## Credits

This project is possible because of prior art:

 - [rookie](https://github.com/thewh1teagle/rookie)
 - [browser\_cookie3](https://github.com/borisbabic/browser_cookie3)

## AI / LLM (Non-)Usage

This project (since forking from rookie) was developed without using AI tooling.
AI usage for contributions is strictly banned, with the exception of AI tooling
trained on exclusively CC0, Unlicense, or equivalently-licensed code.  By
opening a PR on this repository, you assert that the work (excluding any test
data) is either human generated, computer generated trained on works in the
public domain, or a combination of the two.  AI bug reports are welcome as
issues, as long as there is a disclaimer that it was discovered with AI.

## Help

If you want help using or contributing to this library, feel free to send me an
email at <aldaronlau@gmail.com>.

[Ardaku MSRV guidelines]: https://github.com/ardaku/.github/blob/v1/profile/MSRV.md
[LICENSE\_APACHE]: https://github.com/AldaronLau/pookie/blob/v0/LICENSE_APACHE
[LICENSE\_BOOST]: https://github.com/AldaronLau/as_repr/blob/v0/LICENSE_BOOST
[LICENSE\_MIT]: https://github.com/AldaronLau/as_repr/blob/v0/LICENSE_MIT
[LICENSE\_ZLIB]: https://github.com/AldaronLau/as_repr/blob/v0/LICENSE_ZLIB
[documentation]: https://docs.rs/pookie
