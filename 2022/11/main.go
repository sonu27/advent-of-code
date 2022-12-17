package main

import (
	"fmt"
	"os"
	"sort"
	"strconv"
	"strings"
)

type Monkey struct {
	startingItem []int
	s1, s2, s3   string
	divisor      int
	trueMonkey   int
	falseMonkey  int
	count        int
}

var modulo = 1

func main() {
	b, err := os.ReadFile("./2022/11/input.txt")
	if err != nil {
		panic(err)
	}

	var ms []Monkey

	in := strings.Split(string(b), "\n\n")
	for _, v := range in {
		var m Monkey

		ss := strings.Split(v, "\n")
		tmp := strings.TrimPrefix(ss[1], "  Starting items: ")
		tmpArr := strings.Split(tmp, ", ")
		tmpArrInt := make([]int, len(tmpArr))
		for i, v := range tmpArr {
			tmpArrInt[i], _ = strconv.Atoi(v)
		}
		m.startingItem = tmpArrInt

		fmt.Sscanf(ss[2], "  Operation: new = %s %s %s", &m.s1, &m.s2, &m.s3)
		fmt.Sscanf(ss[3], "  Test: divisible by %d", &m.divisor)
		modulo *= m.divisor
		fmt.Sscanf(ss[4], "    If true: throw to monkey %d", &m.trueMonkey)
		fmt.Sscanf(ss[5], "    If false: throw to monkey %d", &m.falseMonkey)

		ms = append(ms, m)
	}

	game(ms, 20, true)
	game(ms, 10000, false)
}

func game(ms []Monkey, rounds int, part1 bool) {
	modulo = 1
	for i := 0; i < rounds; i++ {
		for j := range ms {
			m := &ms[j]
			for len(m.startingItem) > 0 {
				v := m.startingItem[0]
				old := false
				s3, err := strconv.Atoi(m.s3)
				if err != nil {
					old = true
				}
				wl := v
				if m.s2 == "+" {
					if old {
						wl += wl
					} else {
						wl += s3
					}
				}
				if m.s2 == "*" {
					if old {
						wl *= wl
					} else {
						wl *= s3
					}
				}

				if part1 {
					wl /= 3
				} else {
					wl %= modulo
				}

				if wl%m.divisor == 0 {
					ms[m.trueMonkey].startingItem = append(ms[m.trueMonkey].startingItem, wl)
				} else {
					ms[m.falseMonkey].startingItem = append(ms[m.falseMonkey].startingItem, wl)
				}

				m.count++
				m.startingItem = m.startingItem[1:]
			}
		}
	}

	sort.Slice(ms, func(i, j int) bool {
		return ms[i].count > ms[j].count
	})

	for _, v := range ms {
		fmt.Println(v.count)
	}

	fmt.Println("Part:", ms[0].count*ms[1].count)
}
