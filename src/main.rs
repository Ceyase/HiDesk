// Specify the Windows subsystem to eliminate console window.
// Requires Rust 1.18.
//#![windows_subsystem = "windows"]
use std::thread;
use hbb_common::log;

use librustdesk::*;
use tonic::{transport::Server, Request, Response, Status};
use std::net::TcpListener;
use std::net::UdpSocket;
use std::env;
use std::process::Command;
use std::process::Child;
use std::fs;
use rand::Rng;
use platform_dirs::{AppDirs, UserDirs};

use hiper_link::plugin_server::{Plugin, PluginServer};
use hiper_link::{EventsRequest, EventsResponse, InteractionRequest, InteractionResponse};

use lazy_static::lazy_static;

pub mod hiper_link {
    include!("proto.rs");
}

lazy_static! {
	static ref password:String = rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(6)
        .map(char::from)
        .collect::<String>();
}

#[derive(Debug, Default)]
pub struct API {}

pub fn GetIPs() -> Option<String> {
	let output = Command::new("cmd").arg("/c").arg("ipconfig").output().expect("命令执行异常错误提示");
	let output_str = String::from_utf8_lossy(&output.stdout);
	for item in output_str.split("\\r\\n") {
		for prekey in item.split("\r") {
			if prekey.find("IPv4") != None {
				let listtemp = prekey.split(":");
				for i in listtemp{
					if &i[1..3] == "6." {
						return Some(i[1..i.len()].to_string());
					}
				}
			}
		}
    }
	return Some("请启动Hiper".to_string());
}

#[tonic::async_trait]
impl Plugin for API {

    // 加载插件
    async fn on_load(
        &self,
        _request: Request<EventsRequest>,
    ) -> Result<Response<EventsResponse>, Status> {
        let reply = hiper_link::EventsResponse {value : "1".into()};


        let app_dirs = AppDirs::new(Some("name"), false).unwrap();
        let appdata = app_dirs.config_dir;


		Command::new("cmd").arg("/c").arg("mkdir %AppData%\\HiDesk\\config").output();
		Command::new("cmd").arg("/c").arg("echo [options] > %AppData%\\HiDesk\\config\\HiDesk2.toml").output();
		Command::new("cmd").arg("/c").arg("echo direct-server = 'Y' >> %AppData%\\HiDesk\\config\\HiDesk2.toml").output();
		Command::new("cmd").arg("/c").arg("echo password = '".to_string()+&password+"' >> %AppData%\\HiDesk\\config\\HiDesk.toml").output();


        let exepath = env::current_exe()?.display().to_string();
        Command::new(exepath).arg("--server").spawn().unwrap();
        Ok(Response::new(reply))
    }

    // 停用插件
    async fn on_unload(
        &self,
        _request: Request<EventsRequest>,
    ) -> Result<Response<EventsResponse>, Status> {
        let reply = hiper_link::EventsResponse {value : "1".into()};
		
        Ok(Response::new(reply))
    }

    // 安装插件
    async fn on_install(
        &self,
        _request: Request<EventsRequest>,
    ) -> Result<Response<EventsResponse>, Status> {
        let reply = hiper_link::EventsResponse {value : "1".into()};

        Ok(Response::new(reply))
    }

    // 卸载插件
    async fn on_uninstall(
        &self,
        _request: Request<EventsRequest>,
    ) -> Result<Response<EventsResponse>, Status> {
        let reply = hiper_link::EventsResponse {value : "1".into()};

        Ok(Response::new(reply))
    }

    // HL 启动
    async fn on_start(
        &self,
        _request: Request<EventsRequest>,
    ) -> Result<Response<EventsResponse>, Status> {
        let reply = hiper_link::EventsResponse {value : "1".into()};

        Ok(Response::new(reply))
    }

    // HL 停止
    async fn on_stop(
        &self,
        _request: Request<EventsRequest>,
    ) -> Result<Response<EventsResponse>, Status> {
        let reply = hiper_link::EventsResponse {value : "1".into()};

        Ok(Response::new(reply))
    }

