package main

import "fmt"

// CycleQueue 当无环queue满时需搬迁数据O(N), 为避免可用环形queue
// 当head==tail时, queue空
// 当(head+1)%cap==tail时, queue满, 此时tail未存数据(即环形queue会浪费一个存储空间, 只能存cap-1个)
type CycleArrQueue struct {
	head int
	tail int
	arr  []int
	cap  int
}

func NewCycleArrQueue(cap int) *CycleArrQueue {
	cq := &CycleArrQueue{
		head: 0,
		tail: 0,
		arr:  make([]int, cap, cap),
		cap:  cap,
	}
	fmt.Printf("NewCycleArrQueue, %s\n", cq)
	return cq
}

func (cq *CycleArrQueue) String() string {
	return fmt.Sprintf("head: %v, tail: %v", cq.head, cq.tail)
}

func (cq *CycleArrQueue) Push(v int) bool {
	if (cq.tail+1)%cq.cap == cq.head {
		fmt.Printf("push fail, %s\n", cq)
		return false
	}
	cq.arr[cq.tail] = v
	cq.tail = (cq.tail + 1) % cq.cap
	fmt.Printf("push %v ok, %s\n", v, cq)
	return true
}

func (cq *CycleArrQueue) Pop() {
	if cq.head == cq.tail {
		fmt.Printf("pop fail, %s\n", cq)
		return
	}
	v := cq.arr[cq.head]
	cq.head = (cq.head + 1) % cq.cap
	fmt.Printf("pop %v ok, %s\n", v, cq)
}
