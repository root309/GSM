use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // 共有メモリに格納するデータ
    let data = Arc::new(Mutex::new(0));

    // スレッドを作成して共有メモリを操作
    let mut handles = vec![];
    for _ in 0..4 {
        let data = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let mut data = data.lock().unwrap();
            *data += 1;
        });
        handles.push(handle);
    }

    // スレッドの終了を待つ
    for handle in handles {
        handle.join().unwrap();
    }

    // 共有メモリ内のデータを表示
    let result = data.lock().unwrap();
    println!("Final result: {}", *result);
}
