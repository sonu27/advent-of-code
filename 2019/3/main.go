package main

import "fmt"

type Point struct {
	X, Y int
}

type Wire struct {
	data map[Point]int
}

func (w Wire) Get(x, y int) int {
	v, ok := w.data[Point{X: x, Y: y}]
	if ok {
		return v
	}
	return 0
}

func (w *Wire) Set(x, y, d int) {
	xy := Point{X: x, Y: y}
	w.data[xy] = d
}

func main() {
	w1 := Wire{data: map[Point]int{}}
	w1.Set(1, 1, 1)

	p1 := Point{
		X: 1,
		Y: 1,
	}
	if i, ok := w1.data[p1]; ok {
		fmt.Println(i)
	}

	p2 := w1.Get(1, 1)
	fmt.Println(p2)
}
