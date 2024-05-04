package main

import (
	"encoding/csv"
	"fmt"
	"os"
)

func main() {
	records := [][]string{
		[]string{"名前", "年齢", "出身地", "性別"},
		[]string{"山本", "24", "兵庫", "男"},
	}

	f, err := os.Create("test.csv")
	if err != nil {
		fmt.Println(err)
	}

	w := csv.NewWriter(f)
	w.WriteAll(records)
}