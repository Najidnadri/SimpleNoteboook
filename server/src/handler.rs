use crate::{RegisterError, error::{LoginError, AppError}};

use std::{io::{Write, BufWriter, BufReader, BufRead, Read}, fs::File};

use bcrypt::{self, verify};
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

#[derive(Deserialize, Serialize, Debug)]
pub struct LoginInfo {
    pub username_email: String,
    pub pass: String,
}

impl RegisterInfo {
    pub fn register_account(&self) -> Result<(), RegisterError> {
        let file = std::fs::OpenOptions::new().read(true).write(true).append(true).open("users.txt").expect("cannot open users file");
        let users = users_collection(&file);
        match self.check_existed(users) {
            Ok(_) => {
                let pass = &self.password;   
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
            },
            Err(e) => {
                return Err(e);
            } 
        }

        let new_file = std::fs::File::create(format!("{}.txt", self.username.clone())).unwrap();
        Ok(())
    }

    fn check_existed(&self, users: Vec<User>) -> Result<(), RegisterError> {
        let mut username_existed = false;
        let mut email_taken = false;

        for i in users {
            if i.username == self.username {
                username_existed = true;
            }
    
            if i.email == self.email {
                email_taken = true;
            }
        }

        if username_existed || email_taken {
            let err = RegisterError {
                username_existed,
                email_taken,
            };
            return Err(err)
        }
        Ok(())
    }
}

impl LoginInfo {
    pub fn validate_account(self) -> Result<(), LoginError> {
        let file = std::fs::File::open("users.txt").expect("cannot open users.txt when validating");
        let users = users_collection(&file);
    
        for i in users {
            if i.username == self.username_email || i.email == self.username_email {
                let valid = verify(&self.pass, &i.hash).expect("something wrong in bycrpt");
                if valid {
                    return Ok(())
                } else {
                    return Err(LoginError::WrongPassword("Wrong Password!".to_string()))
                }
            } else {
                continue
            }
        }
        Err(LoginError::UsernameErr("username or does not exist".to_string()))
    }
}

pub fn users_collection(file: &File) -> Vec<User> {
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

pub fn fetch_note(user: String) -> String {
    let f = std::fs::File::open(format!("{}.txt", user)).unwrap();
    let mut reader = BufReader::new(f);
    let mut buff = [0u8; 3000];
    let _readed = reader.read(&mut buff).unwrap();
    String::from_utf8_lossy(&buff).to_string()
}
/* 
fn check_password_secure(pass: &str) -> Result<(), RegisterError> {
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
        return Ok(())
    } else {
        return Err(RegisterError::PasswordInsecure("Your password is a hacker's breakfast, make it more secure!".to_string()))
    }
}

pub fn filter_error(err: AppError) {
    match err {
        AppError::LoginError(e) => {
            match e {
                LoginError::UsernameErr(s) => println!("{}", s),
                LoginError::WrongPassword(s) => println!("{}", s),
            }
        },
        AppError::RegisterError(e) => {
            match e {
                RegisterError::EmailTaken(s) => println!("{}", s),
                RegisterError::PasswordInsecure(s) => println!("{}", s),
                RegisterError::UsernameExisted(s) => println!("{}", s),
            }
        }
    }
}
*/