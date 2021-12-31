use crate::RegisterError;

use std::{io::{Write, BufWriter, BufReader, BufRead}, fs::File};

use bcrypt;
use serde::{self, Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct RegisterInfo {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize,Serialize, Debug)]
pub struct User {
    pub username: String,
    pub email: String,
    pub hash: String,
}

pub struct LoginInfo {
    pub username: String,
    pub pass: String,
}

impl RegisterInfo {
    pub fn register_account(&self) -> Result<(), RegisterError> {
        let file = std::fs::OpenOptions::new().read(true).write(true).append(true).open("users.txt").expect("cannot open users file");
        let users = users_collection(&file);
        match self.check_existed(users) {
            Ok(_) => {
                let pass = &self.password;
                if check_password_secure(pass) {
                    let hash = bcrypt::hash(pass, bcrypt::DEFAULT_COST).expect("cannot brcypt pass");
                    let user = User {
                        username: self.username.clone(),
                        email: self.email.clone(),
                        hash,
                    };
                    let serialized_user = serde_json::to_string(&user).expect("cannot serialize user");
                    
                    //write to file
                    let mut writer = BufWriter::new(file);
                    writeln!(writer, "{}", serialized_user).expect("cannot write to users.txt");
                }
            },
            Err(e) => {
                return Err(e);
            } 
        }

        Ok(())
    }

    fn check_existed(&self, users: Vec<User>) -> Result<(), RegisterError> {
        for i in users {
            if i.username == self.username {
                return Err(RegisterError::UsernameExisted("username existed".to_string()));
            }
    
            if i.email == self.email {
                return Err(RegisterError::EmailTaken("Email taken".to_string()));
            }
        }
        Ok(())
    }
}

fn users_collection(file: &File) -> Vec<User> {
    let reader = BufReader::new(file);
    let lines = reader.lines();
    let mut users = Vec::new();

    for i in lines {
        if let Ok(s) = i {
            let deserialized_users: User = serde_json::from_str(&s).expect("cannot deserialize users");
            users.push(deserialized_users);
        }
    }
    users
}

fn check_password_secure(pass: &str) -> bool {
    let mut number = false;
    let mut other_char = false;
    let mut uppercase = false;

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
    }
    
    if number == true && other_char == true && uppercase == true {
        return true
    } else {
        return false
    }
}

pub fn filter_error(err: RegisterError) {
    match err {
        RegisterError::EmailTaken(s) => println!("{}", s),
        RegisterError::PasswordInsecure(s) => println!("{}", s),
        RegisterError::UsernameExisted(s) => println!("{}", s),
    }
}