
pub mod server {
    use std::collections::HashMap;
    use std::io::{Error, ErrorKind};
    use std::os::unix::prelude::FileExt;
    
    type ConnectionMap = HashMap<u16, String>;

    struct ConnectionManager {
        connection_map: ConnectionMap,
        socket: std::net::UdpSocket
    }

    pub fn server_main() -> std::io::Result<()> {
       
        let mut connection_manager = ConnectionManager{
            connection_map: ConnectionMap::new(),
            socket: std::net::UdpSocket::bind("127.0.0.1:2000")?
        };

        while (true) {
            let mut buffer : [u8; 1024]= [0; 1024];
            let (number_of_bytes, src_addr) = connection_manager.socket.recv_from(&mut buffer).expect("Didn't receive data");
            let filled_buf = &mut buffer[..number_of_bytes];
           
            let sender_port = src_addr.port();
            
            let opcode_bytes: [u8; 2] = [filled_buf[0], filled_buf[1]];

            let opcode = u16::from_be_bytes(opcode_bytes);
        
            let rrq_opcode = crate::tftp_protocol::tftp::OpCode::RRQ as u16; 
            let ack_opcode = crate::tftp_protocol::tftp::OpCode::ACK as u16; 

            if opcode == rrq_opcode {
                let filename_buffer: Vec<u8> = filled_buf[2..].to_vec();
                let filename = String::from_utf8(filename_buffer).expect(""); 
                let read_request = crate::tftp_protocol::tftp::ReadRequest {
                    filename: filename.clone(),
                    mode: String::from("octet")
                };
                connection_manager.connection_map.insert(sender_port, filename.clone());
                recv_rrq(read_request, &connection_manager)?;
            } else if opcode == ack_opcode {
                let block_number_bytes: [u8; 2] = [filled_buf[2], filled_buf[3]];
                let ack = crate::tftp_protocol::tftp::Acknowledge{
                    block_number: u16::from_be_bytes(block_number_bytes) 
                }; 
                recv_ack(ack, &connection_manager, sender_port)?;
            }
            else {
                println!("Received unrecognized operation - {}", opcode);
            }
        }
        Ok(())
    }
    
    fn send_data(data: crate::tftp_protocol::tftp::Data, connection_manager: &ConnectionManager) -> std::io::Result<()> {
        println!("Sending DATA");
        let mut message_buffer : Vec<u8> = Vec::new();
        let data_opcode = crate::tftp_protocol::tftp::OpCode::DATA as u16; 
        message_buffer.extend_from_slice(&data_opcode.to_be_bytes());
        message_buffer.extend_from_slice(&data.block_number.to_be_bytes());
        message_buffer.extend_from_slice(&data.data);
        connection_manager.socket.send_to(&message_buffer, "127.0.0.1:2001")?;
        Ok(()) 
    }

    fn recv_rrq(read_request: crate::tftp_protocol::tftp::ReadRequest, connection_manager: &ConnectionManager) -> std::io::Result<()> {
        println!("Receiving RRQ");
        let mut file = std::fs::File::open(read_request.filename)?;
        
        let mut read_buffer : [u8; crate::tftp_protocol::tftp::BLOCK_SIZE] = [0; crate::tftp_protocol::tftp::BLOCK_SIZE];

        let bytes_read = file.read_at(&mut read_buffer, 0)?;
        
        let mut filled_buf : Vec<u8> = Vec::new();
        filled_buf.extend_from_slice(&read_buffer[..bytes_read]);

        let data = crate::tftp_protocol::tftp::Data{
            block_number: 0,
            data: filled_buf 
        }; 

        send_data(data, &connection_manager)?;
        Ok(())
    }

    fn recv_ack(ack: crate::tftp_protocol::tftp::Acknowledge, connection_manager: &ConnectionManager, sender_port: u16) -> std::io::Result<()> {
        println!("Receiving ACK");
        let mut file_name : String = String::from("");

        match connection_manager.connection_map.get(&sender_port) {
            Some(filename) => file_name = filename.clone(), 
            None => file_name = String::from("")
        }

        if file_name.is_empty() {
            return Err(Error::new(ErrorKind::Other, "Pre-existing connection not found for ACK"));
        }

        let mut file = std::fs::File::open(file_name)?;
        let mut read_buffer : [u8; crate::tftp_protocol::tftp::BLOCK_SIZE]= [0; crate::tftp_protocol::tftp::BLOCK_SIZE];

        let offset : u64 = ack.block_number as u64 * crate::tftp_protocol::tftp::BLOCK_SIZE as u64; 
        let bytes_read = file.read_at(&mut read_buffer, offset)?;
        
        let mut filled_buf : Vec<u8> = Vec::new();
        filled_buf.extend_from_slice(&read_buffer[..bytes_read]);

        let data = crate::tftp_protocol::tftp::Data{
            block_number: 0,
            data: filled_buf 
        }; 

        send_data(data, &connection_manager)?;
        Ok(())
    }
}
