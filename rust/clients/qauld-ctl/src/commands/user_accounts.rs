use prost::Message;

use crate::{
    cli::AccountSubcmd,
    commands::RpcCommand,
    proto::{user_accounts, CreateUserAccount, Modules, UserAccounts},
};

impl RpcCommand for AccountSubcmd {
    fn encode_request(&self) -> Result<(Vec<u8>, Modules), Box<dyn std::error::Error>> {
        match &self {
            AccountSubcmd::Default => {
                let proto_message = UserAccounts {
                    message: Some(user_accounts::Message::GetDefaultUserAccount(true)),
                };

                // encode message
                let mut buf = Vec::with_capacity(proto_message.encoded_len());
                proto_message
                    .encode(&mut buf)
                    .expect("Vec<u8> provides capacity as needed");

                Ok((buf, Modules::Useraccounts))
            }
            AccountSubcmd::Create { username, password } => {
                let proto_message = UserAccounts {
                    message: Some(user_accounts::Message::CreateUserAccount(
                        CreateUserAccount {
                            name: username.to_string(),
                            password: password.as_ref().map(|p| p.to_string()),
                        },
                    )),
                };

                let mut buf = Vec::with_capacity(proto_message.encoded_len());
                proto_message
                    .encode(&mut buf)
                    .expect("Vec<u8> provides capacity as needed");

                Ok((buf, Modules::Useraccounts))
            }
            _ => {
                todo!()
            }
        }
    }

    fn decode_response(&self, data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        let user_accounts = UserAccounts::decode(data)?;
        match user_accounts.message {
            Some(user_accounts::Message::DefaultUserAccount(default_useraccount)) => {
                if default_useraccount.user_account_exists {
                    if let Some(my_user_account) = default_useraccount.my_user_account {
                        println!("Your user account is:");
                        println!(
                            "{}, ID[{}]",
                            my_user_account.name, my_user_account.id_base58
                        );
                        println!("    public key: {}", my_user_account.key_base58);

                        if my_user_account.has_password {
                            println!("Your password is enabled");
                        } else {
                            println!("Your password is disabled");
                        }
                    }
                } else {
                    println!("No user account created yet. Please create an account by running: qauld-ctl account create --help");
                }
            }
            _ => {}
        };
        Ok(())
    }
}
