package main

import (
	"fmt"
	"os"
)

func main() {
	b, err := os.ReadFile("./2022/06/input.txt")
	if err != nil {
		panic(err)
	}

	fmt.Println("Part 1:", findXUniqueChars(string(b), 4))
	fmt.Println("Part 2:", findXUniqueChars(string(b), 14))
}

func findXUniqueChars(in string, num int) int {
	lastX := make([]string, 0, num)
	for i, c := range in {
		lastX = append(lastX, string(c))

		// remove the first element if the slice is too long
		if len(lastX) > num {
			lastX = lastX[1:]
		}

		if len(lastX) == num && len(set(lastX)) == num {
			return i + 1
		}
	}

	return -1
}

// set returns a slice containing only the unique strings from the input slice.
func set(input []string) []string {
	// Create a map to hold the unique strings
	uniqueStrings := make(map[string]struct{})

	// Iterate over the input slice and add each string to the map
	for _, s := range input {
		uniqueStrings[s] = struct{}{}
	}

	// Create a slice to hold the unique strings
	result := make([]string, 0, len(uniqueStrings))

	// Iterate over the unique strings in the map and add them to the slice
	for s := range uniqueStrings {
		result = append(result, s)
	}

	// Return the slice of unique strings
	return result
}
