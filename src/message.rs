use crate::Result;

use crate::byte_buf::DnsByteBuf;
use crate::name::{NAME, QNAME};
use crate::resource_record::*;


#[derive(Debug)]
pub struct Message {
    pub  header : Header,
    pub  question: Vec<Question>,
    pub  answer:  Vec<ResouceRecord>,
    pub  authority: Vec<ResouceRecord>,
    pub  additional: Vec<ResouceRecord>,
}


impl Message {

    pub fn new(domain: &str, query: bool) -> Message {
        let qname = QNAME{
            name: domain.to_owned(),
            length:0usize,
        };
        let question = Question {
            qname, 
            qtype : QTYPE::A,
            qclass :  QCLASS::IN,

        };

        let header = Header::new(query, OPCODE::QUERY);

        Message {
            header,
            question:Vec::new(), 
            answer: Vec::new(),
            authority: Vec::new(),
            additional: Vec::new(),
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let v = Vec::new();
        let mut byte_buf = DnsByteBuf::new(v,0);
        self.header.to_bytes(&mut byte_buf);
        for i in 0..self.question.len() {
            self.question[i].to_bytes(&mut byte_buf);
        }
        for an in 0..self.answer.len() {
            self.answer[an].to_bytes(&mut byte_buf);
        }
        for an in 0..self.authority.len() {
            self.authority[an].to_bytes(&mut byte_buf);
        }
        for an in 0..self.additional.len() {
            self.additional[an].to_bytes(&mut byte_buf);
        }

        let vec = byte_buf.get_vec();
        vec
    }

    pub fn from_bytes(bytes: Vec<u8>) -> Result<Self> {
        // let header:Vec<u8> = bytes[0..12].try_into()?;
        let mut byte_buf = DnsByteBuf::new(bytes,0);
        let header = Header::from_bytes(&mut byte_buf)?;

        let mut question = Vec::<Question>::new();
        let mut i = 0;
        while i <  header.question_count {
          i = i+1; 
          let q = Question::from_bytes( &mut byte_buf)?;
          question.push(q);
        }
        

        let mut answer = Vec::<ResouceRecord>::new();
        let mut i = 0;
        while i < header.answer_count {
            i = i+1; 
            let rr = ResouceRecord::from_bytes(&mut byte_buf)?;
            answer.push(rr);
           
        }

        let mut authority = Vec::<ResouceRecord>::new();
        let mut i = 0;
        while i < header.nameserver_count {
            i = i+1; 
            let rr = ResouceRecord::from_bytes(&mut byte_buf)?;
            authority.push(rr);
           
        }

        let mut additional = Vec::<ResouceRecord>::new();
        let mut i = 0;
        while i < header.additional_count {
            i = i+1; 
            let rr = ResouceRecord::from_bytes(&mut byte_buf)?;
            additional.push(rr);
           
        }

        Ok( Message{
            header,
            question,
            answer,
            authority,
            additional,
        })
    }
}
#[derive(Debug)]
pub struct Header {
    pub id : u16,
    flag:  HeaderFlag,
    pub question_count: u16,
    pub answer_count: u16,
    pub nameserver_count: u16,
    pub additional_count: u16,

}

impl Header{
    pub fn new(query:bool, op_code: OPCODE) ->Self {
        Header {
            id:0u16,
            flag: HeaderFlag::new(query, op_code),
            question_count: 0u16,
            answer_count: 0u16,
            nameserver_count: 0u16,
            additional_count: 0u16,
        }
    }

    pub fn to_bytes(&self, byte_buf: &mut DnsByteBuf) -> Result<usize> {
        byte_buf.put_u16(self.id);
        byte_buf.put_u16(self.flag.to_u16());
        byte_buf.put_u16(self.question_count);
        byte_buf.put_u16(self.answer_count);
        byte_buf.put_u16(self.nameserver_count);
        byte_buf.put_u16(self.additional_count);
        Ok(12)
    }

    pub fn from_bytes(bytes: &mut DnsByteBuf) -> Result<Self> {
        let id =  bytes.get_u16()?;
        let flag =  HeaderFlag::from_bytes(bytes.get_bytes(2)?)?;
        let question_count =   bytes.get_u16()?;
        let answer_count = bytes.get_u16()?;
        let nameserver_count =  bytes.get_u16()?;
        let additional_count = bytes.get_u16()?;
        Ok( Header {
            id,
            flag,
            question_count,
            answer_count,
            nameserver_count,
            additional_count,
        })
    }
}


#[derive(Debug,Clone, Copy)]
pub enum OPCODE {
    QUERY  = 0, // the standard QUERY
    IQUERY  = 1, // an inverse query (IQUERY)
    STATUS  = 2, //   a server status request (STATUS)
    Unknown = 15,//
}

impl OPCODE{
    pub fn from_u8(num: u8) -> Result<Self> {
        let qtype:OPCODE  = unsafe {::std::mem::transmute(num as u8)};
        Ok(qtype)
    }
    pub fn to_u8(&self) -> u8 {
      let x =   *self;
      x as u8
    }
}

#[derive(Debug,Clone, Copy)]
pub enum RCODE {
    Success = 0, //= 0, // the standard QUERY
    FormatError  = 1, //Format error
    ServerFailure  = 2, //  
    NameError =3,//
    NotImplemented =4,//
    Refused = 5, // The name server refuses to    perform the specified operation for  policy reasons.
    Unknown = 15,//
}

impl RCODE{
    pub fn from_u8(num: u8) -> Result<Self> {
        let qtype:RCODE  = unsafe {::std::mem::transmute(num)};
        Ok(qtype)
    }
    pub fn to_u8(&self) -> u8 {
      let x =   *self;
      x as u8
    }
}

#[derive(Debug)]
struct HeaderFlag {
    qr : u8, // one bit query or response  query (0), or a response (1).
    opcode  : OPCODE, //A four bit field that specifies kind of query in this     message
    aa:  u8, 
    tc:  u8,       
    rd:  u8,               
    ra:  u8, 
    z:   u8,
    rcode:  RCODE,                    
}


impl HeaderFlag{
    pub fn new(query:bool, op_code: OPCODE, ) ->Self {
        HeaderFlag {
            qr:query as u8,
            opcode  :op_code,
            aa: 0u8,
            tc:  0u8,       
            rd:  0u8,               
            ra:  0u8, 
            z:0u8,
            rcode:  RCODE::Success,    
        }
    }


