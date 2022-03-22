extern crate serde;
use serde_xml_rs::from_str;
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
/// 
pub struct Bookmark {
    /// 
    pub title:Option<String>,
    /// 
    pub info:Option<Info>,
    /// 
    pub desc:Option<Desc>,
    /// 
    pub id:u32
    /// 
    pub added:String,
    /// 
    pub href:String, // required
    /// 
    pub modified:String,
    /// 
    pub visited:String,
    
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
/// 
pub struct Folder {
    /// 
    pub title:Option<String>,
    /// 
    pub info:Option<Info>,
    /// 
    pub desc:Option<Desc>,
    /// 
    pub id:u32
    /// 
    pub added:String,
    /// 
    pub folded: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
/// 
pub struct Metadata {
    pub owner:String,
    // anything can be here....
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
/// 
pub struct Alias {
    pub r#ref:Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
/// 
pub enum Node{
    Separator,
    Desc(String),
    Bookmark(Bookmark),
    Folder(Folder),
    Alias(Alias),
    Info(Metadata),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
/// 
pub struct Xbel {
    /// 
    pub title:Option<String>,
    /// 
    pub info:Option<Info>,
    /// 
    pub desc:Option<Desc>,
    /// 
    pub version:String,
    /// 
    #[serde(rename = "$value")]
    pub nodes:Vec<Node>,
}
