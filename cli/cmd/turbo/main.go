package main

import "C"
import (
	"fmt"
	"os"
)

func main() {
	fmt.Printf("ERROR: Go binary cannot be used on its own. Please build as c-archive and use with Rust crate")
	os.Exit(1)
}

//export nativeRunWithArgs
func nativeRunWithArgs(argsString string) C.uint {
	fmt.Printf("Args: %v\n", argsString)
	return C.uint(0)
}
