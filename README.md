# send-vrc-desktop-rs

[send_vrc_desktop](https://github.com/bootjp/send_vrc_desktop) written in Rust.

## Installation

```ps1
cargo install --git https://github.com/koyashiro/send-vrc-desktop-rs
```

## Usage

```ps1
Invoke-WebRequest -Method POST -ContentType application/json -Body '{ "url": "https://www.youtube.com/watch?v=c-ZWPYJYiAg"}' http://localhost:11400/url
```
