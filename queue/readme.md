[toc]

体会: 练算法还是先用熟悉的语言, 否则用新语言可能会因语法而劝退, 其实算法就是个思路, 可以分为以下3个阶段
1. 先用go实现一遍算法, 要上手自己写, 用于学思路, 方便后续看源码里的各种算法
2. 再用新语言(如rust)写一遍, 用于学新语言, 用于后续看新语言写的代码

队列就是尾进,头出, 很多开源基础组件都用到了queue, 其用array或list均可实现

# 无环
## 用array实现
因在操作点有头尾2处, 故可维护2个指针, 分别指向头和尾
下面是我们的实现
![image.png](https://note.youdao.com/yws/res/121809/WEBRESOURCE405ecccae3bd7840bada85707bb0b177)
```go
package main

import (
	"fmt"
)

type ArrQueue struct {
	arr  []int
	cap  int
	head int
	tail int
}

func NewArrQueue(cap int) ArrQueue {
	return ArrQueue{
		arr:  make([]int, cap, cap), // 原计划用原生array实现, 但可用[3]int定义, 却不可用[cap]int定义, 故用slice实现
		cap:  cap,
		head: 0,
		tail: 0,
	}
}

// 固定cap的queue, 当head==tail时触发搬迁O(N)
func (q *ArrQueue) Push(v int) bool {
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

func (q *ArrQueue) Pop() bool {
	if q.head == q.tail {
		fmt.Printf("pop fail, head: %v, tail: %v\n", q.head, q.tail)
		return false
	}
	v := q.arr[q.head]
	q.head++
	fmt.Printf("pop %v ok, head: %v, tail: %v\n", v,q.head, q.tail)
	return true
}

func main() {
	fmt.Println("----[ArrQueue]")
	aq := NewArrQueue(3)
	fmt.Println("----Push")
	aq.Push(0)
	aq.Push(1)
	aq.Push(2)
	aq.Push(3)
	fmt.Println("----Pop")
	aq.Pop()
	aq.Pop()
	aq.Pop()
	aq.Pop()
	fmt.Println("----Push")
	aq.Push(0)
	aq.Push(1)
	aq.Push(2)
	aq.Push(3)
	fmt.Println("----done")
}
```

结果如下
```bash
----[ArrQueue]
----Push
push 0 ok, head: 0, tail: 1
push 1 ok, head: 0, tail: 2
push 2 ok, head: 0, tail: 3
push fail because full, head: 0, tail: 3
----Pop
pop 0 ok, head: 1, tail: 3
pop 1 ok, head: 2, tail: 3
pop 2 ok, head: 3, tail: 3
pop fail, head: 3, tail: 3
----Push
rebalance to head: 0, tail: 0
push 0 ok, head: 0, tail: 1
push 1 ok, head: 0, tail: 2
push 2 ok, head: 0, tail: 3
push fail because full, head: 0, tail: 3
----done
```

## 用linked-list实现
![image.png](https://note.youdao.com/yws/res/121787/WEBRESOURCE1dbd26ad28516a48016ead6a97600af9)
```go
package main

import (
	"fmt"
)

type ListQueue struct {
	head *Node
	tail *Node
}

type Node struct {
	Elem int
	Next *Node
}

func NewListQueue() ListQueue {
	sentinel := &Node{
		Elem: -1,
		Next: nil,
	}
	l := ListQueue{
		head: sentinel,
		tail: sentinel,
	}
	fmt.Printf("NewListQueue, %v\n", l.String())
	return l
}

func (lq *ListQueue) Push(v int) {
	lq.tail.Next = &Node{
		Elem: v,
		Next: nil,
	}
	lq.tail = lq.tail.Next
	fmt.Printf("push %v ok, %v\n", v, lq)
}

func (lq *ListQueue) Pop() bool {
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

func (lq *ListQueue) String() string {
	s := ""
	iter := lq.head
	for iter != nil {
		s += fmt.Sprintf("%v->", iter.Elem)
		iter = iter.Next
	}
	return s
}

func main() {
	fmt.Println("----[ListQueue]")
	lq := NewListQueue()
	fmt.Println("----Push")
	lq.Push(0)
	lq.Push(1)
	lq.Push(2)
	lq.Push(3)
	fmt.Println("----Pop")
	lq.Pop()
	lq.Pop()
	lq.Pop()
	lq.Pop()
	fmt.Println("----Push")
	lq.Push(0)
	lq.Push(1)
	lq.Push(2)
	lq.Push(3)
	fmt.Println("----done")
}
```

结果如下
```bash
----[ListQueue]
NewListQueue, -1->
----Push
push 0 ok, -1->0->
push 1 ok, -1->0->1->
push 2 ok, -1->0->1->2->
push 3 ok, -1->0->1->2->3->
----Pop
pop 0 ok, -1->1->2->3->
pop 1 ok, -1->2->3->
pop 2 ok, -1->3->
pop 3 ok, -1->
----Push
push 0 ok, -1->0->
push 1 ok, -1->0->1->
push 2 ok, -1->0->1->2->
push 3 ok, -1->0->1->2->3->
----done
```

# 环形队列
## 用array实现
CycleQueue 当无环queue满时需搬迁数据O(N), 为避免可用环形queue
当head==tail时, queue空
当(head+1)%cap==tail时, queue满, 此时tail未存数据(即环形queue会浪费一个存储空间, 只能存cap-1个)
```go
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

func main() {
	fmt.Println("----[CycleArrQueue]")
	caq := NewCycleArrQueue(3)
	fmt.Println("----Push")
	caq.Push(0)
	caq.Push(1)
	caq.Push(2)
	caq.Push(3)
	fmt.Println("----Pop")
	caq.Pop()
	caq.Pop()
	caq.Pop()
	caq.Pop()
	fmt.Println("----Push")
	caq.Push(0)
	caq.Push(1)
	caq.Push(2)
	caq.Push(3)
	fmt.Println("----done")
}
```

结果如下
```bash
----[CycleArrQueue]
NewCycleArrQueue, head: 0, tail: 0
----Push
push 0 ok, head: 0, tail: 1
push 1 ok, head: 0, tail: 2
push fail, head: 0, tail: 2
push fail, head: 0, tail: 2
----Pop
pop 0 ok, head: 1, tail: 2
pop 1 ok, head: 2, tail: 2
pop fail, head: 2, tail: 2
pop fail, head: 2, tail: 2
----Push
push 0 ok, head: 2, tail: 0
push 1 ok, head: 2, tail: 1
push fail, head: 2, tail: 1
push fail, head: 2, tail: 1
----done
```

## 用list实现
环形链表queue, tail后接head形成环, 每次在tail和head之间Push()和Pop(), 很高效
刚开始head与tail即成环
Push() head=>新node=>...=>tai=>head
Pop() head=>被移除的node=>...=>tai=>head
```go
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

// Push head=>新node=>...=>tai=>head
func (clq *CycleListQueue) Push(v int) {
	node := &Node{
		Elem: v,
		Next: clq.head.Next,
	}
	clq.head.Next = node
	fmt.Printf("push %v ok, %s\n", v, clq.String())
}

// Pop head=>被移除的node=>...=>tai=>head
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

func main() {
	fmt.Println("----[CycleListQueue]")
	clq := NewCycleListQueue()
	fmt.Println("----Push")
	clq.Push(0)
	clq.Push(1)
	clq.Push(2)
	clq.Push(3)
	fmt.Println("----Pop")
	clq.Pop()
	clq.Pop()
	clq.Pop()
	clq.Pop()
	fmt.Println("----Push")
	clq.Push(0)
	clq.Push(1)
	clq.Push(2)
	clq.Push(3)
	fmt.Println("----done")
}
```

结果如下
```bash
----[CycleListQueue]
NewCycleQueue, -1->-2
----Push
push 0 ok, -1->0->-2
push 1 ok, -1->1->0->-2
push 2 ok, -1->2->1->0->-2
push 3 ok, -1->3->2->1->0->-2
----Pop
pop 3 ok, -1->2->1->0->-2
pop 2 ok, -1->1->0->-2
pop 1 ok, -1->0->-2
pop 0 ok, -1->-2
----Push
push 0 ok, -1->0->-2
push 1 ok, -1->1->0->-2
push 2 ok, -1->2->1->0->-2
push 3 ok, -1->3->2->1->0->-2
----done
```