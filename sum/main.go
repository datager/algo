package main

func main() {
	println(sumInts(map[string]int64{"a": 1, "b": 2}))
	println(sumFloats(map[string]float64{"a": 1.1, "b": 2.2}))
	println(sumIntsOrFloats(map[string]int64{"a": 1, "b": 2}))
	println(sumIntsOrFloats(map[string]float64{"a": 1.1, "b": 2.2}))
	println(sumNumbers(map[string]int64{"a": 1, "b": 2}))
	println(sumNumbers(map[string]float64{"a": 1.1, "b": 2.2}))
}

func sumInts(m map[string]int64) int64 {
	var s int64
	for _, v := range m {
		s += v
	}
	return s
}

func sumFloats(m map[string]float64) float64 {
	var s float64
	for _, v := range m {
		s += v
	}
	return s
}

// sumIntsOrFloats sums the values of map m. It supports both int64 and float64
// as types for map values.
func sumIntsOrFloats[K comparable, V int64 | float64](m map[K]V) V {
	var s V
	for _, v := range m {
		s += v
	}
	return s
}

type Number interface {
	int64 | float64
}

func sumNumbers[K comparable, V Number](m map[K]V) V {
	var s V
	for _, v := range m {
		s += v
	}
	return s
}
