package main

import (
	"log"
	"math/rand"
)

func main() {
	arr := make([]int64, 0)
	for i := 0; i < 200; i++ {
		arr = append(arr, int64(rand.Int()))
	}
	log.Print("BubbleSort: ", NewBubbleSort(arr).Sort())
	arr = make([]int64, 0)
	for i := 0; i < 200; i++ {
		arr = append(arr, int64(rand.Int()))
	}
	log.Print("InsertSort: ", NewInsertSort(arr).Sort())
}
