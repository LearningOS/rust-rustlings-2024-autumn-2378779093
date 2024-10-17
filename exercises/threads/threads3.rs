// threads3.rs
//
// Execute `rustlings hint threads3` or use the `hint` watch subcommand for a
// hint.


use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

struct Queue {
    length: u32,
    first_half: Vec<u32>,
    second_half: Vec<u32>,
}

impl Queue {
    fn new() -> Self {
        Queue {
            length: 10,
            first_half: vec![1, 2, 3, 4, 5],
            second_half: vec![6, 7, 8, 9, 10],
        }
    }
}

fn send_tx(q: Arc<Queue>, tx: Arc<Mutex<mpsc::Sender<u32>>>) {
    let qc1 = Arc::clone(&q);
    let qc2 = Arc::clone(&q);
    let tx1 = Arc::clone(&tx);
    let tx2 = Arc::clone(&tx);

    // 发送第一个半部分
    let thread1 = thread::spawn(move || {
        let tx = tx1.lock().unwrap();  // 锁定Sender
        for &val in &qc1.first_half {
            println!("sending {:?}", val);
            if let Err(e) = tx.send(val) {
                eprintln!("Error sending value: {}", e);
            }
            thread::sleep(Duration::from_secs(1));
        }
    });

    // 发送第二个半部分
    let thread2 = thread::spawn(move || {
        let tx = tx2.lock().unwrap();  // 锁定Sender
        for &val in &qc2.second_half {
            println!("sending {:?}", val);
            if let Err(e) = tx.send(val) {
                eprintln!("Error sending value: {}", e);
            }
            thread::sleep(Duration::from_secs(1));
        }
    });

    // 等待两个线程完成
    thread1.join().unwrap();
    thread2.join().unwrap();
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let queue = Arc::new(Queue::new());
    let tx = Arc::new(Mutex::new(tx));  // 用Mutex保护Sender

    // 使用clone()来避免move
    send_tx(queue.clone(), tx);

    let mut total_received: u32 = 0;
    for received in rx {
        println!("Got: {}", received);
        total_received += 1;
    }

    println!("total numbers received: {}", total_received);
    assert_eq!(total_received, queue.length);
}

