package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strings"
)

func main() {
	file, err := os.Open("input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)

	world := make(map[string]string)

	for scanner.Scan() {
		s := strings.Split(scanner.Text(), ")")
		world[s[1]] = s[0]
	}

	part1(world)

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}
}

func part1(world map[string]string) {
	sum := 0

	for i, _ := range world {
		sum += getSteps(world, i, "COM")
	}

	fmt.Println(sum)
}

func part2(world map[string]string) {
	youPath := getToCOMPath(world, "YOU")
	sanPath := getToCOMPath(world, "SAN")

	var found string
	for _, y := range youPath {
		for _, s := range sanPath {
			if y == s {
				found = s
				break
			}
		}
		if found == y {
			break
		}
	}

	sum := getSteps(world, "YOU", found) - 1
	sum += getSteps(world, "SAN", found) - 1
	fmt.Println(sum)
}

func getToCOMPath(world map[string]string, obj string) []string {
	o := obj
	var path []string
	for world[o] != "COM" {
		o = world[o]
		path = append(path, o)
	}

	return path
}

func getSteps(world map[string]string, obj string, to string) int {
	sum := 1
	o := obj
	for world[o] != to {
		o = world[o]
		sum++
	}

	return sum
}
