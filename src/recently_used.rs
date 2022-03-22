
extern crate serde;
use serde_xml_rs::from_str;
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DateString(String);

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename = "$value")]
pub struct MimeMimeType {
    pub r#type:String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename = "bookmark:application")]
pub struct BookmarkApplication {
    pub name:String,
    pub exec:String,
    pub modified:String,
    pub count:u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub struct Metadata {
    pub owner:String,
    #[serde(rename = "mime:mime-type")]
    pub mime:MimeMimeType,
    #[serde(rename = "bookmark:applications")]
    pub bookmark:Vec<BookmarkApplication>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub struct Info {
    pub metadata:Metadata,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub struct Bookmark {
    pub href:String,
    pub added:String,//DateString
    pub modified:String,
    pub visited:String,
    pub info:Info,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub struct Xbel {
    #[serde(rename = "xmlns:mime")]
    pub mime:String,
    #[serde(rename = "xmlns:bookmark")]
    pub bookmark:String,
    #[serde(rename = "$value")]
    pub items:Vec<Bookmark>,
}
impl Default for Xbel {
    fn default() -> Self {
        Self {
            items:vec![],
        }
    }
}
impl Xbel {
        pub fn read<P: Clone + AsRef<std::path::Path> + std::fmt::Debug>(filename:P) -> Option<Self> {
        if let Ok(file_string) = std::fs::read_to_string(filename.clone()) {
            let decoded:Self = match from_str(file_string.as_str()) {
                Ok(decoded) => decoded,
                Err(e) => {
                    println!("Xbel::read()->from_str()\nError:{}\nFilename:{:?}", e, filename);
                    return None
                },
            };
            return Some(decoded);
        }
        // I do not think this is possible to get to
        println!("Xbel::read() *FAILED* Filename:{:?}", filename);
        None
    }
}
