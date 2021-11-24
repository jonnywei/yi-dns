use yi_dns::Message;
use yi_dns::Raw;
use yi_dns::ResouceRecord;
use yi_dns::RDATA;
use yi_dns::Soa;
fn main() {
    let message =Message::new("abc.com",true);

    println!("{:#?}",message);

    let rr = ResouceRecord::new_rtype("abc".to_string(), 22,yi_dns::rtype::A,RDATA::A(222));
    println!("{:#?}",rr);


    let rr = ResouceRecord::new_rtype("abc".to_string(), 22,
    yi_dns::rtype::SOA,RDATA::SOA(Soa::new("abc".to_string())));


    
    let rr = ResouceRecord::new("abc".to_string(), 22,
    RDATA::RAW(Raw::new(12,[1,1].to_vec())));

}