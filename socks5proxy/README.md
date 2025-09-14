## README

A simple SOCKS5 proxy server implemented in Rust.

### Usage

```shell
socks5proxy -l 127.0.0.1:8080
curl --socks5-hostname 127.0.0.1:8080 https://www.baidu.com
```

### Dependencies

This project uses the following crates:
- [argparse](https://github.com/tailhook/rust-argparse) - Command-line argument parsing
- [byteorder](https://docs.rs/byteorder) - Library for reading/writing numbers in big-endian and little-endian formats
