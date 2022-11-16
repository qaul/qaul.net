# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [2.0.0 beta 9] Unreleased

Added

- Internet Nodes now have an optional alias name.
- New upgrade module to automatically migrate versions with incompatible configuration or data base structures.
- All chat files display screen.
- New Mainland Chinese user interface translation.
- New Taiwanese Chinese user interface translation.

Changed

- Upgraded some rust libraries to newest versions.
- Many UI updates.

## [2.0.0 beta 8] 2022-11-10

Changed

- GUI: Many updates.
- Libqaul: Upgraded deprecated ping & identify functions

Fixed

- Fixed chat image loading bug.

## [2.0.0 beta 7] 2022-11-5

Changed

- Upgraded rust-libp2p to version 0.49.0
- Many GUI updates.
- Snap package on grade stable.

Fixed

- Fixed group chat rendering panic
- Fixed storage user adding widget
- Windows UI is now using the entire window.
