


#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ResultCode {
  NOERROR = 0,
  FORMERR = 1,
  SERVFAIL = 2,
  NXDOMIAN = 3,
  NOTIMP = 4,
  REFUSED = 5,
}

impl ResultCode {
    pub fn from_num(num: u8) -> ResultCode {
      match num {
          1 => ResultCode::FORMERR,
          2 => ResultCode::SERVFAIL,
          3 => ResultCode::NXDOMIAN,
          4 => ResultCode::NOTIMP,
          5 => ResultCode::REFUSED,
          0 => ResultCode::NOERROR,
          _ => ResultCode::NOERROR,
      }
    }
}