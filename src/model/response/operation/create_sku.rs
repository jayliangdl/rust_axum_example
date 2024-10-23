use serde::Serialize;
#[derive(Serialize,Debug,Clone)]
pub struct CreateSku{
    pub sku_code:String
}