# CopyClip

# show whether tests are passing or not (from GitHub workflows)
[![macOS](https://github.com/trvswgnr/copyclip/actions/workflows/test.macos.yml/badge.svg)](https://github.com/trvswgnr/copyclip/actions/workflows/test.macos.yml)
[![Windows](https://github.com/trvswgnr/copyclip/actions/workflows/test.windows.yml/badge.svg)](
  https://github.com/trvswgnr/copyclip/actions/workflows/test.windows.yml
)
[![Linux](
  https://github.com/trvswgnr/copyclip/actions/workflows/test.linux.yml/badge.svg
)](
  https://github.com/trvswgnr/copyclip/actions/workflows/test.linux.yml
)

A super simple CLI tool for piping text to the clipboard, written in Rust.

## Installation

- Unix:
  Run the Unix install script:

  ```sh
  curl -sL https://raw.githubusercontent.com/trvswgnr/copyclip/main/install-unix.sh | $SHELL
  ```
- Windows:
  Run the Windows install script:

  ```powershell
  iwr https://raw.githubusercontent.com/trvswgnr/copyclip/main/install-windows.ps1 -useb | iex
  ```
