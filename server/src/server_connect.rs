use std::error::Error;
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
pub fn build_session(server_ip_address: &str, server_port: u16, user: &str, password: &str) -> Result<Session, Box<dyn Error>>{
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




