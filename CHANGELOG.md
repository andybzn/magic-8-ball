# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.2.0] - 02024-06-13

### Changed

- Updated release profile to optimize binary size

## [1.1.0] - 02024-04-27

### Added

- Added the ability to return just the raw answer text with `-a` or `--answer-only`
- Added the ability to return the answer and question as a json file using `-j` or `--json`

### Changed

- README update to cater for the new functionality and provide installation instructions
- Replaced hand-rolled argument parsing with `clap::Parser`

## [1.0.1] - 02024-04-26

### Added

- This CHANGELOG file
- README now contains information on the project, installation, usage, contribution guidelines and licensing information
- README: Project screenshots

### Fixed

- Fixed `clippy` lint regarding wildcard imports

### Changed

- Upgraded dependencies
- Suppressed the last underline on print to console, for aesthetic purposes

## [1.0.0] - 02023-08-30

### Added

- Initial build & release of magic8
