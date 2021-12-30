struct User {
    username: String,
    email: String,
    password: String,
}

enum Action {
    ValidateAccount(User),
    RegisterAccount(User),
    SavePage,
}

enum ServerResponse {
    AccountValidated,
    AccountRegistered,
    SavedPage,
}

fn main() {
    let demo_user = User {
        username: "Najidnadri".to_string(),
        email: "muhd.najid.nadri@gmail.com".to_string(),
        password: "najidnadri".to_string(),
    };
    let demo_request = Action::RegisterAccount(demo_user);

    match demo_request {
        Action::ValidateAccount(user) => {},
        Action::RegisterAccount(user) => {
            println!("register user")
        },
        Action::SavePage => {},
    }
}

fn register_account(user: User) {

}

fn validate_account(user: User) {

}

fn save_page() {

}
