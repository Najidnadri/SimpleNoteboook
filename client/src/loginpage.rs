use egui::{Style, Visuals, Rect, Pos2, Vec2};

use crate::{handler::{Event, GRAY, LIGHT_YELLOW, send_request, ServerResponse, Page}, Action, error::LoginError, regpage::PassStatus};


pub fn login_page(event: &mut Event, ctx: &egui::CtxRef) {
    event.msg = "Please input your username and password".to_string();

    let mut visuals = Visuals::default();
    visuals.faint_bg_color = GRAY;
    visuals.dark_mode = false;
    visuals.override_text_color = Some(LIGHT_YELLOW);

    //let mut style = Style::default();

    
    let login_card = Rect::from_center_size(Pos2::new(500.0, 600.0), Vec2::new(300.0, 500.0));

    egui::TopBottomPanel::top("header").default_height(100.).show(ctx, |ui| {
        ui.add_sized([100.0, 100.0],  egui::Label::new("My simple notebook").underline()); 
    });

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.allocate_ui_at_rect(login_card, |ui| {
                ui.heading(event.msg.clone());
                ui.add_space(10.0);
                ui.label("Username or email");
                let _username_email = ui.add(egui::TextEdit::singleline(&mut event.login_info.username_email));
                ui.add_space(10.0);
                ui.label("Password");
                let _pass = ui.add(egui::TextEdit::singleline(&mut event.login_info.pass).password(true));
                ui.add_space(10.0);
                let login_button = ui.button("Log In");
                let reg_button = ui.button("Dont have an account yet?");
                let shortcut = ui.button("go shortcut");
                if shortcut.clicked() {
                    event.page = Page::HomePage
                }
                if login_button.clicked() {
                    let action = Action::ValidateAccount(event.login_info.clone());
                    let serialized_action = serde_json::to_string(&action).expect("cannot serialized login action");
                    match send_request(serialized_action, &event.stream) {
                        ServerResponse::AccountValidated(s) => {
                            event.data = s;
                            event.user = event.login_info.username_email.clone();
                            event.msg = "Welcome!!".to_string();
                            event.login_info.username_email = String::default();
                            event.login_info.pass = String::default();
                            event.page = Page::HomePage;
                        },
                        ServerResponse::LoginError(e) => {
                            match e {
                                LoginError::WrongPassword(_) => {
                                    event.msg = "WRONG PASSWORD BRO!".to_string();
                                    event.page = Page::ErrLoginPage;
                                },
                                LoginError::UsernameErr(_) => {
                                    event.msg = "Cannot find username or email in our 100TB system!".to_string();
                                    event.page = Page::ErrLoginPage;
                                },
                            }
                        },
                        _ => {},
                    }
                }
                if reg_button.clicked() {
                    event.page = Page::RegistrationPage(false, false, PassStatus::Zero, false);
                }
            });
        });
    });
}

pub fn err_login_page(event: &mut Event, ctx: &egui::CtxRef) {
    let mut visuals = Visuals::default();
    visuals.faint_bg_color = GRAY;
    visuals.dark_mode = false;
    visuals.override_text_color = Some(LIGHT_YELLOW);

    //let mut style = Style::default();

    
    let login_card = Rect::from_center_size(Pos2::new(500.0, 600.0), Vec2::new(300.0, 500.0));

    egui::TopBottomPanel::top("header").default_height(100.).show(ctx, |ui| {
        ui.add_sized([100.0, 100.0],  egui::Label::new("My simple notebook").underline()); 
    });

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.allocate_ui_at_rect(login_card, |ui| {
                ui.heading(event.msg.clone());
                ui.add_space(10.0);
                ui.label("Username or email");
                let _username_email = ui.add(egui::TextEdit::singleline(&mut event.login_info.username_email));
                ui.add_space(10.0);
                ui.label("Password");
                let _pass = ui.add(egui::TextEdit::singleline(&mut event.login_info.pass).password(true));
                ui.add_space(10.0);
                let login_button = ui.button("Log In");
                let reg_button = ui.button("Dont have an account yet?");

                if login_button.clicked() {
                    let action = Action::ValidateAccount(event.login_info.clone());
                    let serialized_action = serde_json::to_string(&action).expect("cannot serialized login action");
                    match send_request(serialized_action, &event.stream) {
                        ServerResponse::AccountValidated(s) => {
                            event.data = s;
                            event.msg = "Welcome!!".to_string();
                            event.login_info.username_email = String::default();
                            event.login_info.pass = String::default();
                        },
                        ServerResponse::LoginError(e) => {
                            match e {
                                LoginError::WrongPassword(_) => {
                                    event.msg = "WRONG PASSWORD BRO!".to_string();
                                    event.page = Page::ErrLoginPage;
                                },
                                LoginError::UsernameErr(_) => {
                                    event.msg = "Cannot find username or email in our 100TB system!".to_string();
                                    event.page = Page::ErrLoginPage;
                                },
                            }
                        },
                        _ => {},
                    }
                }
                if reg_button.clicked() {
                    event.page = Page::RegistrationPage(false, false, PassStatus::Zero, false);
                }
            });
        });
    });
}