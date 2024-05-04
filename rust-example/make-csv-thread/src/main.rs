use std::fs::File;
use std::io::BufWriter;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;
use csv::Writer;

// 1つのスレッドが処理する行数
const CHUNK_SIZE: usize = 100_000;
// スレッド数
const NUM_THREADS: usize = 4;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // CSVファイルの作成
    let file = File::create("output.csv")?;
    // バッファリングされたライターの作成
    let writer = BufWriter::new(file);
    // CSVライターを作成
    let csv_writer = Arc::new(Mutex::new(Writer::from_writer(writer)));

    // データ生成の開始行と終了行を計算
    let lines_per_thread = 100_000_000 / NUM_THREADS;

    // マルチスレッド用のベクタを用意
    let mut handles = vec![];

    // 時間計測の開始
    let start_time = Instant::now();

    // 複数のスレッドを生成してデータの書き込みを行う
    for thread_id in 0..NUM_THREADS {
        // 各スレッドの開始行と終了行を計算
        let start = thread_id * lines_per_thread;
        let end = if thread_id == NUM_THREADS - 1 {
            // 最後のスレッドは全行をカバーする
            100_000_000
        } else {
            start + lines_per_thread
        };

        // スレッドを作成
        let csv_writer_clone = Arc::clone(&csv_writer);
        let handle = thread::spawn(move || {
            for i in start..end {
                // CSV行を生成
                let row = vec![
                    (i + 1).to_string(), // 行番号
                    format!("Value {}", i + 1), // 任意の値
                    format!("Data {}", i + 1), // 任意のデータ
                ];
                // CSVライターにロックをかけて行を書き込み
                let mut csv_writer_lock = csv_writer_clone.lock().unwrap();
                csv_writer_lock.write_record(&row).unwrap();
            }
        });

        // スレッドのハンドルを保存
        handles.push(handle);
    }

    // 全てのスレッドが終了するまで待つ
    for handle in handles {
        handle.join().unwrap();
    }

    // ファイルへの書き込みを確定
    csv_writer.lock().unwrap().flush()?;

    // 時間計測の終了
    let duration = start_time.elapsed();
    println!("CSVファイルの生成が完了しました。所要時間: {:.2?}", duration);

    Ok(())
}
