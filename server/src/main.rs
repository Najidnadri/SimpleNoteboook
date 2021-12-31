mod error;
mod handler;


use error::RegisterError;
use handler::{RegisterInfo, LoginInfo, filter_error};

use crate::error::AppError;


enum Action {
    ValidateAccount(LoginInfo),
    RegisterAccount(RegisterInfo),
    SavePage,
}

enum ServerResponse {
    AccountValidated,
    AccountRegistered,
    SavedPage,
}

fn main() {
    let login_info = LoginInfo {
        username_email: "Najidnadri".to_string(),
        pass: "M_uhd_na_jid29".to_string(),
    };

    let demo_request = Action::ValidateAccount(login_info);

    match demo_request {
        Action::ValidateAccount(user) => {
            println!("validating account..");
            let validation = user.validate_account();
            match validation {
                Ok(_) => println!("Welcome!!"),
                Err(e) => filter_error(AppError::LoginError(e)),
            }
        },
        Action::RegisterAccount(user) => {
            println!("registering user");
            let registering = user.register_account();
            match registering {
                Ok(_) => println!("User Registered!"),
                Err(e) => {
                    filter_error(AppError::RegisterError(e));
                }
            }
        },
        Action::SavePage => {},
    }
}

fn _save_page() {

}