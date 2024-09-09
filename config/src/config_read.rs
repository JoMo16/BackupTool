use serde::{Deserialize, Serialize};

/*
    Config
 */
#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    backup: Backup,
    server: Server,
}

impl Config {
    pub fn get_backup(self) -> Backup {
        self.backup
    }
    pub fn get_server(self) -> Server {
        self.server
    }
}

/*
    备份的相关配置
 */
#[derive(Debug, Deserialize, Serialize)]
pub struct Backup {
    file_path: String,
    auto_backup_time: String,
}

impl Backup {
    pub fn new(file_path: String, auto_backup_time: String) -> Self {
        Self { file_path, auto_backup_time }
    }
    pub fn set_file_path(&mut self, file_path: String) {
        self.file_path = file_path;
    }
    pub fn set_auto_backup_time(&mut self, auto_backup_time: String) {
        self.auto_backup_time = auto_backup_time;
    }
    pub fn file_path(&self) -> &str {
        &self.file_path
    }
    pub fn auto_backup_time(&self) -> &str {
        &self.auto_backup_time
    }
}

/*
    服务器相关的配置
 */
#[derive(Debug, Deserialize, Serialize)]
pub struct Server {
    server_ip_address: String,
    server_port: u16,
    user: String,
    password: String,
    remote_path: String,
}

impl Server {
    pub fn new(server_ip_address: String, server_port: u16, user: String, password: String, remote_path: String) -> Self {
        Self { server_ip_address, server_port, user, password, remote_path }
    }
    pub fn set_server_ip_address(&mut self, server_ip_address: String) {
        self.server_ip_address = server_ip_address;
    }
    pub fn set_server_port(&mut self, server_port: u16) {
        self.server_port = server_port;
    }
    pub fn set_user(&mut self, user: String) {
        self.user = user;
    }
    pub fn set_password(&mut self, password: String) {
        self.password = password;
    }
    pub fn set_remote_path(&mut self, remote_path: String) {
        self.remote_path = remote_path;
    }
    pub fn server_ip_address(&self) -> &str {
        &self.server_ip_address
    }
    pub fn server_port(&self) -> u16 {
        self.server_port
    }
    pub fn user(&self) -> &str {
        &self.user
    }
    pub fn password(&self) -> &str {
        &self.password
    }
    pub fn remote_path(&self) -> &str {
        &self.remote_path
    }
}

/*
    auth: jomo Time: 2024.9.3
    读取config.yaml的配置信息,并将其转换为Config
 */
pub(self) fn read_config () ->  Result<Config, Box<dyn std::error::Error>>{
    // 初始化配置文件路径
    let config_content = include_str!("../../config.yaml");
    let config: Config = serde_yaml::from_str(&config_content)?;    //将config转变为Result

    Ok(config)
}

/*
    auth: jomo Time: 2024.9.3
    拆分Config对象中的对象，将对象独立出来
 */
pub fn get_config () -> Config {
    match read_config() {
        Ok(config) => {
            config
        }
        Err(e) => {
            panic!("文件不存在或存在其他错误，\n {}", e)
        }
    }
}



/*
    测试模块
 */
#[cfg(test)]
mod test {
    use crate::config_read::{get_config};

    #[test]
    fn test_get_config() {
        let config = get_config();

        println!("{:?}", config);
    }

}