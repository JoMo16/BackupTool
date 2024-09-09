use std::error::Error;
use std::io::Write;
use std::net::TcpStream;
use ssh2::Session;

/*
    auth: jomo time: 2024.9.3
    格式化主机地址
 */
pub(crate) fn fmt_server_host(server_ip_address: &str, server_port: u16) -> String {
    format!("{}:{}", server_ip_address, server_port)
}

/*
    auth: jomo time: 2024.9.3
    跟指定的服务器建立初始连接
 */
pub(crate) fn connect_server(server_ip_address: &str, server_port: u16) ->  Result<TcpStream, Box<dyn std::error::Error>>{
    let host = fmt_server_host(server_ip_address, server_port);
    // 建立tcp连接
    let tcp_stream = TcpStream::connect(host)?;
    // 返回建立连接结果
    Ok(tcp_stream)
}

/*
    auth: jomo time: 2024.9.3
    根据建立的tcp连接再建立一个会话
 */
pub(crate) fn build_session(server_ip_address: &str, server_port: u16, user: &str, password: &str) -> Result<Session, Box<dyn Error>>{
    let tcp_connect_result = connect_server(server_ip_address, server_port);    //获取tcp连接
    let tcp_connect_stream = match tcp_connect_result {
        Ok(tcp_stream) => {
            tcp_stream
        }
        Err(e) => {
            println!("服务器连接异常，请检查是否正确补全配置文件或检查服务器防火墙是否正常启动: {}", e);
            return Err(e);
        }
    };

    let mut session = Session::new()?;
    session.set_tcp_stream(tcp_connect_stream); //建立tcp连接流
    session.handshake()?;   //握手
    session.userauth_password(user, password)?; //使用账号和密码登录

    // 验证使用用户名+密码登录是否成功
    if !session.authenticated() {
        println!("登录失败，请检查用户和密码是否正确")
    }

    Ok(session)

}

/*
    auth: jomo time: 2024.9.4
    将文件流上传到服务器
 */
pub fn sftp_upload_file_to_server(remote_path: &str, file_stream: Vec<u8>,server_ip_address: &str, server_port: u16, user: &str, password: &str) -> Result<(), Box<dyn Error>> {
    // 获取session连接，判断上一个步骤是否出错
    let sess = match build_session(server_ip_address, server_port, user, password){
        Ok(session) => {
            println!("建立session会话成功");
            session
        }
        Err(e) => {
            println!("建立session会话失败");
            return Err(e)
        }
    };

    // 创建sftp会话
    let sftp = sess.sftp()?;
    // 将文件上传至服务器
    let mut remote_file = sftp.create(remote_path.as_ref())?;
    let upload_result = remote_file.write_all(file_stream.as_ref())?;

    Ok(upload_result)
}


/*
    测试模块
 */
#[cfg(test)]
mod test {
    use crate::server_connect::{build_session, connect_server};

    #[test]
    fn test_tcp_connect () {
        let result = connect_server("192.168.10.5", 22);
        match result {
            Ok(_) => println!("connect success"),
            Err(_) => println!("connect failed")
        }
    }

    #[test]
    fn test_build_session() {
        let result = build_session("192.168.10.5", 22, "root", "123");
        match result {
            Ok(_) => {
                println!("建立会话成功")
            }
            Err(_) => {
                println!("建立会话失败")
            }
        }
    }
}




