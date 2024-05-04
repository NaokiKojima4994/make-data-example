use std::fs::File;
use std::io::{BufWriter, Write};
use csv::Writer;
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 出力先のCSVファイルを作成
    let file = File::create("output.csv")?;
    // バッファリングされたライターを作成
    let writer = BufWriter::new(file);
    // CSVライターを作成
    let mut csv_writer = Writer::from_writer(writer);

    // 1億行のCSVデータを生成して書き込み
    let total_lines = 100_000_000;

    // 時間計測の開始
    let start_time = Instant::now();

    // データ生成とCSVファイルへの書き込み
    for i in 0..total_lines {
        // CSV行を生成
        let row = vec![
            (i + 1).to_string(),  // 行番号
            format!("Value {}", i + 1), // 任意の値
            format!("Data {}", i + 1), // 任意のデータ
        ];
        // CSVライターを使って行を書き込み
        csv_writer.write_record(&row)?;
    }

    // ファイルへの書き込みを確定
    csv_writer.flush()?;

    // 時間計測の終了
    let duration = start_time.elapsed();
    println!("CSVファイルの生成が完了しました。所要時間: {:.2?}", duration);

    Ok(())
}
