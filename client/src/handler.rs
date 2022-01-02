use std::{net::{TcpStream, Shutdown}, io::{BufWriter, Read, Write}};

use eframe::epi::App;
use egui::{Vec2, Rect, Pos2, Color32, Visuals};
use serde::{Deserialize, Serialize};

use crate::Action;

pub const LIGHT_YELLOW: Color32 = Color32::from_rgb(255, 255, 0xE0);
pub const GRAY: Color32 = Color32::from_rgb(160, 160, 160);

pub enum Page {
    LoginPage,
    RegistrationPage,
    HomePage,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum ServerResponse {
    AccountValidated,
    AccountRegistered,
    SavedPage,
    Err,
}

pub struct Event {
    pub msg: String,
    pub page: Page,
    pub reg_info: RegisterInfo,
    pub login_info: LoginInfo,
    pub stream: TcpStream,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct LoginInfo {
    pub username_email: String,
    pub pass: String,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct RegisterInfo {
    pub username: String,
    pub email: String,
    pub password: String,
}

impl Event {
    pub fn new(stream: TcpStream) -> Self {
        Event {
            msg: String::from("Username or email"),
            page: Page::LoginPage,
            reg_info: RegisterInfo::default(),
            login_info: LoginInfo::default(),
            stream,
        }
    }
}


impl App for Event {
    fn update(&mut self, ctx: &egui::CtxRef, _frame: &mut eframe::epi::Frame<'_>) {
        match self.page {
            Page::LoginPage => login_page(self, ctx),
            _ => {}
        }
    }

    fn name(&self) -> &str {
        "Simple Note Book"
    }
}

fn login_page(event: &mut Event, ctx: &egui::CtxRef) {

    let mut visuals = Visuals::default();
    visuals.faint_bg_color = GRAY;
    visuals.dark_mode = false;
    visuals.override_text_color = Some(LIGHT_YELLOW);
    
    let login_card = Rect::from_center_size(Pos2::new(500.0, 600.0), Vec2::new(300.0, 500.0));

    egui::TopBottomPanel::top("header").default_height(100.).show(ctx, |ui| {
        ui.add_sized([100.0, 100.0],  egui::Label::new("My simple notebook").underline()); 
    });

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.allocate_ui_at_rect(login_card, |ui| {
                ui.label(event.msg.clone());
                let _username_email = ui.add(egui::TextEdit::singleline(&mut event.login_info.username_email));
                let _pass = ui.add(egui::TextEdit::singleline(&mut event.login_info.pass).password(true));
                let login_button = ui.button("Log In");
                if login_button.clicked() {
                    let action = Action::ValidateAccount(event.login_info.clone());
                    let serialized_action = serde_json::to_string(&action).expect("cannot serialized action");
                    make_client_request(event, serialized_action)
                }
            });
        });
    });
}

fn make_client_request(event: &mut Event, action: String) {

    let response = send_request(action, &event.stream);

    match response {
        ServerResponse::AccountValidated => event.msg = "Welcome!!".to_string(),
        ServerResponse::AccountRegistered => todo!(),
        ServerResponse::SavedPage => todo!(),
        ServerResponse::Err => todo!(),
    }
}

fn send_request(action: String, mut stream: &TcpStream) -> ServerResponse {
    let mut writer = BufWriter::new(stream);
    writer.write_all(action.as_bytes()).expect("could not write");
    writer.flush().expect("cannot flush");
    println!("After write");

    //read from server
    let mut data = [0 as u8; 50]; // using 50 byte buffer
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

fn eliminate_zeros(data: [u8; 50]) -> String {
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