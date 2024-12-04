package lib

func Difference[T int | float64](a, b T) T {
	if a > b {
		return a - b
	}
	return b - a
}
