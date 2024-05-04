use std::fs::File;
use std::io::{BufWriter, Write};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Instant;
use csv::Writer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 出力先のCSVファイルを作成
    let file = File::create("output.csv")?;
    // バッファリングされたライターを作成
    let writer = BufWriter::new(file);
    // CSVライターを作成
    let csv_writer = Arc::new(Mutex::new(Writer::from_writer(writer)));

    // ゴルーチンでデータ生成と書き込みを行うためのチャネル
    let (tx, rx) = mpsc::channel();

    // データ生成用スレッド
    let data_thread = {
        let tx = tx.clone();
        thread::spawn(move || {
            for i in 0..100_000_000 {
                // CSV行を生成
                let row = vec![
                    (i + 1).to_string(), // 行番号
                    format!("Value {}", i + 1), // 任意の値
                    format!("Data {}", i + 1), // 任意のデータ
                ];
                // チャネルにデータを送信
                tx.send(row).expect("データ送信エラー");
            }
            // データ生成完了後にチャネルを閉じる
            drop(tx);
        })
    };

    // ファイル書き込み用スレッド
    let write_thread = {
        let csv_writer = Arc::clone(&csv_writer);
        thread::spawn(move || {
            for row in rx {
                // CSVライターを使ってファイルに行を書き込み
                let mut csv_writer = csv_writer.lock().expect("ライターロックエラー");
                csv_writer.write_record(&row).expect("CSVへの書き込みエラー");
            }
        })
    };

    // データ生成スレッドとファイル書き込みスレッドの終了を待つ
    data_thread.join().expect("データ生成スレッドエラー");
    write_thread.join().expect("ファイル書き込みスレッドエラー");

    // CSVライターをフラッシュして書き込みを確定
    csv_writer.lock().unwrap().flush()?;

    println!("CSVファイルの生成が完了しました。");
    Ok(())
}
