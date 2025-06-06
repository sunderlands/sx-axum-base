use std::{fs::File, io::Read, sync::OnceLock};

use anyhow::Context;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub server: Server,
    pub database: Option<Database>,
    pub log: Log,
}
impl Config {
    fn load() -> anyhow::Result<Self> {
        let path = ".toml";
        let error_msg =
            |msg: &str| -> String { format!("{}，配置文件路径:\"{}\"", msg, path) };

        let mut file = File::open(path).context(error_msg("读取配置文件失败"))?;
        let mut buf = String::new();
        file.read_to_string(&mut buf)
            .context(error_msg("配置文件内容读取失败"))?;
        let settings = toml::from_str(&buf).context(error_msg("配置文件格式化失败"))?;

        Ok(settings)
    }
}

#[derive(Debug, Deserialize)]
pub struct Server {
    pub host: String,
    pub port: u16,
    pub cors: Option<Cors>,
}

#[derive(Debug, Deserialize)]
pub struct Cors {
    pub origin: Option<Vec<String>>,
    pub methods: Option<Vec<String>>,
    pub headers: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct Database {
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct Log {
    pub level: String,
    pub debug: bool,
}

static CONFIG: OnceLock<Config> = OnceLock::new();

pub fn instance() -> &'static Config {
    CONFIG.get_or_init(|| match Config::load() {
        Ok(config) => config,
        Err(err) => panic!("{}", toolkit::to_string::err_chain(err)),
    })
}
