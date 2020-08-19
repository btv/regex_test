use regex::Regex;
use std::{fs::File,io::prelude::*};

fn create_regex() -> Regex {
    let mut regex2 = String::new();
    let mut file = File::open("traffic.in").unwrap();
    file.read_to_string(&mut regex2).unwrap();
    let re2 = Regex::new(&regex2).expect("could not encode regex!");
    dbg!("{:?}", &re2);
    re2
}

fn main() {
    let re1 = create_regex();
    let cookie = String::from("hs=1090488148; svSession=7f5774a77907540af3ac18850d8857ee00b3a2a7f8651155b10b649da43f3e99b63a34a629f6574d945d1dba1a4c0da31e60994d53964e647acf431e4f798bcd848063a691d03ee36db2be2e4e3d388709b9451e1f45ad79d0ff062fdec20fe7; XSRF-TOKEN=1597434272|rwf40R9-irIq; TS01e85bed=01f0e931317a2530a7d775faf0a58c68d6f60a550a352a5ee449edb3d1c0929f0eec5e1ed28d8c77d19423a929d6f69c1ac1586520; TS015649a3=01f0e93131382668cacf8a70cde79be0d713686eb006fcfd4797bae0f1ca34b6e4ab6d322bdb14dfad16ef64db3ec611c8f82fdcf2; bSession=21348a83-1bf7-4e2b-aed9-bbea0398490e|1; ssr-caching=\"cache#desc=hit#varnish=hit#dc#desc=42\"");

    match re1.captures(&cookie) {
        Some(cap) => {
            dbg!("svSession: {:?}, re1: {:?}", &cap.get(1).map_or("", |m| m.as_str()), &re1);
        },
        _ => {
            dbg!("cookie: {:?}, re: {:?}", &cookie, &re1);
        }
    };
}
