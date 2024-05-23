<div align="center">

[![Discord]][Discord-invite]
<img src="https://img.shields.io/static/v1?label=Status&message=On%20hold&color=red">

</br>
  <a href="https://github.com/polyphony-chat/polyphony">
    <img src="https://github.com/polyphony-chat/branding/blob/main/logos/polyphony-2-5-round8bit.png?raw=true" alt="The Polyphony logo. a purple, square background with rounded edges. on this background, there are four vertically stacked, white lines. The lines each resemble a sine curve, although they are all shaped a little differently." width="128" height="128">
  </a>

<h3 align="center">Polyphony</h3>

  <p align="center">
    A multi-instance, cross-platform Discord/Spacebar API-compatible chat client, written in Rust using leptos and optionally Tauri.
    <br />
    <a href="https://github.com/polyphony-chat/polyphony-web"><strong>Explore the docs »</strong></a>
    <br />
    <br />
    <a href="https://github.com/polyphony-chat/polyphony-web/issues">Report Bug</a>
    ·
    <a href="https://github.com/polyphony-chat/polyphony-web/issues">Request Feature</a>
    ·
    <a href="https://discord.gg/8tKSC8wzDq">Join Discord</a>
  </p>
</div>

</div>

![Mockup of the 'Polyphony' client, set in dark purple hues. The bottom navigation bar displays connection to 2 instances, user '@Flori#7676' status on 'spacebar.chat', green 'online' status symbol, a 'coding' status, and 'Visual Studio code' playing. A left-side guild navigator hosts gray circles for guild previews, with one selected guild marked by a purple dot. A card-like guild preview on the right showcases the chosen 'Polyphony' guild, marked 'public' and 'verified', on a purple-blue gradient background. Beneath, a channel preview lists two categories, each with three channels coded by icons: 'welcome' (hashtag), 'announcements' (megaphone), and additional 'welcome' channels (hashtags). A centered search bar features the text 'search for anything' and an icon with three dots. At the bottom, a text input box labeled 'say something' is flanked by a paperclip and three-dots icons.](https://github.com/polyphony-chat/design/blob/main/ui/client-mockup-draft-neo.png?raw=true)
*early mockup of the client, not indicative of the current state of the code.* 

## Setting up a dev environment

This repository uses Tauri 2.0.0-alpha for mobile support in the future. Tauri-generated mobile project
files can be found under `crates/polyphony-tauri/gen/android` and `/apple` for Android and iOS 
respectively.

Add wasm as a compilation target

```bash
rustup target add wasm32-unknown-unknown
```

Install `tauri-cli` and `trunk`
```bash
cargo install tauri-cli trunk --locked
```

To start the web-based application, change into the `crates/polyphony-wasm` directory and run `trunk serve`.

To start the Tauri powered Desktop-App instead, change into the `crates/polyphony-tauri` directory and run `cargo tauri dev`. 

> Note that the Tauri Dev Server will try to run and listen for the `trunk serve` Dev server. If it fails to do so (perhaps because Port `8080` is already occupied), it will not be able to start the Tauri App.

[Rust]: https://img.shields.io/badge/Rust-orange?style=plastic&logo=rust
[Rust-url]: https://www.rust-lang.org/
[build-shield]: https://img.shields.io/github/actions/workflow/status/polyphony-chat/polyphony-web/rust.yml?style=flat
[build-url]: https://github.com/polyphony-chat/polyphony-web/blob/main/.github/workflows/rust.yml
[contributors-shield]: https://img.shields.io/github/contributors/polyphony-chat/polyphony-web.svg?style=flat
[contributors-url]: https://github.com/polyphony-chat/polyphony-web/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/polyphony-chat/polyphony-web.svg?style=flat
[forks-url]: https://github.com/polyphony-chat/polyphony-web/network/members
[stars-shield]: https://img.shields.io/github/stars/polyphony-chat/polyphony-web.svg?style=flat
[stars-url]: https://github.com/polyphony-chat/polyphony-web/stargazers
[issues-shield]: https://img.shields.io/github/issues/polyphony-chat/polyphony-web.svg?style=flat
[issues-url]: https://github.com/polyphony-chat/polyphony-web/issues
[license-shield]: https://img.shields.io/github/license/polyphony-chat/polyphony-web.svg?style=f;at
[license-url]: https://github.com/polyphony-chat/polyphony-web/blob/master/LICENSE
[Discord]: https://dcbadge.vercel.app/api/server/m3FpcapGDD?style=flat
[Discord-invite]: https://discord.com/invite/m3FpcapGDD
