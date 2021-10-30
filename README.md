# config-loader
A tool for quickly switching between different file configurations, using
symbolic links.

## Usage
To use it, download the latest release for your platform from the
[Releases tab](https://github.com/zedseven/config-loader/releases).

Simply run the program, and upon first launching it will ask if you want
to create a starter config file.

Once it's created, open it and edit it to your liking. The following is an
example config:

```toml
# Config Loader - https://github.com/zedseven/config-loader

# Here's where you define the file targets. These are the files you
# have to switch out whenever you want to change to a different configuration.
# Please use absolute file paths.
[targets]
cool_picture = "F:\\Main Windows Folders\\Downloads\\CoolPicture.png"

# Here's where each loadout is defined. Include as many files for each loadout
# as you need.
# Each loadout is completely separate from the others. Each one should be a
# complete set of all the files necessary to work.
[[loadouts]]
name = "AngeryBird"
[loadouts.files]
cool_picture = "F:\\Pictures\\Pictures2\\AngeryBird.png"

[[loadouts]]
name = "Cheese"
[loadouts.files]
cool_picture = "F:\\Pictures\\Pictures2\\Cheese.png"

```

In this (fairly silly) example, the configuration simply swaps out the same
`cool_picture` file with either a picture of a bird, or a picture of
cheese - depending on the selected loadout.

Once you've set up a config file, refresh the config, and you should be good
to go.

### Note
On Windows machines,
[Administrator privileges are required](https://security.stackexchange.com/a/10198)
to make symbolic links. If you get a `A required privilege is not held by the client.`
error, that's why.

If you don't trust the program to give it Administrator privileges, the source
code is available, and you can build it from source.

This program should work on Windows and Unix (Linux, MacOS) systems without a
problem. If you encounter one, please
[open an issue](https://github.com/zedseven/config-loader/issues).

## Project License
This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or
  http://opensource.org/licenses/MIT)

at your option.

### Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in *config-loader* by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.
