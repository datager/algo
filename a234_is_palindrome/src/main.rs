use std::fmt::Display;

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
}

fn main() {
    println!("自己写一遍")
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
}
