use egui::{Visuals, Rect, Pos2, Vec2};

use crate::{handler::{Event, GRAY, LIGHT_YELLOW, ServerResponse, send_request, Page, RED, GREEN, LIGHT_RED}, Action};

#[derive(Clone, Copy, PartialEq)]
pub enum PassStatus {
    TooShort,
    UniqueCharMissing,
    NumberMissing,
    UppercaseMissing,
    Good,
    Zero,
}

pub fn registration_page(event: &mut Event, ctx: &egui::CtxRef, usernameerr: bool, emailtaken: bool, passerr: PassStatus, confirmation: bool) {
    event.msg = "Please input the registration details".to_string();
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
                ui.heading(event.msg.clone());
                ui.add_space(20.0);
                ui.label("username");
                if usernameerr {
                    ui.colored_label(RED, "USERNAME EXISTED");
                }
                let _username = ui.add(egui::TextEdit::singleline(&mut event.reg_info.username));
                ui.add_space(10.0);
                ui.label("email");
                if emailtaken {
                    ui.colored_label(RED, "EMAIL TAKEN");
                }
                let _email = ui.add(egui::TextEdit::singleline(&mut event.reg_info.email));
                ui.add_space(10.0);
                ui.label("password");
                match passerr {
                    PassStatus::TooShort => {
                        ui.colored_label(RED, "The password is too short!");
                        ui.colored_label(LIGHT_RED, "Please include special char, number and uppercase letter in your password");
                    },
                    PassStatus::UniqueCharMissing => {
                        ui.colored_label(RED, "Missing unique char");
                        ui.colored_label(LIGHT_RED, "Please include special char, number and uppercase letter in your password");
                    },
                    PassStatus::NumberMissing => {
                        ui.colored_label(RED, "Number char missing");
                        ui.colored_label(LIGHT_RED, "Please include special char, number and uppercase letter in your password");
                    }
                    PassStatus::UppercaseMissing => {
                        ui.colored_label(RED, "uppercase char missing");
                        ui.colored_label(LIGHT_RED, "Please include special char, number and uppercase letter in your password");
                    },
                    PassStatus::Good => {
                        ui.colored_label(GREEN, "Now this is what I called a strong password");
                    }
                    PassStatus::Zero => (),
                }
                let pass = ui.add(egui::TextEdit::singleline(&mut event.reg_info.password).password(true));
                ui.add_space(10.0);
                ui.label("Password confirmation");
                if confirmation {
                    ui.colored_label(RED, "The passwords are not the same!!");
                }
                let confirm_pass = ui.add(egui::TextEdit::singleline(&mut event.reg_info.confirm_pass).password(true));
                ui.add_space(10.0);
                if passerr == PassStatus::Good && 
                confirmation == false {
                    let reg_button = ui.add_enabled(true, egui::Button::new("Register Now!"));
                    if reg_button.clicked() {
                        event.msg = "Registering..".to_string();
                        let action = Action::RegisterAccount(event.reg_info.clone());
                        let serialized_action = serde_json::to_string(&action).expect("cannot serialized reg action");
                        match send_request(serialized_action, &event.stream) {
                            ServerResponse::AccountRegistered => {
                                event.msg = "REGISTERED!!".to_string();
                                event.reg_info.username = String::default();
                                event.reg_info.email = String::default();
                                event.reg_info.password = String::default();
                                event.page = Page::RegisteredPage;
                            },
                            ServerResponse::RegErr(e) => {
                                event.page = Page::RegistrationPage(e.username_existed, e.email_taken, passerr, confirmation)
                            }
                            _ => {},
                        }
                    }
                } else {
                    let _reg_button = ui.add_enabled(false, egui::Button::new("Register Now!"));
                }
                ui.add_space(10.0);
                if ui.button("Back").clicked() {
                    event.page = Page::LoginPage;
                }

                if pass.changed() {
                    match check_password_secure(&event.reg_info.password) {
                        PassStatus::TooShort => {
                            event.page = Page::RegistrationPage(usernameerr, emailtaken, PassStatus::TooShort, confirmation)
                        },
                        PassStatus::UniqueCharMissing => {
                            event.page = Page::RegistrationPage(usernameerr, emailtaken, PassStatus::UniqueCharMissing, confirmation)
                        },
                        PassStatus::NumberMissing => {
                            event.page = Page::RegistrationPage(usernameerr, emailtaken, PassStatus::NumberMissing, confirmation)
                        }
                        PassStatus::UppercaseMissing => {
                            event.page = Page::RegistrationPage(usernameerr, emailtaken, PassStatus::UppercaseMissing, confirmation)
                        },
                        PassStatus::Good => {
                            event.page = Page::RegistrationPage(usernameerr, emailtaken, PassStatus::Good, confirmation)
                        }
                        PassStatus::Zero => {
                            event.page = Page::RegistrationPage(usernameerr, emailtaken, PassStatus::Zero, confirmation)
                        }
                    }
                }

                if confirm_pass.changed() {
                    let pass = event.reg_info.password.clone();
                    let confirmation = event.reg_info.confirm_pass.clone();
                    if pass == confirmation {
                        event.page = Page::RegistrationPage(usernameerr, emailtaken, passerr, false);
                    } else {
                        event.page = Page::RegistrationPage(usernameerr, emailtaken, passerr, true);
                    }
                }


            });
        });
    });
}

pub fn err_reg_page(event: &mut Event, ctx: &egui::CtxRef) {

}


pub fn registered_page(event: &mut Event, ctx: &egui::CtxRef) {
    event.msg = "CONGRATS MY BOY YOU ARE REGISTERED!!".to_string();
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
                ui.heading(event.msg.clone());
                ui.add_space(20.0);
                let login_button = ui.button("Continue to the login page");
                if login_button.clicked() {
                    event.page = Page::LoginPage;
                }
            });
        });
    });
}

fn check_password_secure(pass: &str) -> PassStatus {
    let mut number = false;
    let mut other_char = false;
    let mut uppercase = false;
    let mut amount = 0;

    for i in pass.chars() {
        if i.is_ascii_uppercase() {
            uppercase = true
        }

        match i {
            '1' => number = true,
            '2' => number = true,
            '3' => number = true,
            '4' => number = true,
            '5' => number = true,
            '6' => number = true,
            '7' => number = true,
            '8' => number = true,
            '9' => number = true,
            '_' => other_char = true,
            '@' => other_char = true,
            '!' => other_char = true,
            '#' => other_char = true,
            '%' => other_char = true,
            '&' => other_char = true,
            '*' => other_char = true,
            '^' => other_char = true,
            _ => {}
        }

        amount += 1;
    }

    if number == false {
        return PassStatus::NumberMissing;
    } else if other_char == false {
        return PassStatus::UniqueCharMissing;
    } else if uppercase == false {
        return PassStatus::UppercaseMissing;
    } else if amount < 6 {
        return PassStatus::TooShort;
    } else {
        return PassStatus::Good;
    }
    
}