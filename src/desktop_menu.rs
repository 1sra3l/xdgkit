/*!
# WIP
This is not ready for use yet
*/
// desktop_menu.rs
// Rusified in 2021 Copyright Israel Dahl. All rights reserved.
// 
//        /VVVV\
//      /V      V\
//    /V          V\
//   /      0 0     \
//   \|\|\</\/\>/|/|/
//        \_/\_/
// 
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License version 2 as
// published by the Free Software Foundation.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
// 
// You should have received a copy of the GNU General Public License
// along with this program; if not, write to the Free Software
// Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA  02110-1301  USA

use crate::desktop_entry::*;
use crate::basedir::*;

use std::path::PathBuf;
extern crate quick_xml;
extern crate serde;
use quick_xml::de::{from_str, DeError};
//use serde_xml_rs::from_str;//, to_string};
use serde::{Deserialize, Deserializer, Serialize};

fn deserialize_ignore_any<'de, D: Deserializer<'de>>(deserializer: D) -> Result<(), D::Error> {
    serde::de::IgnoredAny::deserialize(deserializer)?;
    Ok(())
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum MergeType {
    Menus,
    Files,
    All,
    #[serde(other, deserialize_with = "deserialize_ignore_any")]
    Other,   
}
impl MergeType {
    pub fn from_string(item:String) -> MergeType{
        if item == "menus" {
            return MergeType::Menus
        } else if item == "files" {
            return MergeType::Files
        }
        MergeType::All
    }
}
impl Default for MergeType {
    fn default() -> Self {
        Self::All
    }
}
#[derive(Debug, Default, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
/*
The `<Layout>` element is an optional part of this specification. Implementations that do not support the `<Layout>` element should preserve any `<Layout>` elements and their contents as far as possible. Each `<Menu>` may optionally contain a `<Layout>` element. If multiple elements appear then only the last such element is relevant. The purpose of this element is to offer suggestions for the presentation of the menu. If a menu does not contain a `<Layout>` element or if it contains an empty `<Layout>` element then the default layout should be used. The `<Layout>` element may contain `<Filename>`, <Menuname>, <Separator> and <Merge> elements. The `<Layout>` element defines a suggested layout for the menu starting from top to bottom. References to desktop entries that are not contained in this menu as defined by the `<Include>` and `<Exclude>` elements should be ignored. References to sub-menus that are not directly contained in this menu as defined by the `<Menu>` elements should be ignored.
*/
pub struct Layout {
    pub filename:Option<String>,
    pub menuname:Option<String>,
    pub separator:Option<String>,
    pub merge:Option<Merge>,
}
/*
`type="menus"|"files"|"all"`

This element may only appear as a child of a `<Layout>` or `<DefaultLayout>` menu. It indicates the point where desktop entries and sub-menus that are not explicitly mentioned within the `<Layout>` or `<DefaultLayout>` element are to be inserted. It has a type attribute that indicates which elements should be inserted: type="menus" means that all sub-menus that are not explicitly mentioned should be inserted in alphabetical order of their visual caption at this point. `type="files"` means that all desktop entries contained in this menu that are not explicitly mentioned should be inserted in alphabetical order of their visual caption at this point. `type="all"` means that a mix of all sub-menus and all desktop entries that are not explicitly mentioned should be inserted in alphabetical order of their visual caption at this point. Each `<Layout>` or ``<DefaultLayout>`` element shall have exactly one <Merge type="all">` element or it shall have exactly one `<Merge type="files">` and exactly one `<Merge type="menus">` element. An exception is made for a completely empty `<Layout>` element which may be used to indicate that the default-layout should be used instead.
*/
#[derive(Debug, Default, Clone, Copy, Deserialize, Serialize, PartialEq)]
pub struct Merge {
    pub r#type:MergeType,
}
/*
   Any number of `<MergeFile>` elements may be listed below a `<Menu>` element, giving the name of another menu file to be merged into this one. The section called [“Merging”](https://specifications.freedesktop.org/menu-spec/latest/ar01s05.html) specifies how merging is done. The root `<Menu>` of the merged file will be merged into the immediate parent of the `<MergeFile>` element. The `<Name>` element of the root `<Menu>` of the merged file are ignored.
 
    If the type attribute is missing or set to "path" then the contents of the `<MergeFile>` element indicates the file to be merged. If this is not an absolute path then the file to be merged should be located relative to the location of the menu file that contains this `<MergeFile>` element.

    Duplicate `<MergeFile>` elements (that specify the same file) are handled as with duplicate `<AppDir>` elements (the last duplicate is used).

    If the type attribute is set to `parent` and the file that contains this `<MergeFile>` element is located under one of the paths specified by `$XDG_CONFIG_DIRS`, the contents of the element should be ignored and the remaining paths specified by `$XDG_CONFIG_DIRS` are searched for a file with the same relative filename. The first file encountered should be merged. There should be no merging at all if no matching file is found.

    *Compatibility note:* The filename specified inside the `<MergeFile>` element should be ignored if the type attribute is set to `parent`, it should however be expected that implementations based on previous versions of this specification will ignore the type attribute and that such implementations will use the filename inside the `<MergeFile>` element instead.

    Example 1: If `$XDG_CONFIG_HOME` is `~/.config/` and `$XDG_CONFIG_DIRS` is `/opt/gnome/:/etc/xdg/` and the file `~/.config/menus/applications.menu` contains `<MergeFile type="parent">/opt/kde3/etc/xdg/menus/applications.menu</MergeFile>` then the file `/opt/gnome/menus/applications.menu` should be merged if it exists. If that file does not exists then the file `/etc/xdg/menus/applications.menu` should be merged instead.
    
    Example 2: If `$XDG_CONFIG_HOME` is `~/.config/` and `$XDG_CONFIG_DIRS` is `/opt/gnome/:/etc/xdg/` and the file `/opt/gnome/menus/applications.menu` contains `<MergeFile type="parent">/opt/kde3/etc/xdg/menus/applications.menu</MergeFile>` then the file `/etc/xdg/menus/applications.menu` should be merged if it exists. 
*/
#[derive(Debug, Default, Clone, Deserialize, Serialize, PartialEq)]
pub struct MergeFile {
    pub r#type:Option<MergeFileType>,
    //TODO
    #[serde(rename = "$value")]
    pub text:String,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
pub enum MergeFileType {
    Path,
    Parent,
}
impl Default for MergeFileType {
    fn default() -> Self {
        Self::Path
    }
}
/// #### Attributes
/// `[show_empty="false"] [inline="false"] [inline_limit="4"] [inline_header="true"] [inline_alias="false"]`
/// 
/// The `<DefaultLayout>` element is an optional part of this specificatPath::new(ion. Implementations that do not support the `<DefaultLayout>` element should preserve any `<DefaultLayout>` elements and their contents as far as possible. Each `<Menu>` may optionally contain a `<DefaultLayout>` element which defines the default-layout for the current menu and all its sub-menus. If a menu has a `<DefaultLayout>` element then this will override any default-layout specified by a parent menu. The default-layout defines the suggested layout if a `<Menu>` element does either not have `<Layout>` element or if it has an empty `<Layout>` element. For explanations of the various attributes see the <Menuname> element. If no default-layout has been specified then the layout as specified by the following elements should be assumed:
/// ```xml
/// <DefaultLayout show_empty="false" inline="false" inline_limit="4" inline_header="true" inline_alias="false">
///   <Merge type="menus"/><Merge type="files"/>
/// </DefaultLayout>
///```
 #[derive(Debug, Default, Clone, Copy, Deserialize, Serialize, PartialEq)]
pub struct DefaultLayout {
    pub show_empty:Option<bool>,
    pub inline:Option<bool>,
    pub inline_header:Option<bool>,
    pub inline_alias:Option<bool>,
    pub inline_limit:Option<u32>,
}

/// #### Attributes
/// `[show_empty="..."] [inline="..."] [inline_limit="..."] [inline_header="..."] [inline_alias="..."]`
/// 
/// This element may only appear as a child of a `<Layout>` or `<DefaultLayout>` menu. Its contents references an immediate sub-menu of the current menu as defined with the `<Menu>` element, as such it should never contain a slash. If no such sub-menu exists the element should be ignored. This element may have various attributes, the default values are taken from the DefaultLayout key. The show_empty attribute defines whether a menu that contains no desktop entries and no sub-menus should be shown at all. The show_empty attribute can be "true" or "false". It may have an inline attribute that can be either "true" or "false". If the inline attribute is "true" the menu that is referenced may be copied into the current menu at the current point instead of being inserted as sub-menu of the current menu. The optional inline_limit attribute defines the maximum number of entries that can be inlined. If the sub-menu has more entries than inline_limit, the sub-menu will not be inlined. If the inline_limit is 0 (zero) there is no limit. The optional inline_header attribute defines whether an inlined menu should be preceded with a header entry listing the caption of the sub-menu. The inline_header attribute can be either "true" or "false". The optional inline_alias attribute defines whether a single inlined entry should adopt the caption of the inlined menu. In such case no additional header entry will be added regardless of the value of the inline_header attribute. The inline_alias attribute can be either "true" or "false".
/// 
///  Example: if a menu has a sub-menu titled "WordProcessor" with a single entry "OpenOffice 4.2", and both inline="true" and inline_alias="true" are specified then this would result in the "OpenOffice 4.2" entry being inlined in the current menu but the "OpenOffice 4.2" caption of the entry would be replaced with "WordProcessor".
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq)]
pub struct Menuname {
    pub show_empty:Option<bool>,
    pub inline:Option<bool>,
    pub inline_header:Option<bool>,
    pub inline_alias:Option<bool>,
    pub inline_limit:Option<u32>,
    #[serde(rename = "$value")]
    pub text:Option<String>, //TODO
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum Rules {
    /// The `<Filename>` element is the most basic matching rule. It matches a desktop entry if the desktop entry has the given desktop-file id. [See Desktop-File Id](https://specifications.freedesktop.org/menu-spec/latest/go01.html#term-desktop-file-id).
    #[serde(rename = "$value")]
    Filename(String),
    /// The `<Category>` element is another basic matching predicate. It matches a desktop entry if the desktop entry has the given category in its `Menu` field.
    #[serde(rename = "$value")]
    Category(String),
    /// The `<All>` element is a matching rule that matches all desktop entries.
    #[serde(rename = "$value")]
    All(Vec<Rules>),
    /// The `<And>` element contains a list of matching rules. If each of the matching rules inside the `<And>` element match a desktop entry, then the entire `<And>` rule matches the desktop entry.
    #[serde(rename = "$value")]
    And(Vec<Rules>),
    /// The `<Or>` element contains a list of matching rules. If any of the matching rules inside the `<Or>` element match a desktop entry, then the entire `<Or>` rule matches the desktop entry. 
    #[serde(rename = "$value")]
    Or(Vec<Rules>),
    /// The `<Not>` element contains a list of matching rules. If any of the matching rules inside the `<Not>` element matches a desktop entry, then the entire `<Not>` rule does not match the desktop entry. That is, matching rules below `<Not>` have a logical OR relationship. 
    #[serde(rename = "$value")]
    Not(Vec<Rules>),
    #[serde(other, deserialize_with = "deserialize_ignore_any")]
    Other,
}
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
/// This element may only appear below `<Menu>`. The `<Move>` element contains pairs of `<Old>`/`<New>` elements indicating how to rename a descendant of the current `<Menu>`. If the destination path already exists, the moved menu is merged with the destination menu [see the section called “Merging” for details](https://specifications.freedesktop.org/menu-spec/latest/ar01s05.html).
/// 
/// `<Move>` is used primarily to fix up legacy directories. For example, say you are merging a `<LegacyDir>` with folder names that don't match the current hierarchy; the legacy folder names can be moved to the new names, where they will be merged with the new folders.
/// 
/// `<Move>` is also useful for implementing menu editing, [see the section called “Menu editing”](https://specifications.freedesktop.org/menu-spec/latest/apd.html#menu-editing).
pub enum Move {
    /// This element may only appear below `<Move>`, and must be followed by a `<New>` element. The content of both `<Old>` and `<New>` should be a menu path, slash-separated concatenation of `<Name>` fields, [see Menu path](https://specifications.freedesktop.org/menu-spec/latest/go01.html#term-menu-path). Paths are interpreted relative to the menu containing the `<Move>` element.
    Old(String),
    /// This element may only appear below `<Move>`, and must be preceded by an `<Old>` element. The `<New>` element specifies the new path for the preceding `<Old>` element. 
    New(String),
    #[serde(other, deserialize_with = "deserialize_ignore_any")]
    Other,
}


#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
/// # Format of menu files
///
/// Menu files must be well-formed XML files and end in the extension ".menu". They should also conform to the menu file DTD which implies that implementation-specific extensions to the file format are not allowed. Implementations may stop processing if they encounter a menu file which does not comply with the associated DTD. Note that the associated DTD may differ in version from the one defined in this document.
///
/// When an implementation updates an existing menu file it may need to update the identifier to a newer version of the DTD. Implementations should never update the identifier of an existing menu file to an older version. In order to remain compatible with newer versions, implementations should ignore and preserve any XML elements, attributes and attribute values that it does not know how to handle.
///
/// # Document Type Declaration
/// Menu files for this version of the specification must use the following namespace, public and system identifiers:
/// * Namespace `http://www.freedesktop.org/standards/menu`
/// * Public Identifier for 1.1 `PUBLIC "-//freedesktop//DTD Menu 1.0//EN"`
/// * System Identifier for 1.1 `http://www.freedesktop.org/standards/menu-spec/menu-1.0.dtd`
///
/// Here is a sample document type declaration:
/// ```xml
///<!DOCTYPE Menu PUBLIC "-//freedesktop//DTD Menu 1.0//EN" "http://www.freedesktop.org/standards/menu-spec/menu-1.0.dtd">
///```
/// All menu files *MUST* include the document type declaration, so that implementations can adapt to different versions of this specification (and so implementations can validate the menu file against the DTD).
///
/// ## NOTES:
/// And, Or, All, Not are capitalized since `Not` cannot be used, I opted for another standard convention ALL CAPS for those words in programming
pub enum Menu {
    /// The root element is `<Menu>`. Each `<Menu>` element may contain any number of nested `<Menu>` elements, indicating submenus.
    Menu(Vec<Menu>),
    /// This element may only appear below `<Menu>`. The content of this element is a directory name. Desktop entries in this directory are scanned and added to the pool of entries which can be included in this `<Menu>` and its submenus. Only files ending in `.desktop` should be used, other files are ignored.
///
    ///Desktop entries in the pool of available entries are identified by their desktop-file id [see Desktop-File Id](https://specifications.freedesktop.org/menu-spec/latest/go01.html#term-desktop-file-id). The desktop-file id of a desktop entry is equal to its filename, with any path components removed. So given a `<AppDir>` `/foo/bar` and desktop entry `/foo/bar/Hello.desktop` the desktop entry would get a desktop-file id of `Hello.desktop`
///
    ///If the directory contains sub-directories then these sub-directories should be (recursively) scanned as well. The name of the subdirectory should be added as prefix to the desktop-file id together with a dash character ("-") So given a `<AppDir>` `/foo/bar` and desktop entry `/foo/bar/booz/Hello.desktop` the desktop entry would get a desktop-file id of `booz-Hello.desktop` A desktop entry `/foo/bar/bo/oz/Hello.desktop` would result in a desktop-file id of `bo-oz-Hello.desktop`
///
    ///`<AppDir>` elements appearing later in the menu file have priority in case of collisions between desktop-file ids.
///
    ///If the filename given as an `<AppDir>` is not an absolute path, it should be located relative to the location of the menu file being parsed.
///
    ///Duplicate `<AppDir>` elements (that specify the same directory) should be ignored, but the last duplicate in the file should be used when establishing the order in which to scan the directories. This is important when merging [see the section called “Merging”](https://specifications.freedesktop.org/menu-spec/latest/ar01s05.html). The order of `<AppDir>` elements with respect to `<Include>` and `<Exclude>` elements is not relevant, also to facilitate merging. 
    AppDir(String),
    ///This element may only appear below `<Menu>`. The element has no content. The element should be treated as if it were a list of `<AppDir>` elements containing the default app dir locations (datadir/applications/ etc.). When expanding `<DefaultAppDirs>` to a list of `<AppDir>`, the default locations that are earlier in the search path go later in the `<Menu>` so that they have priority.
    DefaultAppDirs,
    /// This element may only appear below `<Menu>`. The content of this element is a directory name. Each directory listed in a `<DirectoryDir>` element will be searched for directory entries to be used when resolving the `<Directory>` element for this menu and its submenus. If the filename given as a `<DirectoryDir>` is not an absolute path, it should be located relative to the location of the menu file being parsed.
///
///    Directory entries in the pool of available entries are identified by their relative path [see Relative path](https://specifications.freedesktop.org/menu-spec/latest/go01.html#term-relative-path).
///
///    If two directory entries have duplicate relative paths, the one from the last (furthest down) element in the menu file must be used. Only files ending in the extension `.directory` should be loaded, other files should be ignored.
///
///    Duplicate `<DirectoryDir>` elements (that specify the same directory) are handled as with duplicate `<AppDir>` elements (the last duplicate is used). 
    DirectoryDir(String),
    /// This element may only appear below `<Menu>`. The element has no content. The element should be treated as if it were a list of `<DirectoryDir>` elements containing the default desktop dir locations (`datadir`/desktop-directories/ etc.). The default locations that are earlier in the search path go later in the `<Menu>` so that they have priority. 
    DefaultDirectoryDirs,
    /// Each `<Menu>` element must have a single `<Name>` element. The content of the `<Name>` element is a name to be used when referring to the given menu. Each submenu of a given `<Menu>` must have a unique name. `<Menu>` elements can thus be referenced by a menu path, for example `Applications/Graphics`. The `<Name>` field must not contain the slash character ("/"); implementations should discard any name containing a slash. [See also Menu path](https://specifications.freedesktop.org/menu-spec/latest/go01.html#term-menu-path).
    Name(String),
    /// Each `<Menu>` element has any number of `<Directory>` elements. The content of the `<Directory>` element is the relative path of a directory entry containing meta information about the `<Menu>`, such as its icon and localized name. If no `<Directory>` is specified for a `<Menu>`, its `<Name>` field should be used as the user-visible name of the menu.
///
    /// Duplicate `<Directory>` elements are allowed in order to simplify menu merging, and allow user menus to override system menus. The last `<Directory>` element to appear in the menu file "wins" and other elements are ignored, unless the last element points to a nonexistent directory entry, in which case the previous element should be tried instead, and so on. 
    Directory(String),
    ///Each `<Menu>` may contain any number of `<OnlyUnallocated>` and <NotOnlyUnallocated> elements. Only the last such element to appear is relevant, as it determines whether the `<Menu>` can contain any desktop entries, or only those desktop entries that do not match other menus. If neither `<OnlyUnallocated>` nor <NotOnlyUnallocated> elements are present, the default is <NotOnlyUnallocated>.
///
    ///To handle `<OnlyUnallocated>`, the menu file must be analyzed in two conceptual passes. The first pass processes `<Menu>` elements that can match any desktop entry. During this pass, each desktop entry is marked as allocated according to whether it was matched by an `<Include>` rule in some `<Menu>`. The second pass processes only `<Menu>` elements that are restricted to unallocated desktop entries. During the second pass, queries may only match desktop entries that were not marked as allocated during the first pass. [See the section called “Generating the menus”](https://specifications.freedesktop.org/menu-spec/latest/ar01s06.html).
    OnlyUnallocated,
    NotOnlyUnallocated,
    /// Each `<Menu>` may contain any number of '<Deleted>' and `<NotDeleted>` elements. Only the last such element to appear is relevant, as it determines whether the `<Menu>` has been deleted. If neither '<Deleted>' nor `<NotDeleted>` elements are present, the default is `<NotDeleted>`. The purpose of this element is to support menu editing. If a menu contains a '<Deleted>' element not followed by a `<NotDeleted>` element, that menu should be ignored. 
    Deleted(Vec<Menu>),
    NotDeleted(Vec<Menu>),
    /// An `<Include>` element is a set of rules attempting to match some of the known desktop entries. The `<Include>` element contains a list of any number of matching rules. Matching rules are specified using the elements `<And>`, `<Or>`, `<Not>`, `<All>`, `<Filename>`, and `<Category>`. Each rule in a list of rules has a logical OR relationship, that is, desktop entries which match any rule are included in the menu.
/// 
    /// `<Include>` elements must appear immediately under `<Menu>` elements. The desktop entries they match are included in the menu. `<Include>` and `<Exclude>` elements for a given `<Menu>` are processed in order, with queries earlier in the file handled first. This has implications for merging, [see the section called “Merging”](https://specifications.freedesktop.org/menu-spec/latest/ar01s05.html). [See the section called “Generating the menus” for full details on how to process `<Include>` and `<Exclude>` elements](https://specifications.freedesktop.org/menu-spec/latest/ar01s06.html).
    #[serde(rename = "$value")]
    Include(Vec<Rules>),
    /// Any number of `<Exclude>` elements may appear below a `<Menu>` element. The content of an `<Exclude>` element is a list of matching rules, just as with an `<Include>`. However, the desktop entries matched are removed from the list of desktop entries included so far. (Thus an `<Exclude>` element that appears before any `<Include>` elements will have no effect, for example, as no desktop entries have been included yet.)
    #[serde(rename = "$value")]
    Exclude(Vec<Rules>),

