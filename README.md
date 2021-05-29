# XDGkit

The **ultimate** XDG library and command line tool!
Everything is copy-pasted from (freedesktop.org)[http://freedesktop.org] and rustified as enums and structs with implementations in main for a binary tool to use the specs!

This work could technically regenerate the website documentation via doxygen because Rust is like that.  But I didn't actually do anything to make it possible, though the code is adequately simple... really mostly Ctrl+C in firefox and Ctrl+V in Kate.

xdgkit follows SemVer

For Example:
 * 0.0.1 was the initial release
 * 0.1.0 saw the addition of `icon-theme` to the CLI subcommands
 * 0.2.0 saw the addition of `desktop-menu` to the CLI subcommands
 * 1.0.0 saw a breaking change: moving `OnlyShowIn` to `DesktopEnvironment`

I had to make up some enums for things like `Type` in the desktop_entry format.

## NOTICE FOR ALL FIELDS IN STRUCTS

`CamelCase` is converted consistently as `camel_case`, as you would expect knowing rust's compiler from compiling once differently.

`Type` is `xdg_type`  which is quickly intuitive WHY, since `type` is a reserved word.  `Type` occurs in all of the ini-style configuration files in the XDG standards.

enums are generally fine as-is, however I added IconContext::Unknown
## basedir

This uses `std::env` and returns` Result<String, VarError>` as does `std::env`
This provides all the normal XDG variables, as well as locations for icons, menu/directory files, desktop files, and the autostart directories
The command line parser will automatically check for existing directories.
The functions that have `Vec` like properties (applications directory for example) can all be easily expanded
```rs
// simple use
let app_dirs:Vec<String> = convert_to_vec(applications());
```
This will return an empty vector with an empty string if nothing exists

** this implements `autostart-spec`, `basedir-spec`, and `trash-spec`** from the (XDG specifications)[https://specifications.freedesktop.org/]

## desktop-entry

Reads in a desktop file and turns it into a struct which can be accessed for any of the desktop file features you will find in the freedesktop spec.

As a library this returns a struct of mostly `Option<whatever>`

As a CLI utility it returns a String printed on a new line (or a blank line if the field is empty that you are looking for. In other words, you will need something like:

```sh
function myfun {
 local IFS="
"
 # awesome codes here
}
```

## icon_theme/icon-theme

Reads an `index.theme` ini-style file and turns it into a struct of `Option<whatever>` which can be accessed for any of the icon theme spec features you will find in the freedesktop spec, or the documentation of this library/program.

As a CLI utility it returns a String printed on a new line (or a blank line if the field is empty that you are looking for. In other words, you will need something like:

```sh
function myfun {
 local IFS="
"
 # awesome codes here
}
```

This way any script-based menu can find the correct icons for the theme

# WORKS IN PROGRESS

## desktop_menu/desktop-menu

This reads the menu file and generates a struct containing the entire menu which can be fed into another program to output it into a specific format.



