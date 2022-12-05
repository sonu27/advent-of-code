package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	part1()
	part2()
}

func part1() {
	b, err := os.ReadFile("./2022/05/input.txt")
	if err != nil {
		panic(err)
	}

	in := strings.Split(string(b), "\n\n")
	strs := strings.Split(in[0], "\n")
	stacks := buildStacks(strs)

	strs = strings.Split(in[1], "\n")
	for _, v := range strs {
		quantity, from, to := 0, 0, 0
		_, err := fmt.Sscanf(v, "move %d from %d to %d", &quantity, &from, &to)
		if err != nil && err.Error() != "unexpected EOF" {
			panic(err)
		}
		makeMove(quantity, from, to, stacks)
	}

	p1 := ""
	for _, v := range stacks[1:] {
		p1 += v.Peek()
	}

	fmt.Println("Part1:", p1)
}

func part2() {
	b, err := os.ReadFile("./2022/05/input.txt")
	if err != nil {
		panic(err)
	}

	in := strings.Split(string(b), "\n\n")
	strs := strings.Split(in[0], "\n")
	stacks := buildStacks(strs)

	strs = strings.Split(in[1], "\n")
	for _, v := range strs {
		quantity, from, to := 0, 0, 0
		_, err := fmt.Sscanf(v, "move %d from %d to %d", &quantity, &from, &to)
		if err != nil && err.Error() != "unexpected EOF" {
			panic(err)
		}
		makeMoveP2(quantity, from, to, stacks)
	}

	p2 := ""
	for _, v := range stacks[1:] {
		p2 += v.Peek()
	}

	fmt.Println("Part2:", p2)
}

func buildStacks(in []string) []Stack[string] {
	tmp := strings.Split(in[len(in)-1], " ")
	numOfRows, err := strconv.Atoi(tmp[len(tmp)-1])
	if err != nil {
		panic(err)
	}

	stacks := make([]Stack[string], numOfRows+1)

	for i := len(in) - 2; i > -1; i-- {
		k := 0
		row := 1
		for j := 0; j < len(in[i]); j++ {
			if j%2 == 1 {
				if k%2 == 0 {
					if in[i][j] > 36 {
						stacks[row].Push(string(in[i][j]))
					}
					row++
				}
				k++
			}
		}
	}

	return stacks
}

func makeMove(quantity, from, to int, stacks []Stack[string]) {
	for i := 0; i < quantity; i++ {
		stacks[to].Push(stacks[from].Pop())
	}
}

func makeMoveP2(quantity, from, to int, stacks []Stack[string]) {
	tmp := make([]string, quantity)
	for i := 0; i < quantity; i++ {
		tmp[i] = stacks[from].Pop()
	}
	for i := quantity - 1; i > -1; i-- {
		stacks[to].Push(tmp[i])
	}
}

type Stack[T any] struct {
	values []T
}

func (s *Stack[T]) Push(v T) {
	s.values = append(s.values, v)
}

func (s *Stack[T]) Pop() T {
	v := s.values[len(s.values)-1]
	s.values = s.values[:len(s.values)-1]
	return v
}

func (s *Stack[T]) Peek() T {
	return s.values[len(s.values)-1]
}

func (s *Stack[T]) IsEmpty() bool {
	return len(s.values) == 0
}

func (s *Stack[T]) Size() int {
	return len(s.values)
}

func (s *Stack[T]) Clear() {
	s.values = nil
}
