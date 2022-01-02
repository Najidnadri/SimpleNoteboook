//setup client
mod handler;
use handler::{Event, RegisterInfo, LoginInfo};

use serde::{self, Deserialize, Serialize};

use std::{error::Error, net::TcpStream};
use egui::Vec2;
use eframe::{NativeOptions, run_native};
use tokio;

#[derive(Debug, Deserialize, Serialize)]
enum Action {
    ValidateAccount(LoginInfo),
    RegisterAccount(RegisterInfo),
    SavePage,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let connection = TcpStream::connect("127.0.0.1:8000").unwrap();

    let app = Event::new(connection.try_clone().expect("cannot clone tcpstream"));

    let mut native_option = NativeOptions::default();
    native_option.initial_window_size = std::option::Option::Some(Vec2 { x: 1000., y: 800. });
    native_option.resizable = false;

    println!("after select macro");
    run_native(Box::new(app), native_option);
    
    Ok(())
}
