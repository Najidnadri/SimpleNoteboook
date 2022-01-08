mod error;
mod handler;
mod testing;


use std::{net::{TcpListener, Shutdown, TcpStream}, io::{Read, BufWriter, Write}, thread};

use error::{RegisterError, LoginError, SaveError};
use handler::{RegisterInfo, LoginInfo};
use serde::{Deserialize, Serialize};
use tokio;

use crate::handler::fetch_note;

#[derive(Deserialize, Serialize, Debug)]
struct ChunkDetails {
   // chunknum: usize,
    account: String,
    data: String,
}

#[derive(Deserialize, Serialize, Debug)]
enum Action {
    ValidateAccount(LoginInfo),
    RegisterAccount(RegisterInfo),
    SavePage(ChunkDetails),
}

#[derive(Deserialize, Serialize, Debug)]
enum ServerResponse {
    AccountValidated(String),
    LoginError(LoginError),
    AccountRegistered,
    RegErr(RegisterError),
    SavedPage,
    SavedPageErr,
    Err,
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();

    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            handle_client(stream).await
        }
    }
}

async fn handle_client(stream: TcpStream) {
    tokio::spawn(async move {
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
                            let username = user.username_email.clone();
                            let validation = user.validate_account();
                            match validation {
                                Ok(_) => {
                                    let mut file = std::fs::File::open(format!("{}.txt", username)).unwrap();
                                    let mut buffer = String::new();
                                    let _readed = file.read_to_string(&mut buffer).unwrap();
                                    println!("note: {}", buffer);
                                    let response = ServerResponse::AccountValidated(buffer);
                                    println!("response: {:?}", response);
                                    send_response(response, &stream);
                                },
                                Err(e) => {
                                    let response = ServerResponse::LoginError(e);
                                    send_response(response, &stream);
                                },
                            }
                        },
                        Action::RegisterAccount(user) => {
                            println!("registering user");
                            let registering = user.register_account();
                            match registering {
                                Ok(_) => {
                                    let response = ServerResponse::AccountRegistered;
                                    send_response(response, &stream);
                                    println!("User Registered!")
                                },
                                Err(e) => {
                                    let response = ServerResponse::RegErr(e);
                                    send_response(response, &stream);
                                }
                            }
                        },
                        Action::SavePage(s) => {
                            println!("saving page");
                            match save_page(s).await {
                                Ok(_) => {
                                    let response = ServerResponse::SavedPage;
                                    send_response(response, &stream);
                                    println!("page saved");
                                },
                                Err(e) => {
                                    let response = ServerResponse::SavedPageErr;
                                    send_response(response, &stream);
                                    println!("error while saving page: {:?}", e)
                                },
                            };
                        },
                    }
                },
                Err(_) => {
                    println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
                    stream.shutdown(Shutdown::Both).unwrap();
                }
            }{}
        }
    });  
}

async fn save_page(chunkdetails: ChunkDetails) -> Result<(), SaveError> {
    let mut file = std::fs::OpenOptions::new().write(true).read(true).open(format!("{}.txt", &chunkdetails.account))
    .map_err(|_e| SaveError::CannotOpenfile)
    .unwrap();

    file.set_len(0).map_err(|_e| SaveError::CannotTruncate).unwrap();
    file.write_all(chunkdetails.data.as_bytes()).map_err(|_e| SaveError::CannotWrite).unwrap();

    Ok(())
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