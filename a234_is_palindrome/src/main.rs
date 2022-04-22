use std::fmt::Display;

//用rust实现链表好麻烦, 因为有Option和Box, 所以经常要unwrap和as_ref, as_mut, 概念有点多, 大伙都说rust实现链表是高级难度, 容易劝退, 好歹手写下来了, 先告一段落, 后面继续巩固
// 先定义节点, Option是为了判空, Box是为了放在heap上(由list特性决定的)
type Link = Option<Box<Node>>;
struct Node {
    elem: i32,
    next: Link,
}
impl Node {
    fn new(elem: i32) -> Node {
        Node { elem, next: None }
    }
}
struct List {
    head: Link,
}

// 方便打印
impl Display for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "链表: ")?;
        if self.head.is_none() {
            write!(f, "None")?;
            return Ok(());
        }
        let mut iter = self.head.as_ref();
        while let Some(node) = iter {
            write!(f, "{} -> ", node.elem)?;
            iter = node.next.as_ref();
        }
        write!(f, "None")?; // 尾
        Ok(())
    }
}

impl List {
    fn new() -> Self {
        Self { head: None }
    }
    fn push_tail(&mut self, elem: i32) -> &mut Self {
        if self.head.is_none() {
            self.head = Some(Box::new(Node::new(elem)));
            return self;
        }
        // 找到list尾
        let mut iter = self.head.as_mut();
        while let Some(node) = iter {
            if (node.next.is_none()) {
                // 已找到
                iter = Some(node);
                break;
            }
            iter = node.next.as_mut(); // 未找到, 继续向后遍历
        }
        iter.map(|node| node.next = Some(Box::new(Node::new(elem)))); // 执行插入
        self
    }
    // 找中点
    fn find_mid(&mut self) -> Option<&Box<Node>> {
        if self.head.is_none() {
            return None;
        }
        let mut fast = self.head.as_ref();
        let mut slow = self.head.as_ref();

        while fast.is_some() && fast.unwrap().next.is_some() {
            fast = fast.unwrap().next.as_ref().unwrap().next.as_ref();
            slow = slow.unwrap().next.as_ref();
        }
        slow
    }
    // 是否回文-stack方式
    fn is_palindrome_by_stack(&self) -> bool {
        if self.head.is_none() {
            return true;
        }
        // fill in stack
        let mut arr = vec![];
        let mut iter = self.head.as_ref();
        while let Some(node) = iter {
            arr.push(node.elem);
            iter = node.next.as_ref();
        }
        // get from stack
        iter = self.head.as_ref();
        for elem in arr.into_iter().rev() {
            let node = iter.unwrap();
            if node.elem != elem {
                return false;
            }
            iter = node.next.as_ref();
        }
        true
    }
    // 链表反转
    // 保存 prev, cur, next_tmp三个变量, 向后迭代
    // 每次将cur.next赋值给prev来实现反转
    // 然后prev后移, cur后移, 方便下次迭代
    // 最终返回prev作为list的head
    pub fn reverse(&mut self) {
        let mut prev = None;
        let mut cur = self.head.take();

        while let Some(mut cur_inner) = cur.take() {
            let next_tmp = cur_inner.next.take(); // 临时变量存储
            cur_inner.next = prev.take(); // 执行反转
            prev = Some(cur_inner); // prev后移, 用于下次迭代
            cur = next_tmp; // cur后移, 用于下次迭代
        }
        self.head = prev.take();
    }
    // 链表中环的检测
    // pub fn is_cycle(&mut self) -> bool {
    //     let mut cur = self.head.as_ref();
    //     while let Some(node) = cur {
    //         if node == cur {
    //             return true;
    //         }
    //     }
    //     false
    // }

    // 两个有序的链表合并

    // 删除链表倒数第 n 个结点
}
fn main() {
    print!("1");
}
mod tests {
    use super::List;
    #[test]
    fn test_display_list() {
        let mut l = List::new();
        l.push_tail(1).push_tail(2).push_tail(3);
        println!("{}", l);
    }

    #[test]
    fn test_find_mid() {
        let mut l = List::new();
        l.push_tail(1).push_tail(2).push_tail(3);
        let m = l.find_mid();
        assert!(m.unwrap().as_ref().elem == 2, "mid should be 2");
    }

    #[test]

    fn test_is_palindrome_by_stack_should_ok() {
        let mut l = List::new();
        l.push_tail(1)
            .push_tail(2)
            .push_tail(3)
            .push_tail(2)
            .push_tail(1);
        let b = l.is_palindrome_by_stack();
        assert!(b == true, "是回文");
    }

    #[test]
    fn test_is_palindrome_by_stack_should_fail() {
        let mut l = List::new();
        l.push_tail(1)
            .push_tail(2)
            .push_tail(3)
            .push_tail(4)
            .push_tail(5);
        let b = l.is_palindrome_by_stack();
        assert!(b == false, "不是回文");
    }

    #[test]
    fn test_reverse_should_ok() {
        let mut l = List::new();
        l.push_tail(1)
            .push_tail(2)
            .push_tail(3)
            .push_tail(4)
            .push_tail(5);
        println!("原始为: {}", l);
        l.reverse();
        println!("反转后: {}", l);
    }
}
