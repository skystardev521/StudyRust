
mod bytes;

fn main() {

    let mut vbytes = vec![0u8; 42];

    bytes::write_bytes(&mut vbytes, &mut 0, "hello ccc".as_bytes());

    println!("str:{:?}", String::from_utf8_lossy(bytes::read_bytes(&mut vbytes, &mut 0, 8)));

    let mut pos = 0;
    let mut vec = vec![0u8; 42];
    println!("Vec Len:{}", vec.len());

    bytes::write_u8(&mut vec, &mut pos, 9);
    bytes::write_i8(&mut vec, &mut pos, -9);
    bytes::write_u16(&mut vec, &mut pos, 999);

    bytes::write_i16(&mut vec, &mut pos, -999);
    bytes::write_u32(&mut vec, &mut pos, 99999);
    bytes::write_i32(&mut vec, &mut pos, -99999);
    bytes::write_u64(&mut vec, &mut pos, 9999999999);
    bytes::write_i64(&mut vec, &mut pos, -9999999999);
    bytes::write_f32(&mut vec, &mut pos, 99999.99);
    bytes::write_f64(&mut vec, &mut pos, -99999.99);

    println!("pos:{}", pos);

    let mut pos = 0;
    println!("read_u8:{}", bytes::read_u8(&vec, &mut pos));
    println!("read_i8:{}", bytes::read_i8(&vec, &mut pos));
    println!("read_u16:{}", bytes::read_u16(&vec, &mut pos));
    println!("read_i16:{}", bytes::read_i16(&vec, &mut pos));
    println!("read_u32:{}", bytes::read_u32(&vec, &mut pos));
    println!("read_i32:{}", bytes::read_i32(&vec, &mut pos));
    println!("read_u64:{}", bytes::read_u64(&vec, &mut pos));
    println!("read_i64:{}", bytes::read_i64(&vec, &mut pos));
    println!("read_f32:{}", bytes::read_f32(&vec, &mut pos));
    println!("read_f64:{}", bytes::read_f64(&vec, &mut pos));
    println!("pos:{}", pos);

    let f32_bytes = 99.9f32.to_le_bytes();
    println!("leu8_to_f32:{}", bytes::read_f32(&f32_bytes, &mut 0));

    let f64_bytes = 99.9f64.to_le_bytes();
    println!("leu8_to_f64:{}", bytes::read_f64(&f64_bytes, &mut 0));

    let be = 1u16.to_be_bytes();
    let le = 1u16.to_le_bytes();
    println!(
        "be:{} le:{}",
        bytes::read_u16(&be, &mut 0).to_le(),
        bytes::read_u16(&le, &mut 0)
    );

    let fle = 1f32.to_le_bytes();
    let fbe = 0.000000000000000000000000000000000000000046006f32.to_be_bytes();
    println!(
        "be:{} le:{}",
        bytes::read_f32(&fbe, &mut 0),
        bytes::read_f32(&fle, &mut 0)
    );
}

/*
fn _test() {
    let filename = "file.txt";
    println!("Ext:{}", &filename[4 + 1..]);

    let ts1 = String::from("s1 ");
    let ts2 = String::from(" s2");

    println!("----------->s1 + s2 ={}{}", &ts1, &ts2);
    let s3 = ts1 + &ts2;
    println!("----------->s1 + s2 ={}", &s3);

    std::thread::sleep(std::time::Duration::from_secs(1));

    let str1 = String::from("str");
    let fun = |str: &String| -> String {
        println!("xxxxxx:{}", str);
        String::from(str)
    };

    let str2 = fun(&str1);

    println!("xxxxxx:{}", str1);

    let x = Some("foo");
    assert_eq!(x.map_or(42, |v| v.len()), 3);

    let x: Option<&str> = None;
    assert_eq!(x.map_or(42, |v| v.len()), 42);

    let _nmap = (1..20).map(|x| x + 1);

    let tuple: (u32, String) = (5, String::from("str"));
    let (ref x1, ref s2) = tuple;

    println!("x1:{} s2:{} tuple:{:?}", x1, s2, tuple);

    let mut map = std::collections::HashMap::new();

    map.insert("k", "xxx");
    map.insert("k1", "xxxxxx");
    map.insert("k2", "xxxxxxxxx");
    map.insert("k3", "xxxxxxxxxxx");

    for (k, v) in &map {
        println!("k:{} v:{}", k, v);
    }

    if map.contains_key("k") {
        println!("k--v:{}", map.get("k").unwrap());
    }

    TestCode::traits::run();

    let args: Vec<String> = std::env::args().collect();

    println!("args len:{}", args.len());

    if args.len() > 2 {
        let query = &args[1];
        let filename = &args[2];
        println!("Searching for {}", query);
        println!("In file {}", filename);
    }

    return ();
}
*/
