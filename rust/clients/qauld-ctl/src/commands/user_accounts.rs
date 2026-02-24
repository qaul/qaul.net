use prost::Message;

use crate::{cli::AccountSubcmd, commands::RpcCommand, proto::Modules};

mod proto {
    include!("../../../../libqaul/src/rpc/protobuf_generated/rust/qaul.rpc.user_accounts.rs");
}

use proto::{user_accounts, CreateUserAccount, SetPasswordRequest, UserAccounts};

/// Decodes a `GetDefaultUserAccount` response payload and returns the user's raw id bytes.
/// Returns an empty Vec if the account doesn't exist or decoding fails.
pub fn decode_default_user(data: &[u8]) -> Vec<u8> {
    match UserAccounts::decode(data) {
        Ok(ua) => match ua.message {
            Some(proto::user_accounts::Message::DefaultUserAccount(default_useraccount)) => {
                if default_useraccount.user_account_exists {
                    if let Some(acct) = default_useraccount.my_user_account {
                        return acct.id;
                    }
                }
                log::warn!("preflight: no default user account found");
                Vec::new()
            }
            _ => {
                log::warn!("preflight: unexpected user_accounts message type");
                Vec::new()
            }
        },
        Err(e) => {
            log::warn!("preflight: failed to decode user_accounts response: {e}");
            Vec::new()
        }
    }
}

pub fn default_user_proto_message() -> (Vec<u8>, Modules) {
    let proto_message = UserAccounts {
        message: Some(user_accounts::Message::GetDefaultUserAccount(true)),
    };

    // encode message
    let mut buf = Vec::with_capacity(proto_message.encoded_len());
    proto_message
        .encode(&mut buf)
        .expect("Vec<u8> provides capacity as needed");
    (buf, Modules::Useraccounts)
}

impl RpcCommand for AccountSubcmd {
    fn encode_request(&self) -> Result<(Vec<u8>, Modules), Box<dyn std::error::Error>> {
        match &self {
            AccountSubcmd::Default => {
                let res = default_user_proto_message();
                Ok(res)
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
            AccountSubcmd::Password { password } => {
                let proto_message = UserAccounts {
                    message: Some(user_accounts::Message::SetPasswordRequest(
                        SetPasswordRequest {
                            password: Some(password.to_string()),
                        },
                    )),
                };

                let mut buf = Vec::with_capacity(proto_message.encoded_len());
                proto_message.encode(&mut buf).unwrap();
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
            Some(user_accounts::Message::MyUserAccount(acct)) => {
                println!("New user account created:");
                println!("{}, ID[{}]", acct.name, acct.id_base58);
                println!("    public key: {}", acct.key_base58);
            }
            Some(user_accounts::Message::SetPasswordResponse(response)) => {
                if response.success {
                    println!(" Password updated");
                } else {
                    println!("{}", response.error_message);
                }
            }
            _ => {
                log::error!("unprocessable RPC user accounts message");
            }
        };
        Ok(())
    }
}
