# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.0.0] - 2025-02-02

### Changes
  - Require exclusive ownership over the SPI bus through `SpiBus`
    (This is required because the driver manages the CS itself due to timing constraints)

### Added
  - Implement `core::error::Error` for error type
  - Add feature `async` for async variant of the driver

## [0.1.0] - 2024-03-13

### Added
  - HCS-12SS59T driver
    - Brightness control
    - Display strings, chars and arbitrary font table items
    - Set character generator RAM entries
    - Option to control supply voltage via dedicated pin
  - Scrolling text animation helper with `Cycle` and `LeftRight` mode

[Unreleased]: https://github.com/embediver/hcs-12ss59t-rs/tree/master
[0.1.0]: https://github.com/embediver/hcs-12ss59t-rs/tree/v0.1.0
[1.0.0]: https://github.com/embediver/hcs-12ss59t-rs/tree/v1.0.0
