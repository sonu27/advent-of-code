package main

import (
	"bufio"
	"fmt"
	"log"
	"math"
	"os"
	"strconv"
)

func main() {
	file, err := os.Open("../input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)

	sum := 0
	for scanner.Scan() {
		i, _ := strconv.Atoi(scanner.Text())
		x := doCalc(i)
		sum += x

		y := x
		for {
			y = doCalc(y)

			if y < 1 {
				break
			} else {
				sum += y
			}
		}
	}

	fmt.Println(sum)

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}
}

func doCalc(num int) int {
	x := num / 3
	y := math.Floor(float64(x))

	return int(y - 2)
}
