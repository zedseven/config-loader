# Changelog
This only lists versions that have [Releases](https://github.com/zedseven/config-loader/releases) created for them.
In future, version numbers will be updated more conservatively.

## [v2.2.3](https://github.com/zedseven/config-loader/compare/v1.1.2...v2.2.3) - 2021-12-15
- Added support for directory symlinks
- Made the program output the version number on startup
- Added loadout ancestry
  - This allows multiple loadouts to be chained or parented together, so multiple loadouts that all need the same files
  can use a single parent
- Improved the CLI `colour` argument
- Added Windows and Unix-specific starter config files
  - The Windows one uses backslashes and CRLF line-endings, while the Unix one uses forward slashes and LF line-endings
- Updated dependencies
- Changed references to the word `master` because it wasn't really necessary

## [v1.1.2](https://github.com/zedseven/config-loader/compare/v1.1.0...v1.1.2) - 2021-11-09
- Fixed number padding, so it isn't always padded by an extra space

## [v1.1.0](https://github.com/zedseven/config-loader/compare/v1.0.0...v1.1.0) - 2021-11-07
- Added terminal ANSI colouring
- Made `fuzzy_search` default behaviour and removed the CLI option
- Added status badges to `README.md`
- Improved the example config slightly
- Added a demo gif to `README.md`
- Added additional metadata to `Cargo.toml`

## [v1.0.0](https://github.com/zedseven/config-loader/compare/7f940684ce2b1613679333497200c91363434c6f...v1.0.0) - 2021-10-30
Initial release.
