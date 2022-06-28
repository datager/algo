package main

import (
	"log"
)

func main() {
	//arr := []int64{4,6,5,2,1,3}
	//log.Print("BubbleSort: ", NewBubbleSort(arr).Sort())
	//arr = []int64{4,6,5,2,1,3}
	//log.Print("InsertSort: ", NewInsertSort(arr).Sort())
	arr := []int64{4, 6, 5, 2, 1, 3}
	log.Print("InsertSort: ", NewMergeSort(arr).Sort())
}
