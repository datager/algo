package main

import "fmt"

// 环形链表queue, tail后接head形成环, 每次在tail和head之间Push()和Pop(), 很高效
type CycleListQueue struct {
	head *Node
	tail *Node
}

func NewCycleListQueue() *CycleListQueue {
	head := &Node{
		Elem: -1,
		Next: nil,
	}
	tail := &Node{
		Elem: -2,
		Next: nil,
	}
	// 刚开始head与tail即成环
	head.Next = tail
	tail.Next = head
	clq := &CycleListQueue{
		head: head,
		tail: tail,
	}
	fmt.Printf("NewCycleQueue, %s\n", clq.String())
	return clq
}

func (clq *CycleListQueue) String() string {
	s := ""
	iter := clq.head
	for iter.Next != clq.head {
		s += fmt.Sprintf("%v->", iter.Elem)
		iter = iter.Next
	}
	s += fmt.Sprintf("%v", clq.tail.Elem)
	return s
}

// Push() head=>新node=>...=>tai=>head
func (clq *CycleListQueue) Push(v int) {
	node := &Node{
		Elem: v,
		Next: clq.head.Next,
	}
	clq.head.Next = node
	fmt.Printf("push %v ok, %s\n", v, clq.String())
}

// Pop() head=>被移除的node=>...=>tai=>head
func (clq *CycleListQueue) Pop() bool {
	if clq.head.Next == clq.tail {
		fmt.Printf("pop fail, %s\n", clq)
		return false
	}
	v := clq.head.Next.Elem
	clq.head.Next = clq.head.Next.Next
	fmt.Printf("pop %v ok, %s\n", v, clq)
	return true
}
