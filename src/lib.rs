mod byte_buf;
mod name;
mod resource_record;
mod message;
mod max_sub_domain;

use std::{array::TryFromSliceError, string::FromUtf8Error};

use thiserror::Error;

pub use name::*;
pub use message::*;
pub use resource_record::*;

pub type Result<T> = std::result::Result<T, YiDnsError>;

#[derive(Debug,Error)]
pub enum YiDnsError{

    #[error("dns packet error")]
    PacketFormatError ( #[from] TryFromSliceError),
    
    #[error("dns packet error")]
    PacketQNameError ( #[from] FromUtf8Error),
   
    #[error("dns resouce type {0} error")]
    PacketRRTypeError(u16),

    #[error("dns resouce class {0} error")]
    PacketRRClassError(u16),

    #[error("unknown data error")]
    Unknown,
    
}
#[derive(Debug)]
pub struct  NameCache<'a> {
    names : Vec<&'a NAME>,
    pub  packet_index: usize ,
}

mod tests {

    // use crate::message::*;
    use super::*;
    // use super::yi_dns::*;
    #[test]
    fn test_message(){
        let message = Message::new("baidu.com",true);
        println!("{:#?}", message);
    }



    #[test]
    fn test_decode_query(){
        let bytes = [ 0xff ,0xff, 01, 00 ,00 ,01, 00, 00, 00, 00, 00, 00, 03, 0x70 ,0x61, 0x6e,
            05, 0x62, 0x61, 0x69, 0x64, 0x75, 03, 0x63, 0x6f ,0x6d, 00, 00, 01, 00, 01];
        let message =  Message::from_bytes(bytes.to_vec()).unwrap();
        println!("{:#?}", message);
    }





    #[test]
    fn test_decode_additional(){
        let input: &[u8] = &[
            83, 202, // ID
            1, 32, // Flags
            0, 1, // qdcount
            0, 0, // ancount
            0, 0, // nscount
            0, 1, // arcount
            // Q Section
            3, 119, 119, 119, // len: 3 - www
            6, 103, 111, 111, 103, 108, 101, // len: 6 - google
            3, 99, 111, 109, // len: 3 - com
            0,   // name terminator
            0, 1, // qtype
            0, 1, // qclass
            // AR Section
            0, // no name
            0, 1, // type
           0,1, // class
            0, 0, 0, 1, // ttl
            0, 12, // rdlength
            0, 10, 0, 8, 107, 120, 163, 147, 238, 31, 231, 235, // rdata
        ];
        let message =  Message::from_bytes(input.to_vec()).unwrap();
        println!("{:#?}", message);
    }


    #[test]
    fn test_decode_answer_question(){
        let input: &[u8] =  &[
            0xdb, 0x42, 0x81, 0x80, 0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x03, 0x77,
            0x77, 0x77, 0x0c, 0x6e, 0x6f, 0x72, 0x74, 0x68, 0x65, 0x61, 0x73, 0x74, 0x65, 0x72,
            0x6e, 0x03, 0x65, 0x64, 0x75, 0x00, 0x00, 0x01, 0x00, 0x01, 0xc0, 0x0c, 0x00, 0x01,
            0x00, 0x01, 0x00, 0x00, 0x02, 0x58, 0x00, 0x04, 0x9b, 0x21, 0x11, 0x44,
        ];
        let message =  Message::from_bytes(input.to_vec()).unwrap();
        println!("{:#?}", message);
    }



