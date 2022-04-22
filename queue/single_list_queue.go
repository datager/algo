package main

import (
	"fmt"
)

type SingleListQueue struct {
	head *Node
	tail *Node
}

type Node struct {
	Elem int
	Next *Node
}

func NewListQueue() SingleListQueue {
	sentinel := &Node{
		Elem: -1,
		Next: nil,
	}
	l := SingleListQueue{
		head: sentinel,
		tail: sentinel,
	}
	fmt.Printf("NewListQueue, %v\n", l.String())
	return l
}

func (lq *SingleListQueue) Push(v int) {
	lq.tail.Next = &Node{
		Elem: v,
		Next: nil,
	}
	lq.tail = lq.tail.Next
	fmt.Printf("push %v ok, %v\n", v, lq)
}

func (lq *SingleListQueue) Pop() bool {
	if lq.head == nil || lq.head.Next == nil {
		fmt.Printf("pop fail, %v\n", lq)
		return false
	}
	// for log
	v := lq.head.Next.Elem
	// change head
	next := lq.head.Next.Next
	lq.head.Next = next
	// change tail if need
	if lq.head.Next == nil {
		lq.tail = lq.head
	}
	fmt.Printf("pop %v ok, %v\n", v, lq)
	return true
}

func (lq *SingleListQueue) String() string {
	s := ""
	iter := lq.head
	for iter != nil {
		s += fmt.Sprintf("%v->", iter.Elem)
		iter = iter.Next
	}
	return s
}
