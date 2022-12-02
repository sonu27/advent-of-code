package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	file, err := os.Open("./2022/02/input.txt")
	if err != nil {
		panic(err)
	}

	m1 := map[string]int{
		"A X": 4, // draw
		"A Y": 8, // win
		"A Z": 3, // lose
		"B X": 1, // lose
		"B Y": 5, // draw
		"B Z": 9, // win
		"C X": 7, // win
		"C Y": 2, // lose
		"C Z": 6, // draw
	}

	// the second column says how the round needs to end: X means you need to lose, Y means you need to end the round in a draw, and Z means you need to win.
	m2 := map[string]int{
		"A X": 3, // rock - lose - scissors
		"A Y": 4, // rock - draw - rock
		"A Z": 8, // rock - win - paper
		"B X": 1, // paper - lose - rock
		"B Y": 5, // paper - draw - paper
		"B Z": 9, // paper - win - scissors
		"C X": 2, // scissors - lose - paper
		"C Y": 6, // scissors - draw - scissors
		"C Z": 7, // scissors - win - rock
	}

	scanner := bufio.NewScanner(file)
	scoreP1 := 0
	scoreP2 := 0
	for scanner.Scan() {
		line := scanner.Text()
		scoreP1 += m1[line]
		scoreP2 += m2[line]
	}

	fmt.Println("Part 1:", scoreP1)
	fmt.Println("Part 2:", scoreP2)
}
