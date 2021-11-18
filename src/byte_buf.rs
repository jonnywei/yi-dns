
use crate::Result;
pub struct DnsByteBuf {
   bytes: Vec<u8>,
   index: usize,
}

impl DnsByteBuf {

    pub fn new(bytes:Vec<u8>, index: usize) -> DnsByteBuf{
        DnsByteBuf { bytes: bytes, index: index}
    }

    pub fn get_vec(self:Self) ->Vec<u8> {
        self.bytes
    }

    pub fn get_u8(&mut self) ->Result<u8> {
        let p = self.index;
        self.index =  self.index +1;
        let id =  u8::from_be_bytes(self.bytes[p.. self.index].try_into()?);
        Ok(id)
    }

    pub fn get_u16(&mut self) ->Result<u16> {
        let p = self.index;
        self.index =  self.index +2;
        let id =  u16::from_be_bytes(self.bytes[p.. self.index].try_into()?);
        Ok(id)
    }

    pub fn get_u32(&mut self) ->Result<u32> {
        let p = self.index;
        self.index =  self.index +4;
        let id =  u32::from_be_bytes(self.bytes[p.. self.index].try_into()?);
        Ok(id)
    }

    pub fn get_bytes(&mut self ,length: usize) -> Result<&[u8]> {
        let p = self.index;
        self.index =  self.index + length;
        let id = self.bytes[p.. self.index].try_into().unwrap();
        Ok(id)
    }


    pub fn get_index_u8(&mut self, p: usize) -> Result<u8> {
        let index =  p +1;
        let id =  u8::from_be_bytes(self.bytes[p..index].try_into()?);
        Ok(id)
    }

    
    pub fn get_index_bytes(&mut self , p: usize, length: usize) -> Result<&[u8]> {
        let index =p + length;
        let id = self.bytes[p.. index].try_into().unwrap();
        Ok(id)
    }


    

    pub fn put_u8(&mut self,data: u8) {
        self.index =  self.index +1;
        self.bytes.push(data);
    }


    

    pub fn put_u16(&mut self,data:u16){
        self.index =  self.index +2;
        let array = data.to_be_bytes();
        self.bytes.push(array[0]);
        self.bytes.push(array[1]);
    }

    pub fn put_u32(&mut self,data:u32){
        self.index =  self.index +4;
        let array = data.to_be_bytes();
        self.bytes.push(array[0]);
        self.bytes.push(array[1]);
        self.bytes.push(array[2]);
        self.bytes.push(array[3]);
    }


    pub fn put_slice(&mut self,data:&[u8]){
        self.index =  self.index +data.len();
        self.bytes.extend_from_slice(data);
    }

    pub fn put_vec(&mut self,data:Vec<u8>){
        self.index =  self.index +data.len();
        self.bytes.extend(data);
    }
}