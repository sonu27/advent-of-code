package main

import (
	"aoc/lib"
	"bufio"
	"fmt"
	"os"
	"slices"
	"strconv"
	"strings"
)

func main() {
	file, err := os.Open("./2024/07/input.txt")
	if err != nil {
		panic(err)
	}
	defer file.Close()

	var p1, p2 int64

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		tmp := strings.Split(scanner.Text(), ": ")
		num, _ := strconv.ParseInt(tmp[0], 10, 64)
		nums := lib.StringToIntSlice(tmp[1], " ")

		if check2(int(num), nums, 0) {
			p2 += num
		}

		slices.Reverse(nums)
		if check(int(num), nums) {
			p1 += num
		}
	}

	fmt.Println("Part 1: ", p1)
	fmt.Println("Part 2: ", p2)
}

func check(num int, nums []int) bool {
	if num == 0 {
		return true
	}

	if num < 0 || len(nums) == 0 {
		return false
	}

	if num%nums[0] == 0 && check(num/nums[0], nums[1:]) {
		return true
	}

	return check(num-nums[0], nums[1:])
}

func check2(target int, nums []int, curr int) bool {
	if len(nums) == 0 {
		return curr == target
	}

	if check2(target, nums[1:], curr+nums[0]) {
		return true
	}

	if check2(target, nums[1:], curr*nums[0]) {
		return true
	}

	return check2(target, nums[1:], concatenate(curr, nums[0]))
}

func concatenate(a, b int) int {
	multiplier := 1
	for b/multiplier > 0 {
		multiplier *= 10
	}
	return a*multiplier + b
}
