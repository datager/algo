package main

import (
	"fmt"
)

type SingleArrQueue struct {
	arr  []int
	cap  int
	head int
	tail int
}

func NewArrQueue(cap int) SingleArrQueue {
	return SingleArrQueue{
		arr:  make([]int, cap, cap), // 原计划用原生array实现, 但可用[3]int定义, 却不可用[cap]int定义, 故用slice实现
		cap:  cap,
		head: 0,
		tail: 0,
	}
}

// 固定cap的queue, 当head==tail时触发搬迁O(N)
func (q *SingleArrQueue) Push(v int) bool {
	if q.tail == q.cap {
		if q.head == 0 {
			fmt.Printf("push fail because full, head: %v, tail: %v\n", q.head, q.tail)
			return false
		}
		for i := q.head; i < q.cap; i++ {
			q.arr[i-q.head] = q.arr[i]
		}
		q.tail -= q.head
		q.head = 0
		fmt.Printf("rebalance to head: %v, tail: %v\n", q.head, q.tail)
	}
	q.arr[q.tail] = v
	q.tail++
	fmt.Printf("push %v ok, head: %v, tail: %v\n", v, q.head, q.tail)
	return true
}

func (q *SingleArrQueue) Pop() bool {
	if q.head == q.tail {
		fmt.Printf("pop fail, head: %v, tail: %v\n", q.head, q.tail)
		return false
	}
	v := q.arr[q.head]
	q.head++
	fmt.Printf("pop %v ok, head: %v, tail: %v\n", v, q.head, q.tail)
	return true
}
