package lib

import (
	"strconv"
	"strings"
)

func StringToIntSlice(str string) []int {
	tmp := strings.Split(str, " ")

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
