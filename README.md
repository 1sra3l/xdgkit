# XDGkit

The ultimate XDG library and command line tool!
Everything is copy-pasted from (freedesktop.org)[http://freedesktop.org] and rustified as enums and structs with implementations in main for a binary tool to use the specs!

This work could technically regenerate the website documentation via doxygen because Rust is like that.  But I didn't actually do anything to make it possible, though the code is stupidly simple... really mostly Ctrl+C in firefox and Ctrl+V in Kate.


I had to make up some enums for things like `Type` in the desktop_entry format.


`CamelCase` is converted consistently as `camel_case`, as you would expect knowing rust's compiler from compiling once differently.

### Here are the NON-INTUITIVE things

DesktopEntry Spec:
`Type` is `desktop_type` which is quickly intuitive WHY, since `type` is a reserved word.

## basedir
This uses std::env and returns Result<String, VarError> as does std::env
This provides all the normal XDG variables, as well as locations for icons, menu/directory files, desktop files, and the autostart directories

## desktop_entry
This reads in a desktop file and turns it into a struct which can be accessed for any of the desktop file features you will find in the freedesktop spec.

# WORKS IN PROGRESS

## desktop_menu
This reads the menu file and generates a struct containing the entire menu which can be fed into another program to output it into a specific format.

## directory_file
This reads the directory files used by the desktop_menu

## icon_theme
This reads the icon index.theme file and can return file paths for each icon name requested.

