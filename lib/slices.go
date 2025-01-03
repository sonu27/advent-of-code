package lib

import (
	"strconv"
	"strings"
)

func StringToIntSlice(str string, sep string) []int {
	tmp := strings.Split(str, sep)

	var out []int
	for _, x := range tmp {
		i, err := strconv.Atoi(x)
		if err != nil {
			panic(err)
		}

		out = append(out, i)
	}

	return out
}

func StringToInt64Slice(str string, sep string) []int64 {
	tmp := strings.Split(str, sep)

	var out []int64
	for _, x := range tmp {
		i, err := strconv.ParseInt(x, 10, 64)
		if err != nil {
			panic(err)
		}

		out = append(out, i)
	}

	return out
}
