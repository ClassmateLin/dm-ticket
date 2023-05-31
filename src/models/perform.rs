use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use super::CommonParams;

pub struct PerformParams {}

impl PerformParams {
    pub fn build() -> Result<Value> {
        let mut params = serde_json::to_value(CommonParams::build())?;
        params["api"] = "mtop.alibaba.detail.subpage.getdetail".into();
        params["method"] = "GET".into();
        params["v"] = "2.0".into();
        Ok(params)
    }
}

pub struct PerformForm {}
impl PerformForm {
    pub fn build(ticket_id: String, perform_id: String) -> Result<Value> {
        let ex_params = json!({
            "dataType": 2,
            "dataId": perform_id,
            "privilegeActId":""
        });

        let data = json!({
        "itemId": ticket_id,
        "bizCode":"ali.china.damai",
        "scenario":"itemsku",
        "exParams": serde_json::to_string(&ex_params)?,
        "dmChannel":"damai@damaih5_h5"
        });

        Ok(data)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Sku {
    #[serde(rename = "skuId")]
    pub sku_id: String,

    #[serde(rename = "itemId")]
    pub item_id: String,

    #[serde(rename = "priceName")]
    pub price_name: String,

    pub price: String,

    #[serde(rename = "skuSalable")]
    pub sku_salable : String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Perform {
    #[serde(rename = "performId")]
    pub perform_id: String,

    #[serde(rename = "performName")]
    pub perform_name: String,

    #[serde(rename = "skuList")]
    pub sku_list: Vec<Sku>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PerformInfo {
    pub perform: Perform,
}
