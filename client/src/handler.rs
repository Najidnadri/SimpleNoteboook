use std::{net::{TcpStream, Shutdown}, io::{BufWriter, Read, Write}};

use eframe::epi::App;
use egui::Color32;
use serde::{Deserialize, Serialize};
use crate::{regpage::PassStatus, homepage::home_page};
use crate::{error::{LoginError, RegisterError}, loginpage, regpage};

pub const LIGHT_YELLOW: Color32 = Color32::from_rgb(255, 255, 0xE0);
pub const GRAY: Color32 = Color32::from_rgb(160, 160, 160);
pub const RED: Color32 = Color32::from_rgb(255, 0, 0);
pub const GREEN: Color32 = Color32::from_rgb(0, 255, 0);
pub const LIGHT_RED: Color32 = Color32::from_rgb(255, 128, 128);

pub enum Page {
    LoginPage,
    ErrLoginPage,
    RegistrationPage(bool, bool, PassStatus, bool),
    HomePage,
    RegisteredPage,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum ServerResponse {
    AccountValidated(String),
    LoginError(LoginError),
    AccountRegistered,
    RegErr(RegisterError),
    SavedPage,
    SavedPageErr,
    Err,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ChunkDetails {
    pub account: String,
    pub data: String,
}

pub struct Event {
    pub user: String,
    pub msg: String,
    pub page: Page,
    pub reg_info: RegisterInfo,
    pub login_info: LoginInfo,
    pub stream: TcpStream,
    pub data: String,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct LoginInfo {
    pub username_email: String,
    pub pass: String,
}

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct RegisterInfo {
    pub username: String,
    pub email: String,
    pub password: String,
    pub confirm_pass: String,
}

impl Event {
    pub fn new(stream: TcpStream) -> Self {
        Event {
            user: String::default(),
            msg: String::from("Username or email"),
            page: Page::LoginPage,
            reg_info: RegisterInfo::default(),
            login_info: LoginInfo::default(),
            stream,
            data: String::default(),
        }
    }
}


impl App for Event {
    fn update(&mut self, ctx: &egui::CtxRef, _frame: &mut eframe::epi::Frame<'_>) {
        match self.page {
            Page::LoginPage => loginpage::login_page(self, ctx),
            Page::RegistrationPage(a, b, c, d) => regpage::registration_page(self, ctx, a, b, c, d),
            Page::RegisteredPage =>  regpage::registered_page(self, ctx),
            Page::ErrLoginPage => loginpage::err_login_page(self, ctx),
            Page::HomePage => {
                home_page(self, ctx)
            }
        }
    }

    fn name(&self) -> &str {
        "Simple Note Book"
    }
}

pub fn send_request(action: String, mut stream: &TcpStream) -> ServerResponse {
    let mut writer = BufWriter::new(stream);
    writer.write_all(action.as_bytes()).expect("could not write");
    writer.flush().expect("cannot flush");
    println!("After write");

    //read from server
    let mut data = [0 as u8; 1000]; // using 50 byte buffer
    match stream.read(&mut data) {
        Ok(size) => {
            let response = eliminate_zeros(data);
            println!("msg received from server: {} with size {}", response, size);
            let deserialized_response: ServerResponse = serde_json::from_str(&response).expect("cannot deserialized");
            return deserialized_response
        },
        Err(_) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
        }
    } 
    ServerResponse::Err
}

pub fn eliminate_zeros(data: [u8; 1000]) -> String {
    let mut new_data: Vec<u8> = Vec::new();
    for i in data {
        if i == 0 {
            break;
        } else {
            new_data.push(i);
        }
    }
    String::from_utf8(new_data).unwrap()
}