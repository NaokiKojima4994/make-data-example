package main

import (
	"flag"
	"fmt"
)

// go run command-line-args.go -s hoge -i 10 -b true
func main() {
	// flagを実行
	parseArgs()
}
// flag
func parseArgs() {
	//変数でflagを定義します
	var (
		s = flag.String("s", "default message.", "string flag")
		i = flag.Int("i", 0, "int flag")
		b = flag.Bool("b", false, "bool flag")
	)
	//ここで解析されます
	flag.Parse()

	fmt.Printf("param -s : %s\n", *s)
	fmt.Printf("param -i : %d\n", *i)
	fmt.Printf("param -b : %t\n", *b)
}