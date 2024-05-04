package main

import (
	"encoding/csv"
	"fmt"
	"os"
	"strconv"
	"sync"
)

func main() {
	// 出力先のCSVファイルを作成
	file, err := os.Create("output.csv")
	if err != nil {
		fmt.Println("Error creating file:", err)
		return
	}
	defer file.Close()

	// バッファリングされたライターを作成
	writer := csv.NewWriter(file)
	defer writer.Flush() // 書き込みを確定してファイルを閉じる

	// ゴルーチンでデータ生成と書き込みを行うためのチャネル
	dataChannel := make(chan []string, 100)
	var wg sync.WaitGroup

	// データ生成を行うゴルーチン
	wg.Add(1)
	go func() {
		defer wg.Done()
		for i := 0; i < 100_000_000; i++ {
			// CSVの行を生成
			row := []string{
				strconv.Itoa(i + 1), // 行番号
				"Value " + strconv.Itoa(i + 1), // 任意の値
				"Data " + strconv.Itoa(i + 1), // 任意のデータ
			}
			// チャネルにデータを送信
			dataChannel <- row
		}
		// データ生成完了後にチャネルを閉じる
		close(dataChannel)
	}()

	// ファイル書き込みを行うゴルーチン
	wg.Add(1)
	go func() {
		defer wg.Done()
		for row := range dataChannel {
			// CSVライターを使ってファイルに行を出力
			err := writer.Write(row)
			if err != nil {
				fmt.Println("Error writing to file:", err)
				return
			}
		}
	}()

	// 全てのゴルーチンの完了を待つ
	wg.Wait()
	fmt.Println("CSVファイルの生成が完了しました。")
}