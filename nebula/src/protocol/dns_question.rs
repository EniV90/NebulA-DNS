use super::{byte_packet::BytePacketBuffer, query_type::QueryType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DnsQuestion {
  pub name: String,
  pub qtype: QueryType
}

impl DnsQuestion {
  pub fn new(name: String, qtype: QueryType) -> Self {
    Self {
      name,
      qtype

    }
  }

  pub fn read(&mut self, buffer: &mut BytePacketBuffer) -> Result<(), Box<dyn std::error::Error>> {
    buffer.read_qname(&mut self.name)?;
    self.qtype = QueryType::from_num(buffer.read_two_bytes()?);
    let _ = buffer.read_two_bytes()?;

    Ok(())
  }
}