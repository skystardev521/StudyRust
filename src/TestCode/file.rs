use std::error::Error;
use std::io::prelude::*;
use std::path::Path;

use std::fs;
use std::fs::{File, OpenOptions};
use std::io;

fn read_file() -> std::io::Result<String> {
    //::process::Run_Command();

    Ok(String::from("s: &String"))
    //String::from("result")
    //let file = Path::open("./lorem_ipsum.txt");
    //file.read_to_string()
}

fn write_file() {
    let path = Path::new("./lorem_ipsum.txt");

    let display = path.display();

    let mut file = match File::create(&path) {
        Ok(file) => file,
        Err(why) => panic!("couldn`t create:{}:{}", display, why.description()),
    };

    match file.write_all("LOREM_IPSUM".as_bytes()) {
        Ok(_) => println!("successfully wrote to {}", display),
        Err(why) => panic!("couldn`t write to {}: {}", display, why.description()),
    }
}

// `% cat path` 的简单实现
fn cat(path: &Path) -> io::Result<String> {
    let mut f = r#try!(File::open(path));
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// `% echo s > path` 的简单实现
fn echo(s: &str, path: &Path) -> io::Result<()> {
    let mut f = r#try!(File::create(path));

    f.write_all(s.as_bytes())
}

// `% touch path`（忽略已存在文件）的简单实现
fn touch(path: &Path) -> io::Result<()> {
    match OpenOptions::new().create(true).write(true).open(path) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

pub fn TestFile() {
    write_file();

    println!(
        "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx{}",
        read_file().unwrap()
    );

    println!("`mkdir a`");
    // 创建一个目录，返回 `io::Result<()>`
    match fs::create_dir("a") {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(_) => {}
    }

    println!("`echo hello > a/b.txt`");
    // 前面的匹配可以用 `unwrap_or_else` 方法简化
    echo("hello", &Path::new("a/b.txt")).unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });

    println!("`mkdir -p a/c/d`");
    // 递归创建一个目录，返回 `io::Result<()>`
    fs::create_dir_all("a/c/d").unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });

    println!("`touch a/c/e.txt`");
    touch(&Path::new("a/c/e.txt")).unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });

    println!("`ln -s ../b.txt a/c/b.txt`");
    // 创建一个符号链接，返回 `io::Resutl<()>`
    fs::soft_link("../b.txt", "a/c/b.txt").unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });

    println!("`cat a/c/b.txt`");
    match cat(&Path::new("a/c/b.txt")) {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(s) => println!("> {}", s),
    }

    println!("`ls a`");
    // 读取目录的内容，返回 `io::Result<Vec<Path>>`
    match fs::read_dir("a") {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(paths) => {
            for path in paths {
                println!("> {:?}", path.unwrap().path());
            }
        }
    }

    println!("`rm a/c/e.txt`");
    // 删除一个文件，返回 `io::Result<()>`
    fs::remove_file("a/c/e.txt").unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });

    println!("`rmdir a/c/d`");
    // 移除一个空目录，返回 `io::Result<()>`
    fs::remove_dir("a/c/d").unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });
}
