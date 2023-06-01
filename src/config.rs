use log::error;
use schemars::schema::RootSchema;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ticket {
    pub id: String,
    pub num: usize,
    pub sessions: usize,
    pub grade: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Monitor {
    pub enable : bool,
    pub interval : u64,
    pub sessions : Vec<Sessions>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Sessions {
    pub index: usize,

    #[serde(deserialize_with = "deserialize_array_usize")]
    pub grades : Vec<usize>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Account {
    pub cookie: String,
    pub remark: String,
    pub ticket: Ticket,
    pub interval: Option<u64>,
    pub earliest_submit_time: Option<i64>,
    pub request_time: Option<i64>,
    pub retry_times : Option<u8>,
    pub retry_interval : Option<u64>,
    pub monitor : Option<Monitor>,
    pub dingtalk_notify : Option<bool>,
    pub dingtalk_token : Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub accounts: Vec<Account>,
}

fn deserialize_array_usize<'de, D>(deserializer: D) -> Result<Vec<usize>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: &str = serde::Deserialize::deserialize(deserializer)?;
    let items: Result<Vec<usize>, _> = s
        .split(',')
        .map(|item| item.trim().parse::<usize>())
        .collect();
    items.map_err(serde::de::Error::custom)
}


fn load_config<T>(path: &str) -> Option<T>
where
    T: DeserializeOwned,
{
    // 1.通过std::fs读取配置文件内容
    // 2.通过serde_yaml解析读取到的yaml配置转换成json对象
    match serde_yaml::from_str::<RootSchema>(
        &std::fs::read_to_string(path).unwrap_or_else(|_| panic!("failure read file {}", path)),
    ) {
        Ok(root_schema) => {
            // 通过serde_json把json对象转换指定的model
            let data =
                serde_json::to_string_pretty(&root_schema).expect("failure to parse RootSchema");
            let config = serde_json::from_str::<T>(&data)
                .unwrap_or_else(|_| panic!("failure to format json str {}", &data));
            // 返回格式化结果
            Some(config)
        }
        Err(err) => {
            // 记录日志
            error!("{}", err);
            // 返回None
            None
        }
    }
}

pub fn load_global_config() -> Option<Config> {
    load_config("./config/config.yaml")
}