    // 前后端交互
    async fn interaction(
        &self,
        _request: Request<InteractionRequest>,
    ) -> Result<Response<InteractionResponse>, Status> {
        let exepath = env::current_exe()?.display().to_string();
		let function = _request.into_inner().function;
        let mut json_text = "".to_string();

		match &function as &str{
            "getInfo" => {
			let mut ip_text = "";
			json_text = "{\"IP\":\"".to_string()+&GetIPs().unwrap()+"\",\"Password\":\""+&password+"\",\"Version\":\"1.0.2\"}";
			},
			_ => {}
		}

        if function.find("CIP") != None {
            let ip_text = &function[3..function.len()];
            Command::new(exepath).arg("--connect").arg(ip_text).spawn().unwrap();
        }
        let reply = hiper_link::InteractionResponse {value : json_text.into()};
        Ok(Response::new(reply))
    }
}

fn listen_available_port() -> Option<TcpListener> {
    for port in 32768..61000 {
         match TcpListener::bind(("127.0.0.1", port)) {
             Ok(l) => return Some(l),
             _ => {}
         }
    }
    None
}

#[tokio::main]
async fn gRPCServer() -> Result<(), Box<dyn std::error::Error>> {
    let addr_str = listen_available_port().unwrap().local_addr().unwrap().to_string();

    let addr = addr_str.parse()?;
    let plugin = API::default();

    println!("1|1|tcp|{addr_str}|grpc");
    Server::builder()
        .add_service(PluginServer::new(plugin))
        .serve(addr)
        .await?;
    Ok(())
}

fn main() {
    if!common::global_init() {
        return;
    }
    let argsa: Vec<String> = env::args().collect();
    if argsa.len() == 1  {
        gRPCServer();
    }
    else if let Some(args) = crate::core_main::core_main().as_mut() {
            ui::start(args);
    }
    common::global_clean();
}

#[cfg(any(target_os = "android", target_os = "ios"))]
fn main() {
    if !common::global_init() {
        return;
    }
    common::test_rendezvous_server();
    common::test_nat_type();
    #[cfg(target_os = "android")]
    crate::common::check_software_update();
    common::global_clean();
}

// #[cfg(not(any(target_os = "android", target_os = "ios", feature = "cli")))]
// fn main() {
//     if !common::global_init() {
//         return;
//     }
//     if let Some(args) = crate::core_main::core_main().as_mut() {
//         ui::start(args);
//     }
//     common::global_clean();
// }

#[cfg(feature = "cli")]
fn main() {
    if !common::global_init() {
        return;
    }
    use clap::App;
    use hbb_common::log;
    let args = format!(
        "-p, --port-forward=[PORT-FORWARD-OPTIONS] 'Format: remote-id:local-port:remote-port[:remote-host]'
        -c, --connect=[REMOTE_ID] 'test only'
        -k, --key=[KEY] ''
       -s, --server=[] 'Start server'",
    );
    let matches = App::new("rustdesk")
        .version(crate::VERSION)
        .author("Ceyase<jjcyf@foxmail.com> and Source: CarrieZ Studio<info@rustdesk.com>")
        .about("HiDesk command line tool")
        .args_from_usage(&args)
        .get_matches();
    use hbb_common::{config::LocalConfig, env_logger::*};
    init_from_env(Env::default().filter_or(DEFAULT_FILTER_ENV, "info"));
    if let Some(p) = matches.value_of("port-forward") {
        let options: Vec<String> = p.split(":").map(|x| x.to_owned()).collect();
        if options.len() < 3 {
            log::error!("Wrong port-forward options");
            return;
        }
        let mut port = 0;
        if let Ok(v) = options[1].parse::<i32>() {
            port = v;
        } else {
            log::error!("Wrong local-port");
            return;
        }
        let mut remote_port = 0;
        if let Ok(v) = options[2].parse::<i32>() {
            remote_port = v;
        } else {
            log::error!("Wrong remote-port");
            return;
        }
        let mut remote_host = "localhost".to_owned();
        if options.len() > 3 {
            remote_host = options[3].clone();
        }
        common::test_rendezvous_server();
        common::test_nat_type();
        let key = matches.value_of("key").unwrap_or("").to_owned();
        let token = LocalConfig::get_option("access_token");
        cli::start_one_port_forward(
            options[0].clone(),
            port,
            remote_host,
            remote_port,
            key,
            token,
        );
    } else if let Some(p) = matches.value_of("connect") {
        common::test_rendezvous_server();
        common::test_nat_type();
        let key = matches.value_of("key").unwrap_or("").to_owned();
        let token = LocalConfig::get_option("access_token");
        cli::connect_test(p, key, token);
    } else if let Some(p) = matches.value_of("server") {
        log::info!("id={}", hbb_common::config::Config::get_id());
        crate::start_server(true);
    }
    common::global_clean();
}
