use std::sync::mpsc;
use std::thread;
pub fn runThread() {
    let (tx, rx) = mpsc::channel();
    for i in 0..10 {
        let tx = tx.clone();
        thread::spawn(move || {
            let answer = i * i;
            tx.send(answer).unwrap();
        });
    }
    for _ in 0..10 {
        println!("recv:{}", rx.recv().unwrap());
    }

    //testThreadPanic();
}

fn testThreadPanic() {
    let handle = thread::spawn(move || {
        //panic!("oops!");
    });
    let result = handle.join();
    assert!(result.is_err() == false);
}