    #[test]
    fn test_decode_multi_answer(){
        let input: &[u8] = &[
            55, 93, 129, 128, 0, 1, 0, 4, 0, 0, 0, 0, 3, 119, 119, 119, 9, 109, 105, 99, 114, 111,
            115, 111, 102, 116, 3, 99, 111, 109, 0, 0, 1, 0, 1, 3, 119, 119, 119, 9, 109, 105, 99,
            114, 111, 115, 111, 102, 116, 3, 99, 111, 109, 0, 0, 5, 0, 1, 0, 0, 11, 196, 0, 35, 3,
            119, 119, 119, 9, 109, 105, 99, 114, 111, 115, 111, 102, 116, 7, 99, 111, 109, 45, 99,
            45, 51, 7, 101, 100, 103, 101, 107, 101, 121, 3, 110, 101, 116, 0, 3, 119, 119, 119, 9,
            109, 105, 99, 114, 111, 115, 111, 102, 116, 7, 99, 111, 109, 45, 99, 45, 51, 7, 101,
            100, 103, 101, 107, 101, 121, 3, 110, 101, 116, 0, 0, 5, 0, 1, 0, 0, 63, 25, 0, 58, 3,
            119, 119, 119, 9, 109, 105, 99, 114, 111, 115, 111, 102, 116, 7, 99, 111, 109, 45, 99,
            45, 51, 7, 101, 100, 103, 101, 107, 101, 121, 3, 110, 101, 116, 11, 103, 108, 111, 98,
            97, 108, 114, 101, 100, 105, 114, 6, 97, 107, 97, 100, 110, 115, 3, 110, 101, 116, 0,
            3, 119, 119, 119, 9, 109, 105, 99, 114, 111, 115, 111, 102, 116, 7, 99, 111, 109, 45,
            99, 45, 51, 7, 101, 100, 103, 101, 107, 101, 121, 3, 110, 101, 116, 11, 103, 108, 111,
            98, 97, 108, 114, 101, 100, 105, 114, 6, 97, 107, 97, 100, 110, 115, 3, 110, 101, 116,
            0, 0, 5, 0, 1, 0, 0, 3, 90, 0, 28, 6, 101, 49, 51, 54, 55, 56, 4, 100, 115, 99, 98, 10,
            97, 107, 97, 109, 97, 105, 101, 100, 103, 101, 3, 110, 101, 116, 0, 6, 101, 49, 51, 54,
            55, 56, 4, 100, 115, 99, 98, 10, 97, 107, 97, 109, 97, 105, 101, 100, 103, 101, 3, 110,
            101, 116, 0, 0, 1, 0, 1, 0, 0, 0, 16, 0, 4, 23, 40, 73, 65,
        ];

        let message =  Message::from_bytes(input.to_vec()).unwrap();
        println!("{:#?}", message);
       
    }


    #[test]
    fn query_google()->Result<()>{
       
        let input = b"\x00\x03\x01\x00\x00\x01\x00\x00\x00\x00\x00\x00\x06\x67\x6f\x6f\x67\x6c\x65\x03\x63\x6f\x6d\x00\x00\x01\x00\x01";
        let message =  Message::from_bytes(input.to_vec())?;
        println!("{:#?}", message);
        
        Ok(())
    }


    #[test]
    fn reply_google_com()->Result<()>{
       
        let bytes = b"\x00\x03\x81\x80\x00\x01\x00\x0b\x00\x00\x00\x00\x06\x67\x6f\x6f\x67\x6c\x65\x03\x63\x6f\x6d\x00\
        \x00\x01\x00\x01\xc0\x0c\x00\x01\x00\x01\x00\x00\x00\x04\x00\x04\x4a\x7d\xec\x23\xc0\x0c\x00\x01\x00\x01\x00\x00\x00\x04\
        \x00\x04\x4a\x7d\xec\x25\xc0\x0c\x00\x01\x00\x01\x00\x00\x00\x04\x00\x04\x4a\x7d\xec\x27\xc0\x0c\x00\x01\x00\x01\x00\x00\
        \x00\x04\x00\x04\x4a\x7d\xec\x20\xc0\x0c\x00\x01\x00\x01\x00\x00\x00\x04\x00\x04\x4a\x7d\xec\x28\xc0\x0c\x00\x01\x00\x01\
        \x00\x00\x00\x04\x00\x04\x4a\x7d\xec\x21\xc0\x0c\x00\x01\x00\x01\x00\x00\x00\x04\x00\x04\x4a\x7d\xec\x29\xc0\x0c\x00\x01\
        \x00\x01\x00\x00\x00\x04\x00\x04\x4a\x7d\xec\x22\xc0\x0c\x00\x01\x00\x01\x00\x00\x00\x04\x00\x04\x4a\x7d\xec\x24\xc0\x0c\
        \x00\x01\x00\x01\x00\x00\x00\x04\x00\x04\x4a\x7d\xec\x2e\xc0\x0c\x00\x01\x00\x01\x00\x00\x00\x04\x00\x04\x4a\x7d\xec\x26";
        let message =  Message::from_bytes(bytes.to_vec())?;
        println!("{:#?}", message);
        Ok(())
    }


    #[test]
    fn reply_pan_com()->Result<()>{
       
        let bytes = b"\xff\xff\x81\x80\x00\x01\x00\x02\x00\x00\x00\x00\x03\x70\x61\x6e\
        \x05\x62\x61\x69\x64\x75\x03\x63\x6f\x6d\x00\x00\x01\x00\x01\xc0\
        \x0c\x00\x05\x00\x01\x00\x00\x03\xbb\x00\x11\x05\x79\x69\x79\x75\
        \x6e\x01\x6e\x06\x73\x68\x69\x66\x65\x6e\xc0\x16\xc0\x2b\x00\x01\
        \x00\x01\x00\x00\x00\x43\x00\x04\xdc\xb5\x6f\x5b";
        let message =  Message::from_bytes(bytes.to_vec())?;
        println!("{:#?}", message);
        Ok(())
    }



