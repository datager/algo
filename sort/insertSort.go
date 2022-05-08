package main

import (
	"log"
	"time"
)

type insertSort struct {
	arr []int64
}

func NewInsertSort(arr []int64) *insertSort {
	return &insertSort{arr: arr}
}

func (s *insertSort) Sort() []int64 {
	st := time.Now()
	for i := range s.arr { // i: 每个新元素
		value := s.arr[i] // 因为有后移操作，会破坏下标寻址，故须提前保存到变量里
		k := i - 1
		for ; k >= 0; k-- { // k: 每个已有元素, 从后向前
			if value < s.arr[k] { // 有逆序
				s.arr[k+1] = s.arr[k] // 后移
			} else {
				break
			}
		}
		s.arr[k+1] = value // 为value找到了正确的插入位置
	}
	log.Printf("cost: %v", time.Since(st))
	return s.arr
}
