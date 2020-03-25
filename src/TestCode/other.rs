pub mod test_mod {
    pub fn print_mod_name() {
        println!("mod_name:test_mod");
    }
}

struct Example {
    a: String,
}

impl Drop for Example {
    fn drop(&mut self) {
        println!("drop data:{}", self.a);
    }
}

pub fn test_drop() {
    let _a1 = Example {
        a: String::from("hello"),
    };
    println!("run test_drop end");
}
