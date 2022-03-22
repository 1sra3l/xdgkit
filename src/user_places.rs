extern crate serde;
use serde_xml_rs::from_str;
use serde::{Deserialize, Deserializer, Serialize};
//#[serde(rename = "$value")]
//#[serde(rename = "")]

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
/* # */
pub struct {}

pub struct KdeMetadata {
    /// 
    #[serde(rename = "ID")]
    pub id:u32
    /// 
    #[serde(rename = "bookmark:icon")]
    pub bookmark:icon:String,
    /// 
    #[serde(rename = "IsHidden")]
    pub is_hidden:bool,
    /// 
    #[serde(rename = "isSystemItem")]
    pub is_system_item:bool,
    /// 
    pub owner:String,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub struct Metadata {
    /// 
    #[serde(rename = "ID")]
    pub id:u32
    /// 
    #[serde(rename = "bookmark:icon")]
    pub bookmark:icon:String,
    /// 
    #[serde(rename = "IsHidden")]
    pub is_hidden:bool,
    /// 
    #[serde(rename = "isSystemItem")]
    pub is_system_item:bool,
    /// 
    pub owner:String,
    /// 
    pub kde_places_version:u32,
    #[serde(rename = "GroupState-Places-IsHidden")]
    /// 
    pub places_hidden:bool,
    #[serde(rename = "GroupState-Remote-IsHidden")]
    /// 
    pub remote_hidden:bool,
    #[serde(rename = "GroupState-Devices-IsHidden")]
    /// 
    pub devices_hidden:bool,
    #[serde(rename = "GroupState-RemovableDevices-IsHidden")]
    /// 
    pub removable_devices_hidden:bool,
    #[serde(rename = "GroupState-Tags-IsHidden")]
    /// 
    pub tags_hidden:bool,
    #[serde(rename = "withRecentlyUsed")]
    /// 
    pub with_recently_used:bool,
    #[serde(rename = "GroupState-RecentlySaved-IsHidden")]
    /// 
    pub recently_saved_hidden:bool,
    #[serde(rename = "withBaloo")]
    /// 
    pub with_baloo:bool,
    #[serde(rename = "GroupState-SearchFor-IsHidden")]
    /// 
    pub search_for_hidden:bool,
    
}
