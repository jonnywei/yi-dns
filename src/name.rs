use crate::{NameCache, Result, byte_buf::DnsByteBuf};

pub const COMPRESSION_MASK: u8 = 0b1100_0000;
 const COMPRESSION_MASK_U16: u16 = 0b1100_0000_0000_0000;

#[derive(Debug,Clone)]
pub struct  NAME {
    pub  name  : String,
    pub(crate) label : Vec<String>,
    pub  packet_index: usize ,
}

impl NAME {

    pub fn from_bytes(bytes: &mut DnsByteBuf) -> Result<NAME> {

        let mut labels = Vec::new();
        let mut string = String::new();
        let  packet_index = bytes.get_cursor();
        loop {
            let length = bytes.get_u8()?;
            if length & COMPRESSION_MASK == COMPRESSION_MASK {
                let next = bytes.get_u8()?;
                let  index = (length <<2 >>6 ) as usize * 256 + next as usize;
                println!("{},{}",index,next);
                let mut inner_name:NAME = NAME::from_index_bytes(bytes, index)?;
                string.push_str(&inner_name.name);
                labels.append(&mut inner_name.label);
                break;
            }else {
                if length == 0 {
                    if string.len() > 0 {
                        string.pop(); //pop last .
                    }
                    break;
                }
                let label = String::from_utf8(bytes.get_bytes(length as usize)?.to_vec())?;
                string.push_str(&label);
                labels.push(label);
                string.push_str(".");
            }
          
        }
        Ok( NAME {
            label:labels,
            name: string,
            packet_index,
        })
    }

    pub  fn from_index_bytes(bytes: &mut DnsByteBuf, index: usize) -> Result<NAME> {
        let mut index:usize = index;
        let mut string = String::new();
        let mut labels = Vec::new();
        let  packet_index =index;
        loop {
            let length = bytes.get_index_u8(index)?;
            if length & COMPRESSION_MASK == COMPRESSION_MASK {
                let next = bytes.get_index_u8(index+1)?;
                index = (length <<2 >>6 ) as usize * 256 + next as usize;
                println!("{},{}",index,next);
                let mut inner_name:NAME = NAME::from_index_bytes(bytes, index)?;
                string.push_str(&inner_name.name);
                labels.append(&mut inner_name.label);
                break;
            }else {
                index = index + 1;
                if length == 0 {
                    if string.len() > 0 {
                        string.pop(); //pop last .
                    }
                    break;
                }
                let label = String::from_utf8(bytes.get_index_bytes(index,length as usize)?.to_vec())?;
                string.push_str(&label);
                labels.push(label);
                string.push_str(".");
                index =  index+ length as usize; 
            }
          
        }
        Ok( NAME {
            label: labels,
            name: string,
            packet_index,
        })
    }

      /// build new NAME
      pub fn new(name:String) ->NAME{
        NAME {
            packet_index:0 ,
            label: domain_to_label(&name),
            name,
            
        }
    }
    pub fn to_bytes(&self) ->Vec<u8> {
        let mut v = Vec::<u8>::new();
        for s in &self.label {
            v.push(s.len() as u8);
            v.extend(s.to_string().into_bytes());
        }
        v.push(0);
        v
    }
  

    pub fn to_compress_bytes(&self, ctx: & mut NameCache, current: usize) -> Vec<u8> {
        let result =  ctx.sub_domain(self);
        ctx.append(self.clone(),current);
        if result.is_none(){
            return self.to_bytes();
        }       
        let result =  result.unwrap();
        let label_count =self.label.len() - result.1;

        let mut v = Vec::<u8>::new();
        for u in 0..label_count {
            let s =& self.label[u];
            v.push(s.len() as u8);
            v.extend(s.to_string().into_bytes());
        }
        let pointer = COMPRESSION_MASK_U16 | result.0 as u16;
        v.extend(u16::to_be_bytes(pointer));
        v
    }
 
}

fn domain_to_label(domain:&str) -> Vec<String>{
    let mut v = Vec::<String>::new();
    let splits = domain.split(".");
    for s in splits {
        v.push(s.to_string());
    }
    v
}

// #[derive(Debug)]
// pub struct  QNAME {
//     pub  name :String ,
//     pub length:usize ,
// }
// impl QNAME {
//     pub fn to_bytes(&self) ->Vec<u8> {
//         let mut v = Vec::<u8>::new();
//         let mut splits = self.name.split(".");
//         for s in splits {
//             v.push(s.len() as u8);
//             v.extend(s.to_string().into_bytes());
//         }
//         v.push(0);
//         v
//     }
// }

// impl From<NAME> for QNAME {
//     fn from(n: NAME) -> Self {
//         QNAME { name:n.name, length:n.length }
//     }
// }


// pub trait FromStr {
    
//     fn from_str(string: String, length: usize) -> Result<Self> where Self: Sized;
// }

// pub fn from_bytes<R:FromStr>(bytes: &mut DnsByteBuf) -> Result<R> {
//    let mut index:usize = 0;
//    let mut total_length = 0;
//    let mut string = String::new();
//    loop {
//        let length = bytes.get_u8()?;
//        if length & COMPRESSION_MASK == COMPRESSION_MASK {
//            let next = bytes.get_u8()?;
//            index = (length <<2 >>6 ) as usize * 256 + next as usize;
//            println!("{},{}",index,next);
//            let inner_name:NAME = NAME::from_index_bytes(bytes, index)?;
//            string.push_str(&inner_name.name);
//            total_length = total_length + inner_name.length as usize;
//            break;
//        }else {
//            total_length = total_length +length as usize;
//            if length == 0 {
//                if string.len() >= 0 {
//                    string.pop(); //pop last .
//                }
//                break;
//            }
//            let label = String::from_utf8(bytes.get_bytes(length as usize)?.to_vec())?;
//            string.push_str(&label);
//            string.push_str(".");
//        }
     
//    }
//    Ok( FromStr::from_str(string, total_length)?)
// }


// impl FromStr for NAME {
//    fn from_str(string: String, total_length: usize) -> Result<Self> where Self: Sized {
//        Ok(NAME {
//            name: string,
//            length: total_length,
//        })
//    }
// }