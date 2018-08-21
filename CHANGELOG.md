# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog] and this project adheres to
[Semantic Versioning].

[Keep a Changelog]: http://keepachangelog.com/en/1.0.0/
[Semantic Versioning]: http://semver.org/spec/v2.0.0.html

## [0.6.2] - 2018-08-21

### Added
- Feature `system` probes for system libffi instead of downloading and
  bulding our own.

## [0.6.1] - 2018-05-30

### Added
- Setting `doc(html_root_url)` for inter-crate docs linking.
- Testing on Rust 1.20.0 now, as oldest supported version.
- Better message when bindgen fails.

### [0.6.0] - 2017-11-13

### Changed
- Upgraded to `bindgen` 0.31.3.

### [0.5.4] - 2017-11-12

### Changed
- Calling `bindgen` with `blacklist_type` rather than `hide_type`, as the 
latter is deprecated. (Thanks, fitzgen.)

### [0.5.3] - 2017-07-07

### Added
- `lib64/` now in library search path.
- Build instructions now mention C libffi and texinfo.

### [0.5.2] - 2017-04-14

### Fixed
- Avoid some unnecessary C libffi rebuilds. (Thanks, ngkz.)
- Avoids link error on Arch Linux by building C libffi `--withpic`.

### Changed
- Links against a self-build static C libffi rather than dynamic. (Thanks, 
  ngkz.)

### [0.5.0] - 2017-03-02

### Removed
- No longer passing `--disable-docs` to `configure` for C libffi.

## [0.4.7] - 2017-03-01

### Changed
- Hiding `max_align_t` struct in `stddef.h` from bindgen, because it was 
  confusing it.
- Upgraded bindgen (0.22). (Thanks, cholcombe973.)
  
### Added

- Bindgen now generates default impls. (Thanks, cholcombe973.)

## [0.4.6] - 2016-08-29

### Changed
- Upgraded bindgen (0.18).

## [0.4.4] - 2016-06-21

### Changed
- Builds C libffi from a Git submodule. (Thanks, murarth.)

## [0.4.3] - 2016-06-21

### Changed
- Builds dynamic C libffi.

## [0.4.2] - 2016-06-20

### Changed
- Fetching C libffi from a cached copy on my website, because fetching it 
  from ftp is unreliable.
  
## [0.4.1] - 2016-06-20

### Changed
- Using `-lffi` instead of `-llibffi`.

## [0.4.0] - 2016-06-20

### Added
- Fetches and builds its own C libffi now.

## [0.3.4] - 2016-06-17

### Changed
- Updated `clang-sys` version.

## [0.3.3] - 2016-06-14

### Fixed
- Crate name in instructions.

## [0.3.2] - 2016-06-14

### Added
- Better error messages from `build.rs`.
- Clarified dependencies in docs.

## [0.3.0] - 2016-06-14

Split from `libffi` crate.

