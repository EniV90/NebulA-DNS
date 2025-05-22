use std::net::{IpAddr, UdpSocket};

use protocol::{byte_packet::BytePacketBuffer, dns_packet::DnsPacket, dns_question::DnsQuestion, query_type::QueryType};

use crate::protocol::response_packet::response_packet;


mod protocol;



fn look_up(qname: &str, qtype: QueryType, ) -> Result<DnsPacket, Box<dyn std::error::Error>> {
  // let qname = "google.com";
  // let qtype = QueryType::A;

  let server = ("8.8.8.8", 53);

  let socket = UdpSocket::bind(("0.0.0.0", 43210))?;

  let mut packet = DnsPacket::new();

  packet.header.id = 6666;
  packet.header.questions = 1;
  packet.header.recursion_desired = true;

  packet.questions.push(DnsQuestion::new(qname.to_string(), qtype));

  let mut req_buffer = BytePacketBuffer::new();
  let _ = packet.write(&mut req_buffer);
  socket.send_to(&req_buffer.buf[0..req_buffer.pos], server)?;

  let mut res_buffer = BytePacketBuffer::new();
  socket.recv_from(&mut res_buffer.buf)?;

  DnsPacket::from_buffer(&mut res_buffer)

}

fn handle_query(socket: &UdpSocket) -> Result<(), Box<dyn std::error::Error>> {
  let mut req_buffer =BytePacketBuffer::new();

  let(_, src) = socket.recv_from(&mut req_buffer.buf)?;

  let mut request = DnsPacket::from_buffer(&mut req_buffer)?;

  let mut packet =DnsPacket::new();
  packet.header.id = request.header.id;
  packet.header.recursion_desired = true;
  packet.header.recursion_desired = true;
  packet.header.response = true;


  if let Some(question) = request.questions.pop() {
    println!("Received query {:?}", question);

    if let Ok(result) = look_up(&question.name, question.qtype) {
    packet.questions.push(question);
    packet.header.rescode = result.header.rescode;
    }
  }


    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
 response_packet()?;

 Ok(())
}
