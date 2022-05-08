package main

import (
	"log"
	"time"
)

type bubbleSort struct {
	arr []int64
}

func NewBubbleSort(arr []int64) *bubbleSort {
	return &bubbleSort{arr: arr}
}

func (s *bubbleSort) Sort() []int64 {
	st := time.Now()
	sz := len(s.arr)
	if sz < 3 {
		return s.arr
	}
	for i := 0; i < sz-1; i++ {
		hasExchange := false
		for k := 0; k < sz-1-i; k++ {
			if s.arr[k] < s.arr[k+1] {
				tmp := s.arr[k]
				s.arr[k] = s.arr[k+1]
				s.arr[k+1] = tmp
				hasExchange = true
			}
		}
		if !hasExchange {
			break
		}
	}
	log.Printf("cost: %v", time.Since(st))
	return s.arr
}
