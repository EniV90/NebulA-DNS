use std::net::Ipv4Addr;

use super::{byte_packet::BytePacketBuffer, query_type::QueryType};

#[derive(Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord)]

pub enum DnsRecord {
  UNKNOWN {
    domain: String,
    qtype: u16,
    data_length: u16,
    ttl: u32
  }, //0
  A {
    domain: String,
    addr: Ipv4Addr,
    ttl: u32
  } //1

}

impl DnsRecord {
  pub fn read(buffer: &mut BytePacketBuffer) -> Result<DnsRecord, Box<dyn std::error::Error>> {
    let mut domain = String::new();
    buffer.read_qname(&mut domain)?;

    let qtype_num = buffer.read_two_bytes()?;
    let qtype = QueryType::from_num(qtype_num);
    let _ = buffer.read_two_bytes()?;
    let ttl = buffer.read_four_bytes()?;
    let data_length = buffer.read_two_bytes()?;

    match qtype {
      QueryType::A => {
            let raw_addr = buffer.read_four_bytes()?;
            let addrr = Ipv4Addr::new(
              ((raw_addr >> 24) & 0xFF) as u8,
              ((raw_addr >> 16) & 0xFF) as u8,
              ((raw_addr >> 8) & 0xFF) as u8,
              ((raw_addr ) & 0xFF) as u8,
            );
            Ok(DnsRecord::A { domain, addr: addrr, ttl })
          }
      QueryType::UNKNOWN(_) => {
        buffer.move_buffer(data_length as usize)?;

        Ok(DnsRecord::UNKNOWN { 
          domain, 
          qtype: qtype_num, 
          data_length, 
          ttl 
        })
      }
    }

  }

  pub fn write(&self, buffer: &mut BytePacketBuffer) -> Result<usize, Box<dyn std::error::Error>> {
    let start_pos = buffer.pos;

    match *self {
      DnsRecord::A { ref domain, ref addr, ttl } => {
        buffer.write_qname(domain)?;
        buffer.write_u16(QueryType::A.to_num())?;
        buffer.write_u16(1)?;
        buffer.write_u32(ttl)?;
        buffer.write_u16(4)?;

        let octets = addr.octets();
        buffer.write_u8(octets[0])?;
        buffer.write_u8(octets[1])?;
        buffer.write_u8(octets[2])?;
        buffer.write_u8(octets[3])?;
      }
      DnsRecord::UNKNOWN { .. } => {
        println!("Skipping record: {:?}", self)
      }
    }
    Ok(buffer.pos - start_pos)
  }
}