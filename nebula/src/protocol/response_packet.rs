use std::{error::Error, fs::File, io::Read, net::UdpSocket};

use crate::protocol::{dns_question::DnsQuestion, query_type::QueryType};

use super::{byte_packet::BytePacketBuffer, dns_packet::DnsPacket};



pub fn response_packet() -> Result<(), Box<dyn Error>> {
  let qname = "yahoo.com";
  let qtype = QueryType::MX;

  let server = ("8.8.8.8", 53);

  let socket = UdpSocket::bind(("0.0.0.0", 43210))?;

  let mut packet = DnsPacket::new();

  packet.header.id = 6666;
  packet.header.questions = 1;
  packet.header.recursion_desired = true;

  packet.questions.push(DnsQuestion::new(qname.to_string(), qtype));

  let mut req_buffer = BytePacketBuffer::new();
  packet.write(&mut req_buffer);
  socket.send_to(&req_buffer.buf[0..req_buffer.pos], server)?;

  let mut res_buffer = BytePacketBuffer::new();
  socket.recv_from(&mut res_buffer.buf)?;

  
  // let mut f = File::open("response_packet.txt")?;
  // let mut buffer = BytePacketBuffer::new();
  // let _ = f.read(&mut buffer.buf)?;

  let packet = DnsPacket::from_buffer(&mut res_buffer)?;
  println!("{:#?}", packet.header);

  for q in packet.questions {
    println!("{:#?}", q);
  }

  for rec in packet.answers {
    println!("{:#?}", rec);
  }

  for rec in packet.authorities {
    println!("{:#?}", rec);
  }

  for rec in packet.resources {
    println!("{:#?}", rec)
  }
  Ok(())
}