    /// #### Attributes:
    /// `[type="path"|"parent"]`
/// 
    /// Any number of `<MergeFile>` elements may be listed below a `<Menu>` element, giving the name of another menu file to be merged into this one. The section called [“Merging”](https://specifications.freedesktop.org/menu-spec/latest/ar01s05.html) specifies how merging is done. The root `<Menu>` of the merged file will be merged into the immediate parent of the `<MergeFile>` element. The `<Name>` element of the root `<Menu>` of the merged file are ignored.
/// 
    /// If the type attribute is missing or set to "path" then the contents of the `<MergeFile>` element indicates the file to be merged. If this is not an absolute path then the file to be merged should be located relative to the location of the menu file that contains this `<MergeFile>` element.
/// 
    /// Duplicate `<MergeFile>` elements (that specify the same file) are handled as with duplicate `<AppDir>` elements (the last duplicate is used).
/// 
    /// If the type attribute is set to `parent` and the file that contains this `<MergeFile>` element is located under one of the paths specified by `$XDG_CONFIG_DIRS`, the contents of the element should be ignored and the remaining paths specified by `$XDG_CONFIG_DIRS` are searched for a file with the same relative filename. The first file encountered should be merged. There should be no merging at all if no matching file is found.
/// 
    /// *Compatibility note:* The filename specified inside the `<MergeFile>` element should be ignored if the type attribute is set to `parent`, it should however be expected that implementations based on previous versions of this specification will ignore the type attribute and that such implementations will use the filename inside the `<MergeFile>` element instead.
/// 
    /// Example 1: If `$XDG_CONFIG_HOME` is `~/.config/` and `$XDG_CONFIG_DIRS` is `/opt/gnome/:/etc/xdg/` and the file `~/.config/menus/applications.menu` contains `<MergeFile type="parent">/opt/kde3/etc/xdg/menus/applications.menu</MergeFile>` then the file `/opt/gnome/menus/applications.menu` should be merged if it exists. If that file does not exists then the file `/etc/xdg/menus/applications.menu` should be merged instead.
/// 
    /// Example 2: If `$XDG_CONFIG_HOME` is `~/.config/` and `$XDG_CONFIG_DIRS` is `/opt/gnome/:/etc/xdg/` and the file `/opt/gnome/menus/applications.menu` contains `<MergeFile type="parent">/opt/kde3/etc/xdg/menus/applications.menu</MergeFile>` then the file `/etc/xdg/menus/applications.menu` should be merged if it exists. 
    MergeFile(Vec<MergeFile>),
    /// Any number of `<MergeDir>` elements may be listed below a `<Menu>` element. A `<MergeDir>` contains the name of a directory. Each file in the given directory which ends in the ".menu" extension should be merged in the same way that a `<MergeFile>` would be. If the filename given as a `<MergeDir>` is not an absolute path, it should be located relative to the location of the menu file being parsed. The files inside the merged directory are not merged in any specified order.
/// 
    /// Duplicate `<MergeDir>` elements (that specify the same directory) are handled as with duplicate `<AppDir>` elements (the last duplicate is used). 
    MergeDir(Vec<String>),
    /// This element may only appear below `<Menu>`. The element has no content. The element should be treated as if it were a list of `<MergeDir>` elements containing the default merge directory locations. When expanding `<DefaultMergeDirs>` to a list of `<MergeDir>`, the default locations that are earlier in the search path go later in the `<Menu>` so that they have priority. 
    DefaultMergeDirs,
    /// This element may only appear below `<Menu>`. The text content of this element is a directory name. Each directory listed in a `<LegacyDir>` element will be an old-style legacy hierarchy of desktop entries, [see the section called “Legacy Menu Hierarchies”](https://specifications.freedesktop.org/menu-spec/latest/ar01s07.html) for how to load such a hierarchy. Implementations must not load legacy hierarchies that are not explicitly specified in the menu file (because for example the menu file may not be the main menu). If the filename given as a `<LegacyDir>` is not an absolute path, it should be located relative to the location of the menu file being parsed.
/// 
    /// Duplicate `<LegacyDir>` elements (that specify the same directory) are handled as with duplicate `<AppDir>` elements (the last duplicate is used).
/// 
    /// The `<LegacyDir>` element may have one attribute, `prefix`. Normally, given a `<LegacyDir>` `/foo/bar` and desktop entry `/foo/bar/baz/Hello.desktop` the desktop entry would get a desktop-file id of `Hello.desktop`. Given a prefix of `boo-`, it would instead be assigned the desktop-file id `boo-Hello.desktop`. The prefix should not contain path separator ('/') characters. 
    LegacyDir(Option<String>, String),
    /// This element may only appear below `<Menu>`. The element has no content. The element should be treated as if it were a list of `<LegacyDir>` elements containing the traditional desktop file locations supported by KDE with a hard coded prefix of "kde-". When expanding `<KDELegacyDirs>` to a list of `<LegacyDir>`, the locations that are earlier in the search path go later in the `<Menu>` so that they have priority. The search path can be obtained by running `kde-config --path apps`
    KDELegacyDirs(String),
    /// This element may only appear below `<Menu>`. The `<Move>` element contains pairs of `<Old>`/`<New>` elements indicating how to rename a descendant of the current `<Menu>`. If the destination path already exists, the moved menu is merged with the destination menu [see the section called “Merging” for details](https://specifications.freedesktop.org/menu-spec/latest/ar01s05.html).
/// 
    /// `<Move>` is used primarily to fix up legacy directories. For example, say you are merging a `<LegacyDir>` with folder names that don't match the current hierarchy; the legacy folder names can be moved to the new names, where they will be merged with the new folders.
/// 
    /// `<Move>` is also useful for implementing menu editing, [see the section called “Menu editing”](https://specifications.freedesktop.org/menu-spec/latest/apd.html#menu-editing).
    Move(Move),
    /// The `<Layout>` element is an optional part of this specification. Implementations that do not support the `<Layout>` element should preserve any `<Layout>` elements and their contents as far as possible. Each `<Menu>` may optionally contain a `<Layout>` element. If multiple elements appear then only the last such element is relevant. The purpose of this element is to offer suggestions for the presentation of the menu. If a menu does not contain a `<Layout>` element or if it contains an empty `<Layout>` element then the default layout should be used. The `<Layout>` element may contain `<Filename>`, <Menuname>, <Separator> and <Merge> elements. The `<Layout>` element defines a suggested layout for the menu starting from top to bottom. References to desktop entries that are not contained in this menu as defined by the `<Include>` and `<Exclude>` elements should be ignored. References to sub-menus that are not directly contained in this menu as defined by the `<Menu>` elements should be ignored.
    Layout(Layout),
    /// #### Attributes
    /// `[show_empty="false"] [inline="false"] [inline_limit="4"] [inline_header="true"] [inline_alias="false"]`
/// 
    /// The `<DefaultLayout>` element is an optional part of this specificatPath::new(ion. Implementations that do not support the `<DefaultLayout>` element should preserve any `<DefaultLayout>` elements and their contents as far as possible. Each `<Menu>` may optionally contain a `<DefaultLayout>` element which defines the default-layout for the current menu and all its sub-menus. If a menu has a `<DefaultLayout>` element then this will override any default-layout specified by a parent menu. The default-layout defines the suggested layout if a `<Menu>` element does either not have `<Layout>` element or if it has an empty `<Layout>` element. For explanations of the various attributes see the <Menuname> element. If no default-layout has been specified then the layout as specified by the following elements should be assumed:
    /// ```xml
    /// <DefaultLayout show_empty="false" inline="false" inline_limit="4" inline_header="true" inline_alias="false">
    ///   <Merge type="menus"/><Merge type="files"/>
    /// </DefaultLayout>
    ///```
    DefaultLayout(DefaultLayout),
    /// #### Attributes
    /// `[show_empty="..."] [inline="..."] [inline_limit="..."] [inline_header="..."] [inline_alias="..."]`
/// 
    /// This element may only appear as a child of a `<Layout>` or `<DefaultLayout>` menu. Its contents references an immediate sub-menu of the current menu as defined with the `<Menu>` element, as such it should never contain a slash. If no such sub-menu exists the element should be ignored. This element may have various attributes, the default values are taken from the DefaultLayout key. The show_empty attribute defines whether a menu that contains no desktop entries and no sub-menus should be shown at all. The show_empty attribute can be "true" or "false". It may have an inline attribute that can be either "true" or "false". If the inline attribute is "true" the menu that is referenced may be copied into the current menu at the current point instead of being inserted as sub-menu of the current menu. The optional inline_limit attribute defines the maximum number of entries that can be inlined. If the sub-menu has more entries than inline_limit, the sub-menu will not be inlined. If the inline_limit is 0 (zero) there is no limit. The optional inline_header attribute defines whether an inlined menu should be preceded with a header entry listing the caption of the sub-menu. The inline_header attribute can be either "true" or "false". The optional inline_alias attribute defines whether a single inlined entry should adopt the caption of the inlined menu. In such case no additional header entry will be added regardless of the value of the inline_header attribute. The inline_alias attribute can be either "true" or "false".
/// 
    ///  Example: if a menu has a sub-menu titled "WordProcessor" with a single entry "OpenOffice 4.2", and both inline="true" and inline_alias="true" are specified then this would result in the "OpenOffice 4.2" entry being inlined in the current menu but the "OpenOffice 4.2" caption of the entry would be replaced with "WordProcessor".
    Menuname(Menuname),
    /// This element may only appear as a child of a `<Layout>` or `<DefaultLayout>` menu. It indicates a suggestion to draw a visual separator at this point in the menu. `<Separator>` elements at the start of a menu, at the end of a menu or that directly follow other `<Separator>` elements may be ignored. 
    Separator,
    /// `type="menus"|"files"|"all"`
    /// 
    /// This element may only appear as a child of a `<Layout>` or `<DefaultLayout>` menu. It indicates the point where desktop entries and sub-menus that are not explicitly mentioned within the `<Layout>` or `<DefaultLayout>` element are to be inserted. It has a type attribute that indicates which elements should be inserted: type="menus" means that all sub-menus that are not explicitly mentioned should be inserted in alphabetical order of their visual caption at this point. `type="files"` means that all desktop entries contained in this menu that are not explicitly mentioned should be inserted in alphabetical order of their visual caption at this point. `type="all"` means that a mix of all sub-menus and all desktop entries that are not explicitly mentioned should be inserted in alphabetical order of their visual caption at this point. Each `<Layout>` or ``<DefaultLayout>`` element shall have exactly one <Merge type="all">` element or it shall have exactly one `<Merge type="files">` and exactly one `<Merge type="menus">` element. An exception is made for a completely empty `<Layout>` element which may be used to indicate that the default-layout should be used instead.
    Merge(Merge),
     #[serde(other, deserialize_with = "deserialize_ignore_any")]
    Other,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DesktopMenu {
    #[serde(rename = "$value")]
    pub items:Vec<Menu>,
}
impl DesktopMenu {
        pub fn read<P: Clone + AsRef<std::path::Path> + std::fmt::Debug>(filename:P) -> Option<Self> {
        if let Ok(file_string) = std::fs::read_to_string(filename.clone()) {
            let decoded:Self = match quick_xml::de::from_str(file_string.as_str()) {
                Ok(decoded) => decoded,
                Err(e) => {
                    println!("DesktopMenu::read()->from_str()Error:\n*{}*\nFilename:{:?}", e, filename);
                    return None;
                },
            };
            return Some(decoded);
        };
        // I do not think this is possible to get to
        println!("DesktopMenu::read() *FAILED* Filename:{:?}", filename);
        None
    }
}

#[derive(Debug, Clone)]
pub struct MenuItem {
    /// Name
    pub name:String,
    /// Icon 
    pub icon:String,
    ///  Tooltip
    pub tooltip:String,
    ///  Executable
    pub executable:String,
    ///  Is this a separator?
    pub separator:bool,
    /// Does this need a terminal?
    pub terminal:bool,
}
impl Default for MenuItem {
    fn default() -> Self {
        MenuItem {
            name:String::from(""),
            icon:String::from(""),
            tooltip:String::from(""),
            executable:String::from(""),
            separator:false,
            terminal:false,
        }
    }
}
impl MenuItem {
    pub fn make_separator() -> Self {
        let mut m = Self::default();
        m.separator = true;
        m
    }
    pub fn empty_app() -> Self {
        let mut m = Self::default();
        m.icon = String::from("application-default-icon");
        m
    }
}

