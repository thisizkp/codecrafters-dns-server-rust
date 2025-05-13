#[allow(unused_imports)]
use std::net::UdpSocket;

struct DNSHeader {
    pub packet_identifier: u16,

    pub query_response_indicator: bool,
    pub operation_code: u8,
    pub authoritative_answer: bool,
    pub truncated_message: bool,
    pub recursion_desired: bool,
    pub recursion_available: bool,
    pub reserved: u8,
    pub response_code: u8,

    pub question_count: u16,
    pub answer_count: u16,
    pub authority_count: u16,
    pub additional_count: u16,
}

impl DNSHeader {
    pub fn new() -> Self {
        Self {
            packet_identifier: 1234,

            query_response_indicator: true,
            operation_code: 0,
            authoritative_answer: false,
            truncated_message: false,
            recursion_desired: false,
            recursion_available: false,
            reserved: 0,
            response_code: 0,

            question_count: 0,
            answer_count: 0,
            authority_count: 0,
            additional_count: 0,
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(12);

        bytes.extend_from_slice(&self.packet_identifier.to_be_bytes());

        let mut flags1: u8 = 0;
        if self.query_response_indicator {
            flags1 |= 0b10000000;
        }
        flags1 |= (self.operation_code & 0b00001111) << 3;
        if self.authoritative_answer {
            flags1 |= 0b00000100;
        }
        if self.truncated_message {
            flags1 |= 0b00000010;
        }
        if self.recursion_desired {
            flags1 |= 0b00000001;
        }

        let mut flags2: u8 = 0;
        if self.recursion_available {
            flags2 |= 0b10000000;
        }
        flags2 |= (self.reserved & 0b00000111) << 4;
        flags2 |= self.response_code & 0b00001111;

        bytes.push(flags1);
        bytes.push(flags2);

        bytes.extend_from_slice(&self.question_count.to_be_bytes());
        bytes.extend_from_slice(&self.answer_count.to_be_bytes());
        bytes.extend_from_slice(&self.authority_count.to_be_bytes());
        bytes.extend_from_slice(&self.additional_count.to_be_bytes());

        bytes
    }
}

fn main() {
    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let mut buf = [0; 512];

    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                println!("Received {} bytes from {}", size, source);
                let dns_header = DNSHeader::new();
                let response = dns_header.to_bytes();
                udp_socket
                    .send_to(&response, source)
                    .expect("Failed to send response");
            }
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
                break;
            }
        }
    }
}
