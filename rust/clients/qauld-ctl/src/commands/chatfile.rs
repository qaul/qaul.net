use crate::{cli::ChatFileSubcmd, commands::RpcCommand, proto::Modules};

impl RpcCommand for ChatFileSubcmd {
    fn encode_request(&self) -> Result<(Vec<u8>, Modules), Box<dyn std::error::Error>> {
        todo!()
    }

    fn decode_response(&self, data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}
