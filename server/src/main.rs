mod error;
mod handler;


use error::RegisterError;
use handler::{RegisterInfo, User, LoginInfo, filter_error};

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
    let demo_user = RegisterInfo {
        username: "Najidnadri".to_string(),
        email: "mohd.najid.nadri@gmail.com".to_string(),
        password: "M_uhd_na_jid29".to_string(),
    };
    let demo_request = Action::RegisterAccount(demo_user);

    match demo_request {
        Action::ValidateAccount(_user) => {},
        Action::RegisterAccount(user) => {
            println!("registering user");
            let registering = user.register_account();
            match registering {
                Ok(_) => println!("User Registered!"),
                Err(e) => {
                    filter_error(e);
                }
            }
        },
        Action::SavePage => {},
    }
}

fn _validate_account(_user: User) {

}

fn _save_page() {

}