
use crate::NAME;

#[derive(Debug)]
pub struct  NameCache {
    names : Vec<NAME>,
    pub  packet_index: Vec<usize> ,
}

impl NameCache {

   pub(crate) fn new_default () -> Self {
        NameCache {
             names :Vec::new(), 
             packet_index: Vec::new(), 
        }
    }

    pub(crate) fn append(&mut self, name : NAME, index : usize){
        self.names.push(name);
        self.packet_index.push(index);

    }
    pub(crate)  fn sub_domain(&self, new_domain: &NAME) -> Option<(usize,usize)> {
        let  result =self.sub_domain_origin(new_domain);
        if result.0 == -1 { return None};
        let index = result.0 as usize;
        let  base  = &self.names[index];
        let mut index = self.packet_index[index];

        let label_count =base.label.len() - result.2;

        for u in 0..label_count {
            index += 1 + base.label[u].len();
        }
        return Some((index, result.2));
    }

    pub(crate)  fn sub_domain_origin(&self, new_domain: &NAME) ->(isize,usize,usize) {
        let mut result = (-1isize,0,0);
        let mut index : isize = 0;
        for domain in &self.names {
            let (count,label_count) = NameCache::max_sub_domain_vec(&domain.label, &new_domain.label);
            if count > result.1 {
                result = (index, count, label_count);
            }
            index += 1;
        }
        return result;
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


}

#[cfg(test)]
mod tests {

    use super::NameCache;
    use crate::NAME;
    #[test]
    fn test_subdomain_label(){
        let mut name_cache = NameCache::new_default();
        let b = String::from("3333.abc.test.com");
        let a  =NAME::new( String::from("bc.test.com"));
        let c  = NAME::new(String::from("change.mail.sohu.net") );
        let d  = NAME::new(String::from("aaahu.net") );
        let e  = NAME::new(String::from("mail.aaahu.net"));

        name_cache.append(NAME::new(b), 100);
        let result = name_cache.sub_domain(&a);
        println!("{:#?}",result);

        name_cache.append(a, 200);
        let result = name_cache.sub_domain(&c);
        println!("{:#?}",result);

        name_cache.append(c, 300);
        let result = name_cache.sub_domain(&d);
        println!("{:#?}",result);

        name_cache.append(d, 300);
        let result = name_cache.sub_domain(&e);
        println!("{:#?}",result);
    }

}