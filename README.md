# xdgkit

## XDGkit
[![Documentation](https://docs.rs/xdgkit/badge.svg)](https://docs.rs/xdgkit)
[![Crates.io](https://img.shields.io/crates/v/xdgkit.svg)](https://crates.io/crates/xdgkit)


The XDG library and command line tool kit!

Much has been copy-pasted from [freedesktop.org](http://freedesktop.org) and rustified as enums and structs with implementations in main for a binary tool to use the specs.

xdgkit follows SemVer

For Example:
 * 0.0.1 was the initial release
 * 0.1.0 saw the addition of `icon-theme` to the CLI subcommands
 * 0.2.0 saw the addition of `desktop-menu` to the CLI subcommands
 * 2.0.0 saw a breaking change: renaming libxdgkit to xdgkit


 ## basedir

This uses `std::env` and returns` Result<String, VarError>` as does `std::env`
This provides all the normal XDG variables, as well as locations for icons, menu/directory files, desktop files, and the autostart directories
The command line parser will automatically check for existing directories.
The functions that have `Vec` like properties (applications directory for example) can all be easily expanded
```rust
// simple use
let app_dirs:Vec<String> = convert_to_vec(applications());
```
This will return an empty vector with an empty string if nothing exists

** this implements `autostart-spec`, `basedir-spec`, and `trash-spec`** from the [XDG specifications](https://specifications.freedesktop.org/)

### desktop-entry

Reads in a desktop file and turns it into a struct which can be accessed for any of the desktop file features you will find in the freedesktop spec.

As a library this returns a struct of mostly `Option<whatever>`

As a CLI utility it returns a String printed on a new line (or a blank line if the field is empty that you are looking for. In other words, you will need something like:

### icon_finder

Based off of the psuedo code on freedesktop.org
```rust
use xdgkit::icon_finder;

let icon_name = "firefox";
// look for the 48px icon
let icon = match icon_finder::find_icon(icon_name.to_string(),48,1){
    Some(icon)=>icon,
    None=>PathBuf::new(),
};
// this will show the path to the icon in the current theme
println!("Firefox icon:{:?}", icon);
```


### icon_theme/icon-theme

Reads an `index.theme` ini-style file and turns it into a struct of `Option<whatever>` which can be accessed for any of the icon theme spec features you will find in the freedesktop spec, or the documentation of this library/program.

As a CLI utility it returns a String printed on a new line (or a blank line if the field is empty that you are looking for.

## WORKS IN PROGRESS

serde does not work for any xml library currently, due to "repeated, out of order elements", which is required for most xml files.  This will not be fixed in `serde-xml-rs`, so who knows if there will be a rust serde xml library that supports serde and xml and rust....
This effects:

 * recently_used
 * desktop_menu/desktop-menu
 * user_places


Until then, I may get around to manually reading it all...

