package main

import (
	"fmt"
	"io/ioutil"
	"strings"
)

func main() {
	input, _ := ioutil.ReadFile("../input.txt")
	image := strings.TrimSpace(string(input))

	const numberOfPixel = 25 * 6
	var imageDecoded [numberOfPixel]rune

	// We store for each pixel the first digit different from '2'
	for index, digit := range image {
		if digit != '2' && imageDecoded[index%numberOfPixel] == 0 {
			imageDecoded[index%numberOfPixel] = digit
		}
	}

	//We can now print the image, I used the '█' because it's clearer than the '1'
	for i, pixel := range imageDecoded {
		if i%25 == 0 {
			fmt.Println()
		}
		if pixel == '0' {
			fmt.Print(" ")
		} else {
			fmt.Print("█")
		}
	}
}
