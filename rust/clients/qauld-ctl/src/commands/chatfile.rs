use super::{id_string_to_bin, uuid_string_to_bin};
use crate::{cli::ChatFileSubcmd, commands::RpcCommand, proto::Modules};
use prost::Message;

/// include generated protobuf RPC rust definition file
mod proto {
    include!("../../../../libqaul/src/rpc/protobuf_generated/rust/qaul.rpc.chatfile.rs");
}

use proto::{chat_file, ChatFile, FileHistoryRequest, SendFileRequest};

impl RpcCommand for ChatFileSubcmd {
    fn encode_request(&self) -> Result<(Vec<u8>, Modules), Box<dyn std::error::Error>> {
        match &self {
            ChatFileSubcmd::Send {
                group_id,
                file,
                description,
            } => {
                let group_id_bytes = id_string_to_bin(group_id.to_string())
                    .or(uuid_string_to_bin(group_id.to_string()))?;
                log::trace!(
                    "send file to group={}, file path={}, description={}",
                    group_id,
                    file,
                    description
                );

                let proto_message = ChatFile {
                    message: Some(chat_file::Message::SendFileRequest(SendFileRequest {
                        path_name: file.clone(),
                        group_id: group_id_bytes.clone(),
                        description: description.clone(),
                    })),
                };

                Ok((proto_message.encode_to_vec(), Modules::Chatfile))
            }
            ChatFileSubcmd::History { offset, limit } => {
                let proto_message = ChatFile {
                    message: Some(chat_file::Message::FileHistory(FileHistoryRequest {
                        offset: *offset,
                        limit: *limit,
                    })),
                };
                Ok((proto_message.encode_to_vec(), Modules::Chatfile))
            }
        }
    }

    fn decode_response(&self, data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        let chat_file = ChatFile::decode(data)?;
        match chat_file.message {
            Some(chat_file::Message::FileHistoryResponse(proto_file_history)) => {
                // List header
                println!("====================================");
                println!("File Sharing Histories");
                println!("------------------------------------");
                println!(
                    "offset={}, limit={}, total={}",
                    proto_file_history.offset, proto_file_history.limit, proto_file_history.total
                );

                // print all messages in the feed list
                for entry in proto_file_history.histories {
                    println!("[{}] - {}", entry.file_id, entry.file_name);
                    println!("\t Time: {}, SenderId: {}", entry.time, entry.sender_id);
                    println!("\t Group Id: {}", entry.group_id);
                    println!(
                        "\t FileSize: {}, Description: {}",
                        entry.file_size, entry.file_description
                    );
                    println!("");
                }
            }
            _ => {
                log::error!("unprocessable RPC file message");
            }
        };
        Ok(())
    }
}
