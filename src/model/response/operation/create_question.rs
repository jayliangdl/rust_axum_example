use serde::Serialize;
#[derive(Serialize,Debug)]
pub struct CreateQuestion{
    pub data:Option<String>
}