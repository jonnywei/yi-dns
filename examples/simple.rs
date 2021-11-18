use yi_dns::Message;

fn main() {
    let message =Message::new("abc.com",true);

    println!("{:#?}",message);
}