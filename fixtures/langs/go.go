// This is a Go comment
package main

import "fmt"

type Person struct {
	Name string
	Age  int
}

func greet(name string) string {
	message := "Hello, " + name
	return message
}

func main() {
	userName := "World"
	fmt.Println(greet(userName))
}
