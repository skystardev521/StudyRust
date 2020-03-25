use std::process::Command;

pub fn Run_Command() {
    let args: Vec<String> = std::env::args().collect();
    println!("I got {:?} arguments: {:?}.", args.len() - 1, &args[1..]);
    println!("cur dir: {:?}", std::env::current_dir().unwrap());
    println!("cur exe: {:?}", std::env::current_exe().unwrap());
    for _v in std::env::vars() {
        //println!("{:?}", v);
    }

    let output = Command::new("rustc")
        .arg("--help")
        .output()
        .unwrap_or_else(|e| panic!("failed to execute process:{}", e));

    if output.status.success() {
        //let s = String::from_utf8_lossy(&output.stdout);
        //println!("rustc succeeded and stdout was:\n{}", s);

        println!("rustc succeeded and stdout was:\n");
    } else {
        let s = String::from_utf8_lossy(&output.stderr);

        println!("rustc failed and stderr was:\n{}", s);
    }
}
