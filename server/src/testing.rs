
#[test]
fn add_user() {
    use crate::handler::RegisterInfo;

    //put your own details
    let register_info = RegisterInfo {
        username: "Muhdnajid".to_string(),
        email: "muhd.najid.nadri@gmail.com".to_string(),
        password: "M_uhd_na_jid29".to_string(),
    };
    let register_response = register_info.register_account().unwrap();
    assert_eq!(register_response, ())
}

#[test] 
fn validate_user() {
    use crate::handler::LoginInfo;

    //put your own details again
    let login_info = LoginInfo {
        username_email: "Muhdnajid".to_string(),
        pass: "M_uhd_na_jid29".to_string(),
    };

    let login_response = login_info.validate_account().unwrap();
    assert_eq!(login_response, ())
}

#[tokio::test]
async fn save_data() {
    use crate::ChunkDetails;
    use crate::save_page;

    let chunk = ChunkDetails {
        //chunknum: 0,
        account: "Muhdnajid".to_string(),
        data: "Hello my name is najid".to_string(),
    };

    match save_page(chunk).await {
    Ok(_) => {
        println!("page saved!");
    },
    Err(e) => {
        println!("error saving page: {:?}", e);
    },
};
}