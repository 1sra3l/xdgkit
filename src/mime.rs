pub struct Comment {
    #[serde(rename = "$value")]
    pub text:String,
    pub lang:String,
}
pub struct MimeType {
    pub comment:Vec<Comment>,
}