    #[test]
    fn query_qq_com()->Result<()>{
       
        let bytes = b"\x48\x6e\x01\x00\x00\x01\x00\x00\x00\x00\x00\x00\x07\x6d\x6f\x7a\
        \x69\x6c\x6c\x61\x0e\x63\x6c\x6f\x75\x64\x66\x6c\x61\x72\x65\x2d\
        \x64\x6e\x73\x03\x63\x6f\x6d\x00\x00\x1c\x00\x01"
        ;
        let message =  Message::from_bytes(bytes.to_vec())?;
        println!("{:#?}", message);
        Ok(())
    }



    #[test]
    fn response_aaaa()->Result<()>{
       
        let bytes = b"\x48\x6e\x81\x80\x00\x01\x00\x02\x00\x00\x00\x00\x07\x6d\x6f\x7a\
        \x69\x6c\x6c\x61\x0e\x63\x6c\x6f\x75\x64\x66\x6c\x61\x72\x65\x2d\
        \x64\x6e\x73\x03\x63\x6f\x6d\x00\x00\x1c\x00\x01\xc0\x0c\x00\x1c\
        \x00\x01\x00\x00\x00\x75\x00\x10\x26\x06\x47\x00\x00\x00\x00\x00\
        \x00\x00\x00\x00\x68\x10\xf8\xf9\xc0\x0c\x00\x1c\x00\x01\x00\x00\
        \x00\x75\x00\x10\x26\x06\x47\x00\x00\x00\x00\x00\x00\x00\x00\x00\
        \x68\x10\xf9\xf9"
        ;
        let message =  Message::from_bytes(bytes.to_vec())?;
        println!("{:#?}", message);
        Ok(())
    }


    #[test]
    fn test_decode_to_query(){
        let bytes = [ 0xff ,0xff, 01, 00 ,00 ,01, 00, 00, 00, 00, 00, 00, 03, 0x70 ,0x61, 0x6e,
            05, 0x62, 0x61, 0x69, 0x64, 0x75, 03, 0x63, 0x6f ,0x6d, 00, 00, 01, 00, 01];
        let message =  Message::from_bytes(bytes.to_vec()).unwrap();
        println!("{:#?}", message);
        let v = message.to_bytes();
        println!("{:#?}", v);
    }


    #[test]
    fn test_mdns_query(){
        let bytes = b"\x00\x00\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x0f\x32\x30\x32\
        \x31\x30\x39\x31\x36\x2d\x31\x35\x35\x35\x32\x39\x05\x6c\x6f\x63\
        \x61\x6c\x00\x00\x01\x00\x01"
        ;
        let message =  Message::from_bytes(bytes.to_vec()).unwrap();
        println!("{:#?}", message);
        let v = message.to_bytes().unwrap();
        println!("{:#?}", v);
    }


    #[test]
    fn test_mdns_muti_query(){
        let bytes = b"\x00\x00\x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x08\x5f\x68\x6f\
        \x6d\x65\x6b\x69\x74\x04\x5f\x74\x63\x70\x05\x6c\x6f\x63\x61\x6c\
        \x00\x00\x0c\x00\x01\x08\x39\x33\x63\x63\x30\x38\x31\x62\x04\x5f\
        \x73\x75\x62\x0e\x5f\x61\x70\x70\x6c\x65\x2d\x6d\x6f\x62\x64\x65\
        \x76\x32\xc0\x15\x00\x0c\x00\x01"
        ;
        let message =  Message::from_bytes(bytes.to_vec()).unwrap();
        println!("{:#?}", message);
        let v = message.to_bytes().unwrap();
        println!("{:#?}", v.len() == bytes.len());
    }

