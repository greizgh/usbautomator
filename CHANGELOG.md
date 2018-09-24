# Changelog

## Unreleased
### Fixed
- Crashing when serializing the default config file
### Changed
- use cargo package name in notification instead of hardcoded value

## [0.3.0] - 2018-09-20
### Added
- Add option to list input and block devices
- Add option to describe device properties
- Allow to match device based on property set
- Command to watch for changes and display device properties
### Changed
- Updated sample config for ergodox
### Removed
- Restriction on subsystem type

## [0.2.0] - 2018-09-03
### Fixed
- Crashing when notify service is missing
- Do not crash when command fail

## [0.1.0] - 2018-08-08
### Added
- Trigger a command on device plug
- Trigger a command on device unplug
- Notify on device plug/unplug
