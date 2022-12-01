package main

import (
	"fmt"
	"strconv"
	"strings"
)

func main() {
	start := 125730
	end := 579381
	//p1: 2081 p2: 1411

	count := 0
	for i := start; i <= end; i++ {
		nums := convert(i)
		if checkV2(nums) {
			count++
		}
	}

	fmt.Println(count)
}

func convert(num int) []int {
	str := strconv.Itoa(num)
	strArr := strings.Split(str, "")

	intArr := make([]int, len(strArr))
	for i, v := range strArr {
		num, _ := strconv.Atoi(v)
		intArr[i] = num
	}

	return intArr
}

func checkV1(nums []int) bool {
	adjacent := false

	for i := 0; i < len(nums) - 1; i++ {
		if nums[i] > nums[i + 1] {
			return false
		}

		if nums[i] == nums[i + 1] {
			adjacent = true
		}
	}

	return adjacent
}

func checkV2(nums []int) bool {
	onlyTwoAdjacentSame := false
	matchLen := 1

	for i := 0; i < len(nums) - 1; i++ {
		if nums[i] > nums[i + 1] {
			return false
		}

		if nums[i] == nums[i+1] {
			matchLen++
		} else if matchLen == 2 {
			onlyTwoAdjacentSame = true
		} else {
			matchLen = 1
		}
	}

	return onlyTwoAdjacentSame || matchLen == 2
}