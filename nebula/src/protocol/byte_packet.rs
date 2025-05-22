use std::usize;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;




pub struct BytePacketBuffer {
  pub buf: [u8; 512],
  pub pos: usize,
}



impl BytePacketBuffer {
  
  // This give a fresh buffer for holding the packet contents and field for tracking where things are
  pub fn new () -> Self {
    Self { buf: [0; 512], pos: 0, }
  } 

    // The current position with the buffer
    pub fn current_positon_in_buffer(&self) -> usize {
      self.pos

    }

    // Moving the buffer a step forward with a specific number of steps
    pub fn move_buffer(&mut self, moves:usize) -> Result<()> {
      self.pos += moves;

      Ok(())
    }

    // Change the buffer position
    pub fn change_buffer_position(&mut self, pos: usize) -> Result<()> {
      self.pos = pos;

      Ok(())
    }

    // Read a single byte and move the position a single step forward
    pub fn read_single_byte(&mut self) -> Result<u8> {
      
      if self.pos >= 512 {
        return Err("End of Buffer".into())
      }

      let res = self.buf[self.pos];
      self.pos += 1;

      Ok(res)
  }

  // Get a single byte, without changing position
  pub fn get_single_byte(&mut self, pos: usize) -> Result<u8> {

    if pos >= 512 {
      return Err("End of Buffer".into())
    }
    Ok(self.buf[pos])
  }


  // Get range of byte
  pub fn get_range(&mut self, start: usize, length: usize) -> Result<&[u8]> {

    if start + length >= 512 {
      return Err("End of Buffer".into());
    }
    Ok(&self.buf[start..start + length as usize])
  }

  // Read two byte and moves two steps forward
  pub fn read_two_bytes(&mut self) -> Result<u16> {
    let res = ((self.read_single_byte()? as u16) << 8) | (self.read_single_byte()? as u16);

    Ok(res)
  }

  // Read four bytes and moves four steps forward
  pub fn read_four_bytes(&mut self) -> Result<u32> {
      let res = u32::from(self.read_single_byte()?) << 24
        | u32::from(self.read_single_byte()?) << 16
        | u32::from(self.read_single_byte()?) << 8
        | u32::from(self.read_single_byte()?) << 8
        | u32::from(self.read_single_byte()?);

      Ok(res)
  }
  

  pub fn read_qname(&mut self, outstr: &mut String) -> Result<()> {
    let mut pos = self.pos;


    let mut jumped = false;
    let max_jumps = 5;
    let mut jumps = 0;


    let mut delim = "";
    loop {

      if jumps > max_jumps {
        return Err(format!("Limit of jumps has been exceeded {}", max_jumps).into())
      }

      let len = self.get_single_byte(pos)?;
      if (len & 0xC0) == 0xC0 {
        if !jumped  {
          self.change_buffer_position(pos + 2)?
        }

        let b2 = self.get_single_byte(pos + 1)? as u16;
        let offset = (((len as u16) ^ 0xC0) << 8) | b2;
        pos = offset as usize;

        //indicate jumped was performed
        jumped = true;
        jumps += 1;
    
        continue;
      }
      
      else {
        pos += 1;

        if len == 0 {
          break;
        }

        outstr.push_str(delim);

        let str_buffer = self.get_range(pos, len as usize)?;
        outstr.push_str(&String::from_utf8_lossy(str_buffer).to_lowercase());

        delim = ".";

        pos += len as usize;
      }
    }

    if !jumped {
      self.change_buffer_position(pos)?;
    }
      Ok(())
  }

  // Write a single byte to the buffer and advance the position
  pub fn write(&mut self, val: u8) -> Result<()> {
    if self.pos >= 512 {
      return Err("End of buffer".into());
    }

    self.buf[self.pos] = val;
    self.pos += 1;
    Ok(())
  }
  
  // write a u8 to the buffer
  pub fn write_u8(&mut self, val: u8) -> Result<()> {
    self.write(val)?;

    Ok(())
  }

  pub fn write_u16(&mut self, val: u16) -> Result<()> {
    self.write(((val >> 8) & 0xFF) as u8)?;
    self.write((val & 0xff) as u8)?;

    Ok(())
  }

  pub fn write_u32(&mut self, val: u32) -> Result<()> {
    self.write(((val >> 24) & 0xFF) as u8 )?;
    self.write(((val >> 16) & 0xFF) as u8)?;
    self.write(((val >> 8) & 0xFF) as u8)?;
    self.write((val & 0xFF) as u8)?;

    Ok(())
  }

  pub fn write_qname(&mut self, qname: &str) -> Result<()> {
    for part in qname.split(".") {
      if part.len() > 63 {
        return Err("Label to long".into());
      }
      self.write(part.len() as u8)?;
      for c in part.chars() {
        self.write(c as u8)?;
      }
    }
    self.write(0)?;
    Ok(())
  }

  pub fn set(&mut self, pos: usize, val: u8) -> Result<()>{
    self.buf[pos] = val;

    Ok(())
  }

  pub fn set_u16(&mut self, pos: usize, val: u16) -> Result<()> {
    self.set(pos, (val >> 8) as u8)?;
    self.set(pos + 1, (val & 0xFF) as u8)?;

    Ok(())

  }

}
