use std::fmt::Display;

// 链表的实现
pub struct List {
    head: Link,
}
type Link = Option<Box<Node>>;

struct Node {
    elem: i32,
    next: Link,
}

impl Node {
    fn new(elem: i32) -> Self {
        Self { elem, next: None }
    }
}

impl Display for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "链表: ")?;
        if self.head.is_none() {
            write!(f, "None")?;
            return Ok(());
        }
        let mut next = self.head.as_ref();
        while let Some(node) = next {
            // Some(), 即当到达链表尾时, 终止while let不成立, 终止迭代
            write!(f, "{} -> ", node.elem)?;
            next = node.next.as_ref();
        }
        write!(f, "Node")?;
        Ok(())
    }
}

impl List {
    fn new() -> Self {
        Self { head: None }
    }
    // 向链表尾部插入
    fn push(&mut self, elem: i32) -> &mut Self {
        // 若链表为空
        if self.head.is_none() {
            self.head = Some(Box::new(Node::new(elem)));
            return self;
        }
        // 找到链表尾, 赋给pnode
        let mut pnode = self.head.as_mut(); // 头
        while let Some(cur_node) = pnode {
            if cur_node.next.is_none() {
                pnode = Some(cur_node);
                break; // 已找到
            }
            pnode = cur_node.next.as_mut(); // 未找到, pnode后移, 继续向后遍历
        }
        // 执行插入
        pnode.map(|node| node.next = Some(Box::new(Node::new(elem))));

        self
    }

    // 链表反转
    fn reverse(&mut self) {
        let mut rnode: Link = self.head.take();
        let mut lnode: Link = None;
        while let Some(mut node) = rnode {
            rnode = node.next; // rnode不断后移
            node.next = lnode;
            lnode = Some(node);
        }
        self.head = lnode;
    }

    // 找到链表中间节点
    fn middle_node(&self) -> Option<&Box<Node>> {
        if self.head.is_none() {
            return None;
        }

        let mut fast = self.head.as_ref();
        let mut slow = self.head.as_ref();

        while fast.is_some() && fast.unwrap().next.is_some() {
            slow = slow.unwrap().next.as_ref();
            fast = fast.unwrap().next.as_ref().unwrap().next.as_ref();
        }
        slow
    }

    // 判断链表是否回文
    fn is_palidrome_bystack(&self) -> bool {
        if self.head.is_none() {
            return true;
        }
        let mut cur = self.head.as_ref();

        // 都放入栈(用数组实现)
        let mut v = vec![];
        while let Some(node) = cur {
            v.push(node.elem);
            cur = node.next.as_ref();
        }

        // 出栈
        cur = self.head.as_ref();
        for elem in v.into_iter().rev() {
            let node = cur.unwrap();
            if node.elem != elem {
                return false;
            }
            cur = node.next.as_ref();
        }

        true
    }
}

mod tests {
    use super::List;
    #[test]
    fn test_is_palidrome_bystack() {
        let mut list = List::new();
        list.push(1).push(2).push(3).push(2).push(1);
        let ret = list.is_palidrome_bystack();
        println!("{}, {}", list, ret);
        assert!(ret == true, "12321是回文字符串啊");
    }
}

fn main() {
    println!("rust写链表好难")
}
