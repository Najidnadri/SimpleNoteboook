mod error;
mod handler;
mod testing;


use std::{net::{TcpListener, Shutdown, TcpStream}, io::{Read, BufWriter, Write}};

use error::RegisterError;
use handler::{RegisterInfo, LoginInfo, filter_error};
use serde::{Deserialize, Serialize};

use crate::error::AppError;

#[derive(Deserialize, Serialize, Debug)]
enum Action {
    ValidateAccount(LoginInfo),
    RegisterAccount(RegisterInfo),
    SavePage,
}

#[derive(Deserialize, Serialize, Debug)]
enum ServerResponse {
    AccountValidated,
    AccountRegistered,
    SavedPage,
    Err,
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();

    let (stream, _addr) = listener.accept().unwrap();

    loop {
        let mut stream = stream.try_clone().unwrap();
        let mut data = [0 as u8; 1000]; // using 50 byte buffer
        match stream.read(&mut data) {
            Ok(_size) => {
                let request = eliminate_zeros(data);
                let deserialized_request: Action = serde_json::from_str(&request).expect("cannot deserialzied");
                match deserialized_request {
                    Action::ValidateAccount(user) => {
                        println!("validating account..");
                        let validation = user.validate_account();
                        match validation {
                            Ok(_) => {
                                let response = ServerResponse::AccountValidated;
                                send_response(response, &stream);
                            },
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
            },
            Err(_) => {
                println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
                stream.shutdown(Shutdown::Both).unwrap();
            }
        }{}
    } 
}

fn _save_page() {

}

fn eliminate_zeros(data: [u8; 1000]) -> String {
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

fn send_response(response: ServerResponse, stream: &TcpStream) {
    let serialized_response = serde_json::to_string(&response).unwrap();

    let mut writer = BufWriter::new(stream);
    writer.write_all(serialized_response.as_bytes()).expect("could not write");
    writer.flush().expect("cannot flush");
    println!("After write");
}