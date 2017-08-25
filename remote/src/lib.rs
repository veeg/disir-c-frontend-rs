extern crate disir_c;
extern crate protobuf;

use disir_c::{Instance};
use protobuf::{CodedInputStream, CodedOutputStream, Message, ProtobufError, ProtobufResult};

mod disirpb;
use disirpb::*;

mod query;


pub struct Client {
    instance: disir_c::Instance,
}

impl Client {

    pub fn new() -> Client {
        Client { instance: Default::default() }
    }

    // TODO: Should we maybe not return ProtobufError?
    // XXX: Should find a way to return a future
    pub fn process(&self, input_message: Vec<u8>) -> Result<Vec<u8>, ProtobufError> {

        let mut cis = protobuf::CodedInputStream::from_bytes(&input_message);
        let mut from_client = FromClient::new();

        // TODO: Catch protobuf::ProtobufResult
        from_client.merge_from(&mut cis)?;

        println!("Received fromclient message: id={}", from_client.get_command_identifier());


        // Prepare response to client
        let mut to_client = ToClient::new();
        to_client.set_command_identifier(from_client.get_command_identifier());

        let mut output_message: Vec<u8> = Vec::new();
        {
            let mut cos = CodedOutputStream::vec(&mut output_message);

            // TODO: Invoke the sub-operation to respond


            to_client.write_to(&mut cos)?;
            cos.flush()?;
        }

        Ok((output_message))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn process() {
        let mut client = Client::new();

        let res = client.process();
        match res {
            Ok(v) => {
                let cis = CodedInputStream::new(v);
            },
            Err(e) => println!("err {:?}", e),
        }
    }
}
