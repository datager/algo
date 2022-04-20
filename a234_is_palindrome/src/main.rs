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
}

fn main() {}
