struct Foo<'a> {
    x: &'a i32,
}

pub fn test_life_time() {
    let f: Foo;
    //{
    let n = 5; // variable that is invalid outside this block
    let y = &n;
    f = Foo { x: &y };
    //}; // n dropped here
    println!("{}", f.x);

    let v1 = "ab";
    let v2 = "abc";

    test_life_time1(&v1, &v2);
}

fn test_life_time1<'a, 'b: 'a>(v1: &'a str, v2: &'b str) -> &'a str {
    if v1.len() == v2.len() {
        v1
    } else {
        v2
    }
}

#[derive(Debug)]
struct Person {
    name: String,
    addr: String,
}

fn add_val<'a, 'b>(vals: &'a mut Vec<&'b str>, person: &'b Person) {
    vals.push(&person.name);
    vals.push(&person.addr);
}

fn print(vals: &[&str]) {
    let person = vals.get(0).unwrap();

    println!("name:{:?}", person)
}

pub fn test_lefe_time2() {
    let person = Person {
        name: String::from("name"),
        addr: String::from("addr"),
    };

    let mut vals: Vec<&str> = Vec::new();

    add_val(&mut vals, &person);

    print(&vals);

    println!("xxxxxxxx:{}", person.name)
}
