const fs = require('fs');
const { Writable } = require('stream');
const path = require('path');

// 出力先のファイルを作成
const filePath = path.join(__dirname, 'output.csv');
const writeStream = fs.createWriteStream(filePath);

// 書き込みを効率的に行うためのカスタム Writable ストリーム
class CSVWriter extends Writable {
    constructor(options) {
        super(options);
    }

    _write(chunk, encoding, callback) {
        // ファイルにデータを書き込み
        writeStream.write(chunk, encoding, callback);
    }
}

// CSVライターのインスタンスを作成
const csvWriter = new CSVWriter();

// 1億行のCSVデータを生成して書き込み
const totalLines = 100_000_000;

(async function generateCSVData() {
    console.time('CSV Generation Time');

    // CSVヘッダーを最初に書き込む
    const header = 'ID,Value,Data\n';
    csvWriter.write(header);

    // 1億行のデータを生成して書き込み
    for (let i = 0; i < totalLines; i++) {
        // CSV行を生成
        const id = i + 1;
        const value = `Value ${id}`;
        const data = `Data ${id}`;
        const csvRow = `${id},${value},${data}\n`;

        // CSV行を書き込み
        const writeSuccessful = csvWriter.write(csvRow);
        if (!writeSuccessful) {
            // ストリームのバッファが一杯になった場合、データの書き込みが処理されるまで待つ
            await new Promise((resolve) => csvWriter.once('drain', resolve));
        }
    }

    // 書き込みを完了してストリームを終了
    csvWriter.end();
    writeStream.end();

    console.timeEnd('CSV Generation Time');
})();

// 書き込み終了時のイベントをリスン
writeStream.on('finish', () => {
    console.log('CSVファイルの生成が完了しました。');
});