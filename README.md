# CopyClip

[![Test & Build](https://github.com/trvswgnr/copyclip/actions/workflows/test-all.yml/badge.svg?branch=main)](https://github.com/trvswgnr/copyclip/actions/workflows/test-all.yml)

A super simple CLI tool for piping text to the clipboard, written in Rust.

## Installation

1. Make sure Cargo is [installed](https://doc.rust-lang.org/cargo/getting-started/installation.html) on your system:

    ```bash
    cargo --version
    ```

1. Run `cargo install --git https://github.com/trvswgnr/copyclip` to install the latest version of CopyClip.

## Usage

CopyClip is a simple CLI tool that takes text from stdin and copies it to the clipboard. It's intended to be used in a pipeline, like so:

```bash
echo "Hello, world!" | copyclip
```

Then you can paste the text from the clipboard wherever you like.
