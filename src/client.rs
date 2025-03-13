
pub mod client {
    use rand::Rng;
    use std::io::{Write};
   
    pub fn validate_input(args:Vec<String>) -> Option<String>{
        if args.len() == 3 {
            return Some(args[2].clone()); 
        }
        return None;
    }

    pub fn client_main(file_name: String) -> std::io::Result<()> {
        let mut rnd_generator = rand::rng(); 
        let client_addr = std::net::SocketAddr::from(([127, 0, 0, 1], rnd_generator.random_range(2001..2500)));
        let socket = std::net::UdpSocket::bind(client_addr)?;

        println!("Bound socket to address: {}", client_addr);

        let read_request = crate::tftp_protocol::tftp::ReadRequest {
            filename: file_name.clone(),
            mode: String::from("octet")
        };

        send_rrq(read_request, &socket)?;
        println!("Requested file {}", file_name);

        let mut process_messages = true;
        while process_messages {
            match recv_data(file_name.clone(), &socket) {
                Ok(result) => process_messages = result,
                Err(e) => eprintln!("Error: {}", e)
            }
        }

        println!("Finished receiving!");
        Ok(())
    }
    
    fn send_rrq(read_request: crate::tftp_protocol::tftp::ReadRequest, socket: &std::net::UdpSocket) -> std::io::Result<()> {
        println!("Sending RRQ");
        let mut message_buffer : Vec<u8> = Vec::new();
        let rrq_opcode = crate::tftp_protocol::tftp::OpCode::RRQ as u16; 
        message_buffer.extend_from_slice(&rrq_opcode.to_be_bytes());
        message_buffer.extend_from_slice(read_request.filename.as_bytes());
        socket.send_to(&message_buffer, "127.0.0.1:2000")?;
        Ok(())
    }

    fn send_ack(ack: crate::tftp_protocol::tftp::Acknowledge, socket: &std::net::UdpSocket) -> std::io::Result<()> {
        println!("Sending ACK");
        let mut message_buffer : Vec<u8> = Vec::new();
        let ack_opcode = crate::tftp_protocol::tftp::OpCode::ACK as u16; 
        message_buffer.extend_from_slice(&ack_opcode.to_be_bytes());
        message_buffer.extend_from_slice(&ack.block_number.to_be_bytes());
        socket.send_to(&message_buffer, "127.0.0.1:2000")?;
        Ok(())
    }

    fn recv_data(filename: String, socket: &std::net::UdpSocket) -> std::io::Result<bool> {
        println!("Receiving DATA");
        let mut buffer : [u8; 1024]= [0; 1024];
        let (number_of_bytes, _src_addr) = socket.recv_from(&mut buffer).expect("Didn't receive data");
        let filled_buf = &mut buffer[..number_of_bytes];
                
        let block_number_bytes: [u8; 2] = [filled_buf[2], filled_buf[3]];
        let ack = crate::tftp_protocol::tftp::Acknowledge{
            block_number: u16::from_be_bytes(block_number_bytes) 
        }; 

        let data_buffer: Vec<u8> = filled_buf[4..].to_vec();
        
        let mut file = std::fs::OpenOptions::new()
                        .create(true)
                        .append(true)
                        .open(String::from("client-copy.txt"))?;
        
        file.write_all(&data_buffer)?;
        send_ack(ack, &socket)?;
        
        if number_of_bytes < crate::tftp_protocol::tftp::BLOCK_SIZE + 4 {
            return Ok(false);
        }
        Ok(true)
    }

}
