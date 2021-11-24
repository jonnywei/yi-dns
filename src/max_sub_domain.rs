mod tests {

    #[test]
    fn test_subdomain(){
        let b = String::from("3333.abc.test.com");
        let a  = String::from("bc.test.com");
        let c  = String::from("change.mail.sohu.net");
        let d  = String::from(".aaahu.net");
        let e  = String::from("mail.aaahu.net");
        let (last_count,label_count) =  test_sub(&a, &b);
        println!("last_count: {} label_count: {}",last_count,label_count);
        let (last_count,label_count) =  test_sub(&c, &d);
        println!("last_count: {} label_count: {}",last_count,label_count);

        let (last_count,label_count) =  test_sub(&d, &e);
        println!("last_count: {} label_count: {}",last_count,label_count);

        // for la in a.chars().rev() {
            
        //     println!("{}",la);
        // }
    }



    
    #[test]
    fn test_subdomain_vec(){
        let b = String::from("3333.abc.test.com");
        let a  = String::from("bc.test.com");
        let c  = String::from("change.mail.sohu.net");
        let d  = String::from(".aaahu.net");
        let e  = String::from("mail.aaahu.net");
        let (last_count,label_count) =  test_sub(&a, &b);
        
        let mut vec = Vec::new();
        vec.push(&a);
        vec.push(&b);
        vec.push(&c);
        vec.push(&d);
        let result = test_sub_vec(vec, &e);
        println!("{:#?}",result);
    }



    fn test_sub_vec(domain_vec: Vec<&String>, new_domain: &String) ->(isize,usize,usize) {
        let mut result = (-1isize,0,0);
        let mut index : isize = 0;
        for domain in domain_vec {
            let (count,label_count) = test_sub(domain,new_domain);
            if count > result.1{
                result = (index, count, label_count);
            }
            index += 1;
        }
        return result;
    }


    fn test_sub(a:&String, b:&String)->(usize,usize) {
        let mut ita = a.chars().rev();
        let mut itb = b.chars().rev();
        let mut count = 0;
        let mut last_count = 0;
        let mut label_count = 0;
        loop {
            match (ita.next(), itb.next()) {
                (Some(ref a), Some(ref b)) if a == b => {
                    count += 1;
                    if *a == '.' {
                        label_count += 1;
                        last_count = count; 
                    }
                },
                _ => {
                    count = last_count;
                    break;
                },
            }
            // let ca = ita.next();
            // if ita.next() == itb.next() {
            //     count +=1;
            // }
            
        }
        (last_count,label_count)
    }


    fn max_sub_domain_vec(base:&Vec<String>, domain :&Vec<String>)->(usize,usize) {

        let mut ita =base.iter().rev();
        let mut itb = domain.iter().rev();
        let mut count = 0;
        let mut label_count = 0;
        loop {
            match (ita.next(), itb.next()) {
                (Some(ref a), Some(ref b)) if a.eq(b)  => {
                    count += b.len();
                    label_count += 1;
                },
                _ => {
                    break;
                },
            }
        }
        (count,label_count)
    }

    fn domain_to_label(domain:&str) -> Vec<String>{
        let mut v = Vec::<String>::new();
        let splits = domain.split(".");
        for s in splits {
            v.push(s.to_string());
        }
        v
    }


    #[test]
    fn test_subdomain_label(){
        let b = domain_to_label(&String::from("3333.abc.test.com"));
        let a  =domain_to_label( &String::from("bc.test.com"));
        let c  = domain_to_label(&String::from("change.mail.sohu.net"));
        let d  = domain_to_label(&String::from("aaahu.net"));
        let e  = domain_to_label(&String::from("mail.aaahu.net"));
        let (last_count,label_count) =  max_sub_domain_vec(&a, &b);
        println!("last_count: {} label_count: {}",last_count,label_count);
        let (last_count,label_count) =  max_sub_domain_vec(&c, &d);
        println!("last_count: {} label_count: {}",last_count,label_count);
        let (last_count,label_count) =  max_sub_domain_vec(&d, &e);
        println!("last_count: {} label_count: {}",last_count,label_count);

    }


    

}