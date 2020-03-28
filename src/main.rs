mod bytes;//::{Bytes};

fn main() {
    let mut vbytes = vec![0u8; 42];

    let mut _bytes = bytes::Bytes::new(vbytes.as_mut_slice());

    _bytes.write_bytes("hello ccc".as_bytes());

    _bytes.set_pos(0);
    println!("str:{}", String::from_utf8_lossy(_bytes.read_bytes(8)));

    let mut vec = vec![0u8; 42];
    println!("Vec Len:{}", vec.len());
    let mut _bytes = bytes::Bytes::new(vec.as_mut_slice());
    println!("bytes size:{}", _bytes.get_size());
    _bytes.write_u8(9);
    _bytes.write_i8(-9);
    _bytes.write_u16(999);

    _bytes.write_i16(-999);
    _bytes.write_u32(99999);
    _bytes.write_i32(-99999);
    _bytes.write_u64(9999999999);
    _bytes.write_i64(-9999999999);
    _bytes.write_f32(99999.99);
    _bytes.write_f64(-99999.99);

    println!("pos:{}", _bytes.get_pos());

    _bytes.set_pos(0);
    println!("read_u8:{}", _bytes.read_u8());
    println!("read_i8:{}", _bytes.read_i8());
    println!("read_u16:{}", _bytes.read_u16());
    println!("read_i16:{}", _bytes.read_i16());
    println!("read_u32:{}", _bytes.read_u32());
    println!("read_i32:{}", _bytes.read_i32());
    println!("read_u64:{}", _bytes.read_u64());
    println!("read_i64:{}", _bytes.read_i64());
    println!("read_f32:{}", _bytes.read_f32());
    println!("read_f64:{}", _bytes.read_f64());

    println!("pos:{}", _bytes.get_pos());

    let mut f32_bytes = 99.9f32.to_le_bytes();
    let mut _bytes = bytes::Bytes::new(&mut f32_bytes);
    println!("leu8_to_f32:{}", _bytes.read_f32());

    let mut f64_bytes = 99.9f64.to_le_bytes();
    let mut _bytes = bytes::Bytes::new(&mut f64_bytes);
    println!("leu8_to_f64:{}", _bytes.read_f64());

    let mut be_bytes = 1u16.to_be_bytes();
    let mut le_bytes = 1u16.to_le_bytes();
    let mut bytes_be = bytes::Bytes::new(&mut be_bytes);
    let mut bytes_le = bytes::Bytes::new(&mut le_bytes);
    println!(
        "be:{} le:{}",
        bytes_be.read_u16().to_le(),
        bytes_le.read_u16()
    );

    let mut fle_bytes = 1f32.to_le_bytes();
    let mut fbe_bytes = 0.000000000000000000000000000000000000000046006f32.to_be_bytes();

    let mut bytes_fle = bytes::Bytes::new(&mut fle_bytes);
    let mut bytes_fbe = bytes::Bytes::new(&mut fbe_bytes);
    println!("be:{} le:{}", bytes_fle.read_f32(), bytes_fbe.read_f32());
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
