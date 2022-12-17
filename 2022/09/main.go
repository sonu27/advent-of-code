package main

import (
	"bufio"
	"fmt"
	"image"
	"os"
)

func main() {
	p1()
	p2()
}
func p1() {
	file, err := os.Open("./2022/09/input.txt")
	if err != nil {
		panic(err)
	}

	h := image.Point{}
	t := image.Point{}
	m := make(map[image.Point]struct{})
	m[t] = struct{}{}

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		var move string
		var num int
		_, err := fmt.Sscanf(scanner.Text(), "%s %d", &move, &num)
		if err != nil && err.Error() != "unexpected EOF" {
			panic(err)
		}

		for i := 0; i < num; i++ {
			switch move {
			case "U":
				h = h.Add(image.Point{Y: 1})
				if d := h.Sub(t); d.X > 1 || d.Y > 1 || d.X < -1 || d.Y < -1 {
					t = image.Point{
						X: h.X,
						Y: h.Y - 1,
					}
					m[t] = struct{}{}
				}
			case "D":
				h = h.Add(image.Point{Y: -1})
				if d := h.Sub(t); d.X > 1 || d.Y > 1 || d.X < -1 || d.Y < -1 {
					t = image.Point{
						X: h.X,
						Y: h.Y + 1,
					}
					m[t] = struct{}{}
				}
			case "L":
				h = h.Add(image.Point{X: -1})
				if d := h.Sub(t); d.X > 1 || d.Y > 1 || d.X < -1 || d.Y < -1 {
					t = image.Point{
						X: h.X + 1,
						Y: h.Y,
					}
					m[t] = struct{}{}
				}
			case "R":
				h = h.Add(image.Point{X: 1})
				if d := h.Sub(t); d.X > 1 || d.Y > 1 || d.X < -1 || d.Y < -1 {
					t = image.Point{
						X: h.X - 1,
						Y: h.Y,
					}
					m[t] = struct{}{}
				}
			}
		}
	}

	count := 0
	for range m {
		count++
	}

	fmt.Println("Part 1:", count)
}

func p2() {
	file, err := os.Open("./2022/09/input.txt")
	if err != nil {
		panic(err)
	}

	m := make(map[image.Point]struct{})
	var a [10]image.Point
	for i := 0; i < 10; i++ {
		a[i] = image.Point{}
	}

	mxy := map[string]image.Point{
		"U": {Y: 1},
		"D": {Y: -1},
		"L": {X: -1},
		"R": {X: 1},
	}

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		var move string
		var num int
		_, err := fmt.Sscanf(scanner.Text(), "%s %d", &move, &num)
		if err != nil && err.Error() != "unexpected EOF" {
			panic(err)
		}

		for i := 0; i < num; i++ {
			a[0] = a[0].Add(mxy[move])

			for j := 0; j < 9; j++ {
				h := &a[j]
				t := &a[j+1]

				if d := h.Sub(*t); d.X > 1 || d.Y > 1 || d.X < -1 || d.Y < -1 {
					if d.X == 0 {
						t.Y += d.Y / 2
					} else if d.Y == 0 {
						t.X += d.X / 2
					} else {
						if d.X > 0 {
							t.X++
						} else {
							t.X--
						}

						if d.Y > 0 {
							t.Y++
						} else {
							t.Y--
						}
					}
				}
			}
			m[image.Point{
				X: a[9].X,
				Y: a[9].Y,
			}] = struct{}{}
		}
	}

	count := 0
	for range m {
		count++
	}

	fmt.Println("Part 2:", count)
}