    #[test]
    fn test_mdns_response(){
        let bytes = b"\x00\x00\x84\x00\x00\x00\x00\x06\x00\x00\x00\x02\x0c\x63\x36\x64\
        \x61\x38\x66\x63\x64\x61\x30\x34\x64\x08\x5f\x61\x69\x72\x64\x72\
        \x6f\x70\x04\x5f\x74\x63\x70\x05\x6c\x6f\x63\x61\x6c\x00\x00\x21\
        \x80\x01\x00\x00\x00\x78\x00\x2d\x00\x00\x00\x00\x22\x42\x24\x61\
        \x30\x63\x39\x39\x32\x38\x62\x2d\x35\x66\x61\x30\x2d\x34\x62\x62\
        \x36\x2d\x38\x35\x36\x63\x2d\x30\x61\x62\x61\x62\x37\x62\x66\x66\
        \x65\x61\x32\xc0\x27\xc0\x3e\x00\x1c\x80\x01\x00\x00\x00\x78\x00\
        \x10\xfe\x80\x00\x00\x00\x00\x00\x00\x10\x42\x1b\xf0\x20\x42\x52\
        \xb3\xc0\x3e\x00\x01\x80\x01\x00\x00\x00\x78\x00\x04\x0a\x14\x0b\
        \x06\xc0\x0c\x00\x10\x80\x01\x00\x00\x11\x94\x00\x0b\x0a\x66\x6c\
        \x61\x67\x73\x3d\x31\x30\x31\x39\x09\x5f\x73\x65\x72\x76\x69\x63\
        \x65\x73\x07\x5f\x64\x6e\x73\x2d\x73\x64\x04\x5f\x75\x64\x70\xc0\
        \x27\x00\x0c\x00\x01\x00\x00\x11\x94\x00\x02\xc0\x19\xc0\x19\x00\
        \x0c\x00\x01\x00\x00\x11\x94\x00\x02\xc0\x0c\xc0\x0c\x00\x2f\x80\
        \x01\x00\x00\x00\x78\x00\x09\xc0\x0c\x00\x05\x00\x00\x80\x00\x40\
        \xc0\x3e\x00\x2f\x80\x01\x00\x00\x00\x78\x00\x08\xc0\x3e\x00\x04\
        \x40\x00\x00\x08"
        ;
        let message =  Message::from_bytes(bytes.to_vec()).unwrap();
        println!("{:#?}", message);
        let v = message.to_bytes().unwrap();
        println!("{:#?}", v);
        let message =  Message::from_bytes(v).unwrap();
        println!("{:#?}", message);
    }


    #[test]
    fn test_mdns_chinese_response(){
        let bytes = b"\x00\x00\x84\x00\x00\x00\x00\x02\x00\x00\x00\x06\x17\xe9\xa9\xac\
        \xe5\xbb\xb6\xe9\xbe\x99\xe7\x9a\x84\x4d\x61\x63\x42\x6f\x6f\x6b\
        \x20\x50\x72\x6f\x0c\x5f\x64\x65\x76\x69\x63\x65\x2d\x69\x6e\x66\
        \x6f\x04\x5f\x74\x63\x70\x05\x6c\x6f\x63\x61\x6c\x00\x00\x10\x00\
        \x01\x00\x00\x11\x94\x00\x20\x14\x6d\x6f\x64\x65\x6c\x3d\x4d\x61\
        \x63\x42\x6f\x6f\x6b\x50\x72\x6f\x31\x32\x2c\x31\x0a\x6f\x73\x78\
        \x76\x65\x72\x73\x3d\x31\x37\x0f\x5f\x63\x6f\x6d\x70\x61\x6e\x69\
        \x6f\x6e\x2d\x6c\x69\x6e\x6b\xc0\x31\x00\x0c\x00\x01\x00\x00\x11\
        \x94\x00\x1a\x17\xe9\xa9\xac\xe5\xbb\xb6\xe9\xbe\x99\xe7\x9a\x84\
        \x4d\x61\x63\x42\x6f\x6f\x6b\x20\x50\x72\x6f\xc0\x67\xc0\x83\x00\
        \x10\x80\x01\x00\x00\x11\x94\x00\x22\x16\x72\x70\x42\x41\x3d\x43\
        \x37\x3a\x42\x36\x3a\x35\x30\x3a\x32\x42\x3a\x46\x39\x3a\x30\x38\
        \x0a\x72\x70\x56\x72\x3d\x31\x35\x32\x2e\x31\xc0\x83\x00\x21\x80\
        \x01\x00\x00\x00\x78\x00\x1f\x00\x00\x00\x00\xc1\x15\x16\x6d\x61\
        \x79\x61\x6e\x6c\x6f\x6e\x67\x64\x65\x4d\x61\x63\x42\x6f\x6f\x6b\
        \x2d\x50\x72\x6f\xc0\x36\xc0\xdd\x00\x1c\x80\x01\x00\x00\x00\x78\
        \x00\x10\xfe\x80\x00\x00\x00\x00\x00\x00\x1c\xe7\x78\x4c\x82\x3d\
        \x4e\x92\xc0\xdd\x00\x01\x80\x01\x00\x00\x00\x78\x00\x04\x0a\x14\
        \x0b\xfc\xc0\x83\x00\x2f\x80\x01\x00\x00\x11\x94\x00\x09\xc0\x83\
        \x00\x05\x00\x00\x80\x00\x40\xc0\xdd\x00\x2f\x80\x01\x00\x00\x00\
        \x78\x00\x08\xc0\xdd\x00\x04\x40\x00\x00\x08"
        ;
        let message =  Message::from_bytes(bytes.to_vec()).unwrap();
        println!("{:#?}", message);
        let v = message.to_bytes().unwrap();
        println!("{:#?}", v);
    }

}