    fn get_bit( flag: &mut u8, bits: u8) -> u8 {
        let  mask =  2u8.pow(bits.into()) - 1;
        let rd = *flag & mask;
        *flag = *flag >> bits;
        return rd
    }
    /// 
    /// 
    ///    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    ///    |QR|   Opcode  |AA|TC|RD|RA|   Z    |   RCODE   |
    ///    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    /// 
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let mut flag = u8::from_be_bytes(bytes[0..1].try_into()?);
        let rd = HeaderFlag::get_bit(&mut flag, 1);
        let tc = HeaderFlag::get_bit(&mut flag, 1);
        let aa = HeaderFlag::get_bit(&mut flag, 1);
        let op_code = HeaderFlag::get_bit(&mut flag, 4);
        let qr =  HeaderFlag::get_bit(&mut flag, 1);
        let mut flag = u8::from_be_bytes(bytes[1..2].try_into()?);
        let r_code = HeaderFlag::get_bit(&mut flag, 4);
        let z = HeaderFlag::get_bit(&mut flag, 3);
        let ra = HeaderFlag::get_bit(&mut flag, 1);

        Ok( HeaderFlag {
            qr:qr,
            opcode: OPCODE::from_u8(op_code)?,
            aa,
            tc,       
            rd,               
            ra, 
            z,
            rcode: RCODE::from_u8(r_code)?,  
        })
    }

    fn put_bit( flag: &mut u8, bits: u8, val: u8){
        *flag = *flag << bits;
        let  mask =  2u8.pow(bits.into()) - 1;
        let rd = val & mask;
        *flag = *flag |rd;

    }

    pub fn to_u16(&self) -> u16 {
        let mut flag = 0;
        HeaderFlag::put_bit(&mut flag, 1,self.ra);
        HeaderFlag::put_bit(&mut flag, 3,0);
        HeaderFlag::put_bit(&mut flag, 4,self.rcode.to_u8());


        let mut flag_hight = 0;
        HeaderFlag::put_bit(&mut flag_hight, 1,self.qr);
        HeaderFlag::put_bit(&mut flag_hight, 4,self.opcode.to_u8());
        HeaderFlag::put_bit(&mut flag_hight, 1,self.aa);
        HeaderFlag::put_bit(&mut flag_hight, 1,self.tc);
        HeaderFlag::put_bit(&mut flag_hight, 1,self.rd);

        let mut flag_hight = flag_hight as u16 ;
        flag_hight <<8 + flag
      }
}

#[derive(Debug,Copy,Clone)]
pub enum QTYPE {
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

    AXFR     =       252, //A request for a transfer of an entire zone

    MAILB     =      253 , //A request for mailbox-related records (MB, MG or MR)

    MAILA    =       254 , //A request for mail agent RRs (Obsolete - see MX)
    ANY  = 255,

}
impl QTYPE{
    pub fn from_u16(num: u16) -> Result<Self> {
        let qtype:QTYPE  = unsafe {::std::mem::transmute(num as u8)};
        Ok(qtype)
    }
    pub fn to_u16(&self) -> u16 {
      let x =   *self;
      x as u8 as u16
    }
}
#[derive(Debug,Clone, Copy)]
pub enum QCLASS {
    IN  = 1, // the internet

    CS = 2, // the internet
    
    CH = 3, // the CHAOS class

    HS = 4, //  Hesiod

    ANY = 255, // any class
}

// impl EnumU16 for QCLASS {}

impl QCLASS {
    pub fn from_u16(num: u16) -> Result<Self> {
        let qtype:QCLASS  = unsafe {::std::mem::transmute(num as u8)};
        Ok(qtype)
    }
    pub fn to_u16(&self) -> u16 {
      let x =   *self;
      x as u8 as u16
    }
}

#[derive(Debug)]
pub struct Question {
    qname: QNAME,
    qtype :QTYPE,
    qclass: QCLASS,
}

impl Question {

    pub fn from_bytes(bytes: &mut DnsByteBuf) -> Result<Self> {
        let qname =QNAME::from( NAME::from_bytes(bytes)?);
        let qtype = bytes.get_u16()?;
        let qclass =  bytes.get_u16()?;

        Ok( Question {
            qname, 
            qtype:  QTYPE::from_u16(qtype)?,
            qclass:  QCLASS::from_u16(qclass)?,
        })
    }

    pub fn to_bytes(&self, byte_buf: &mut DnsByteBuf) {
        byte_buf.put_vec( self.qname.to_bytes());
        // byte_buf.put_u16(self.qname.);
        byte_buf.put_u16(self.qtype.to_u16());
        byte_buf.put_u16(self.qclass.to_u16());
      
    }
}

