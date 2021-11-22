use crate::{NAME, Result, YiDnsError, byte_buf::DnsByteBuf};

#[derive(Debug, Clone, Copy)]
pub enum TYPE {
    A = 1,

    NS = 2,

    MD = 3,
   
    MF = 4,    // a mail forwarder (Obsolete - use MX)
    
    CNAME = 5, // the canonical name for an alias

    SOA = 6, // marks the start of a zone of authority

    MB = 7, //a mailbox domain name (EXPERIMENTAL)

    MG = 8, // a mail group member (EXPERIMENTAL)

    MR = 9, // a mail rename domain name (EXPERIMENTAL)

    NULL = 10, //a null RR (EXPERIMENTAL)

    WKS = 11, // a well known service description

    PTR = 12, // a domain name pointer

    HINFO = 13, //host information

    MINFO = 14, //mailbox or mail list information

    MX = 15, //mail exchange

    TXT = 16, //text strings

      /// Host address (IPv6) [rfc3596](https://tools.ietf.org/html/rfc3596)
    AAAA = 28,

    /// https://datatracker.ietf.org/doc/html/rfc2052
    ///         Service.Proto.Name TTL Class SRV Priority Weight Port Target
    SRV = 33,

    Unknown =255 ,
}
impl TYPE {
    pub fn from_u16(num: u16) -> Result<Self> {
        let qtype: TYPE = unsafe { ::std::mem::transmute(num as u8) };
        match qtype  {
            TYPE::Unknown  => return Err(YiDnsError::PacketRRTypeError(num)),
            _ =>{},
        }
        Ok(qtype)
    }
    pub fn to_u16(&self) -> u16 {
        let x = *self;
        x as u8 as u16
    }
}
const CACHE_FLUSH_MARK:u16 = 0b1000_0000_0000_0000;

#[derive(Debug, Clone, Copy)]
pub enum CLASS {
    IN = 1, // the internet

    CS = 2, // the internet

    CH = 3, // the CHAOS class

    HS = 4, //  Hesiod

    Unknown = 255,
}

impl CLASS {
    pub fn from_u16(num: u16) -> Result<Self> {
        let class: CLASS = unsafe { ::std::mem::transmute(num as u8) };
        match class  {
            CLASS::Unknown  => return Err(YiDnsError::PacketRRTypeError(num)),
            _ =>{},
        }
        Ok(class)
    }
    pub fn to_u16(&self) -> u16 {
        let x = *self;
        x as u8 as u16
    }
}

#[derive(Debug)]
pub struct ResouceRecord {
    name: NAME,
    rtype: TYPE,
    rclass: CLASS,
    ///
    /// mdns use 
    /// https://datatracker.ietf.org/doc/html/rfc6762#section-10.3
    /// 
    pub cache_flush:bool,
    pub ttl: u32,
    pub rd_length: u16,
    pub rdata: RDATA,
}

impl ResouceRecord {
    pub fn from_bytes(bytes: &mut DnsByteBuf) -> Result<Self> {
        // let header:Vec<u8> = bytes[0..12].try_into()?;
        let name = NAME::from_bytes(bytes)?;
        let rtype = bytes.get_u16()?;
        let rtype = TYPE::from_u16(rtype)?;
        let rclass = bytes.get_u16()?;
        let cache_flush = rclass & CACHE_FLUSH_MARK == CACHE_FLUSH_MARK;
        let rclass = CLASS::from_u16(rclass& !CACHE_FLUSH_MARK)?;
        let ttl = bytes.get_u32()?;
        let rd_length = bytes.get_u16()?;
        // let rd_data = bytes.get_bytes(rd_length as usize)?;
        Ok(ResouceRecord {
            name,
            rtype,
            rclass,
            cache_flush,
            ttl,
            rd_length,
            rdata: RDATA::from_bytes(rtype, rd_length,  bytes)?,
        })
    }

    pub fn to_bytes(&self, byte_buf: &mut DnsByteBuf) {
        byte_buf.put_vec( self.name.to_bytes());
        byte_buf.put_u16(self.rtype.to_u16());
       
        let mut rclass = self.rclass.to_u16();
        if self.cache_flush {
            rclass = CACHE_FLUSH_MARK | rclass;
        }
        byte_buf.put_u16(rclass);

        byte_buf.put_u32(self.ttl);
        byte_buf.put_u16(self.rd_length);
        self.rdata.to_bytes(byte_buf);
    }
}

#[derive(Debug)]
pub enum RDATA {
    CNAME(String),
    PTR(String),
    A(u32),
    AAAA(u128),
    RAW(Vec<u8>),
}

impl RDATA {
    pub fn from_bytes(rtype: TYPE, rd_length:u16,  bytes: &mut DnsByteBuf) -> Result<Self> {
        // let rd_data = bytes.get_bytes(rd_length as usize)?;
        match rtype {
            TYPE::A => {
                 return Ok(RDATA::A(bytes.get_u32()?))   
            },
            _ => {
                return Ok(RDATA::RAW( bytes.get_bytes(rd_length as usize)?.into()))   
            }
        }
    }

    pub fn to_bytes(&self, byte_buf: &mut DnsByteBuf) {
       
        match self {
            RDATA::A(x) => {
                byte_buf.put_u32(*x); 
            },
            RDATA::RAW(x) => {
                byte_buf.put_slice(x); 
            },
            _ => {
                unreachable!(); 
            }
        }
    }
}