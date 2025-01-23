# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [2.0.0 Release Candidate 3] Unreleased

Changed

- Upgraded libp2p to current version 0.55.0
- Upgraded all rust libraries to newest version

## [2.0.0 Release Candidate 2] 2024-11-28

Added

- voice messages (recording is not yet possible on Linux)

Fixed

- zero-terminate path buffer in dart libqaul ffi

Changed

- Upgraded libp2p to current version 0.54.1
- Upgraded all rust libraries to newest version
- Upgraded Kotlin version to 1.9.25
- Upgraded flutter to ^2.24.0

## [2.0.0 Release Candidate 1] 2024-06-08

This release can only be upgraded from qaul version `2.0.0-beta.18`.
Please upgrade to beta 18 first, before upgrading to `Release Candidate 1` or later.

Added

- Warning when trying to upgrade to this version on earlier instances of qaul 2.0.0-beta.17 and lower.
- About screen, with version information

Fixed

- Fixed a possible panic in libqaul that could occur when receiving an empty data package when receiving a file.
- Fixed UI flickering in 'Network View'.

Changed

- Upgraded libp2p to current version 0.53.2.
- Libqaul's communication protocols now have an own codec to encode and decode all messages.
  This codec is fully compatible to how qaul was sending it's messages before.
- Upgraded many libraries to new versions.
- removed old versions of unmaintained `sled` versions, which were only used for the upgrade procedure to qaul 2.0.0 beta 18.
  This means that upgrading to qaul version 2.0.0 release candidate 1 is only possible from qaul version 2.0.0 beta 18.
- Configuration auto-upgrade of all static TCP peer entries to use the QUIC protocol for interconnections.

## [2.0.0 beta 18] 2024-04-11

This release moves away from the unmaintained `sled-extensions` data base extensions to a direct use of the `sled` data base.

Added

- Automatically generate SBOM files for all binaries.
- UI redesign for simple adding of static links (Community Nodes) with many options: QUIC/TCP/IPv4/IPv6

Fixed

- UI: block user issue fixed
- UI: Network view upgraded & fixed not showing all connections
- Fixed a crash on Android, when BLE failed to initialize on a device (thanks to @link2xt)

Changed

- migrated from sled 0.29 used by sled-extensions to current sled 0.34.7.
- upgraded many libraries
- made `quic` the default transport option for static links

## [2.0.0 beta 17] 2023-11-26

This release upgrades libp2p to 0.52.4 and includes the new `quic` transport protocol option.

Added

- added `quic` transport option.
- CI builds arm64 CLI binaries for ARM linux, e.g. for Raspberry Pi.
- CI builds arm64 linux snap app.

Fixed

- UI group creation issue on Android.
- fixed various small bugs

Changed

- upgraded libp2p to 0.52.4
- upgraded rust dalek crypto to new major version.
- upgraded all rust libraries to current version.
- upgraded flutter to 3.16
- upgraded all flutter libraries to current version.
- removed mplex stream multiplexer from TCP transport protocol. TCP transport is always using yamux now.

## [2.0.0 beta 16] 2023-08-01

This release newly includes the android BLE-module. The Android BLE module is still experimental.
The Android BLE module was tested on Android versions 13, 12, 11 & 10.

Added

- Android: BLE-module
- UI: Automatically open Group Chat after it is created.
- Added Raspberry 64 bit build documentation and build-script for libqaul

Fixed

- Fixed a crypto session issue, when two users started the a session parallel in DTN mode.
  This led to the users not being able to communicate with each other anymore.
- Fixed many things related to the BLE-Module, BLE-Manager, BLE connection.
- Fixed a layout issue on the support screen, to allow multiple lines for some text.
- Fixed a GUI issue with public messages, having a sending timestamp in the future.
- Fixed a libqaul group DB index issue, where the first entry sometimes did not start at 0.
- libqaul RPC now returns the session ID for group requests.

Changed

- Android: Changed to a new background execution mode, which fixed known issues with background execution
- Upgraded to Flutter 3.10

## [2.0.0 beta 15] 2023-05-01

This release has breaking changes in the configuration file.

- The listening string became a string array on both, the LAN & Internet configuration:
  - old: `listen: /ip4/0.0.0.0/tcp/0`
  - new: `listen: [/ip4/0.0.0.0/tcp/0, /ip6/::/tcp/0]`

The qaul upgrade process will take care of the automated forward update.
If you want to roll back to an older version, you must roll back the `config.yaml` manually.

Added

- Android: app keeps running in the Background.
  - This release requires the Android permission to always run in the background.
- Run libqaul IPv4 & IPv6 dual stack by default.
  - LAN & INTERNET mode are now listening for incoming connections on IPv4 and IPv6
- Amount of listening interfaces is freely configurable in the configuration file.
  - This feature mainly targets the qauld community nodes, which can configure 0-n interfaces per module.
- Show user online status in direct chatroom.

Fixed

- Chat menu options sometimes misleading or not present.
- Fixed several documentation mistakes.

## [2.0.0 beta 14] 2023-03-20

Added

- Italian Translation

Fixed

- Chinese & German translation
- Diverse Snap problems
- Linux GUI icon problems in nav bar

## [2.0.0 beta 13] 2023-02-05

Added

- Spanish translation
- qauld docker image with docker-compose management
- iOS testflight listing

Fixed

- Snap: ultimately fixed: rights blockings, libqaul compilation for snap store, access rights, etc.
- UI:
  - default locale on startup screen

Changed

- better translation strings for translators
- upgraded to stable flutter version 3.7

## [2.0.0 beta 12] 2022-12-13

Fixed

- Android: build android library for release, fixed a bug that was not building for release.
- Snap: Set correct storage path to where the app has permission to store.

## [2.0.0 beta 11] 2022-12-01

Added

- French translation
- German translation

Changed

- many UI changes & fixes
- upgraded libp2p to version 0.50
- build android library for release
- Window app name title fixed

## [2.0.0 beta 10] 2022-11-24

Added

- Arabic translation
- Russian translation

Changed

- Libqaul only contains parts of libp2p.
- Made public messages selectable.
- fixed a lag of the UI when running qaul for a longer time.
- Many UI updates.

## [2.0.0 beta 9] 2022-11-17

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
