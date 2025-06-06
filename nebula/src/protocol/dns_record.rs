use std::net::{Ipv4Addr, Ipv6Addr};

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
  }, //1
  NS {
    domain: String,
    host: String,
    ttl: u32
  },
  CNAME {
    domain: String,
    host: String,
    ttl: u32
  },
  MX {
    domain: String,
    priority: u16,
    host: String,
    ttl: u32
  },
  AAAA {
    domain: String,
    addr: Ipv6Addr,
    ttl: u32,
  }

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

      QueryType::MX => {
        let priority  = buffer.read_two_bytes()?;
        let mut mx = String::new();
        buffer.read_qname(&mut mx)?;
        
        Ok(DnsRecord::MX { domain, priority, host: mx, ttl })
      }

      QueryType::CNAME => {
        let mut cname = String::new();
        buffer.read_qname(&mut cname)?;

        Ok(DnsRecord::CNAME { domain, host: cname, ttl })
      }

      QueryType::NS => {
        let mut ns = String::new();
        buffer.read_qname(&mut ns)?;

        Ok(DnsRecord::NS { domain, host: ns, ttl })
      }

      QueryType::AAAA => {
        let raw_addr1 = buffer.read_four_bytes()?;
        let raw_addr2 = buffer.read_four_bytes()?;
        let raw_addr3 = buffer.read_four_bytes()?;
        let raw_addr4 = buffer.read_four_bytes()?;
        let addr = Ipv6Addr::new(
          ((raw_addr1 >> 16) & 0xFFFF) as  u16, 
          ((raw_addr1 >> 0) & 0xFFFF) as  u16, 
          ((raw_addr2 >> 16) & 0xFFFF) as  u16, 
          ((raw_addr2 >> 0) & 0xFFFF) as  u16, 
          ((raw_addr3 >> 16) & 0xFFFF) as  u16, 
          ((raw_addr3 >> 0) & 0xFFFF) as  u16, 
          ((raw_addr3 >> 16) & 0xFFFF) as  u16, 
          ((raw_addr3 >> 0) & 0xFFFF) as  u16, 
          );
          Ok(DnsRecord::AAAA { domain, addr, ttl })
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

      DnsRecord::MX { ref domain, priority, ref host, ttl } => {
        buffer.write_qname(domain)?;
        buffer.write_u16(QueryType::MX.to_num())?;
        buffer.write_u16(1)?;
        buffer.write_u32(ttl)?;

        let pos = buffer.pos;
        buffer.write(0)?;

        buffer.write_u16(priority)?;
        buffer.write_qname(host)?;

        let size = buffer.pos - (pos + 2);
        buffer.set_u16(pos, size as u16)?;

      }

      DnsRecord::CNAME { ref domain, ref host, ttl } => {
        buffer.write_qname(domain)?;
        buffer.write_u16(QueryType::CNAME.to_num())?;
        buffer.write_u16(1)?;
        buffer.write_u32(ttl)?;

        let pos = buffer.pos;
        buffer.write_u16(0)?;

        buffer.write_qname(host)?;

        let size = buffer.pos - (pos + 2);
        buffer.set_u16(pos, size as u16)?;
      }

      DnsRecord::NS { ref domain, ref host, ttl } => {
        buffer.write_qname(domain)?;
        buffer.write_u16(QueryType::NS.to_num())?;
        buffer.write_u16(1)?;
        buffer.write_u32(ttl)?;

        let pos = buffer.pos;
        buffer.write_u16(0)?;

        buffer.write_qname(host)?;

        let size = buffer.pos - (pos + 2);
        buffer.set_u16(pos, size as u16)?;
      }

      DnsRecord::AAAA { ref domain, ref addr, ttl } => {
        buffer.write_qname(domain)?;
        buffer.write_u16(QueryType::AAAA.to_num())?;
        buffer.write_u16(1)?;
        buffer.write_u32(ttl)?;
        buffer.write_u16(16)?;

        for octet in &addr.segments() {
          buffer.write_u16(*octet)?;
        }
      }
      DnsRecord::UNKNOWN { .. } => {
        println!("Skipping record: {:?}", self)
      }
    }
    Ok(buffer.pos - start_pos)
  }
}