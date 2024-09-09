use backup::file_read::read_file_as_stream;
use config::config_read::{Backup, Server, };
use server::server_connect::sftp_upload_file_to_server;

pub(crate) fn run_server(backup: Backup, server: Server) {
    let stream = match read_file_as_stream(backup.file_path(), "用例表.md") {
        Ok(file_stream) => file_stream,
        Err(_) => {
            println!("服务器检测失败");
            return;
        }
    };

    let upload_result = sftp_upload_file_to_server(server.remote_path(), stream, server.server_ip_address(), server.server_port(), server.user(), server.password());
    match upload_result {
        Ok(_) => println!("上传成功"),
        Err(e) => println!("上传失败: {}", e)
    }

}


#[cfg(test)]
mod test {
    use config::config_read::{get_config};
    use crate::serve::run_server;

    #[test]
    fn test_run_server() {
        let backup = get_config().get_backup();
        let server = get_config().get_server();

        run_server(backup, server);

    }


}