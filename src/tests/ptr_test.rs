use std::ptr::NonNull;

#[derive(Debug)]
struct LinkedList<T> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    len: i32,
}

#[derive(Debug)]
struct Node<T> {
    next: Option<NonNull<Node<T>>>,
    prev: Option<NonNull<Node<T>>>,
    element: T,
}

impl<T> Node<T> {
    pub fn new(v: T) -> Self {
        Node { element: v, next: None, prev: None }
    }

    pub fn new_box(v: T) -> Box<Self> {
        Box::new(Self::new(v))
    }

    /// The element of Node<T> will be moved
    pub fn into(self: Box<Self>) -> T {
        self.element
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList { head: None, tail: None, len: 0 }
    }

    pub fn new_box() -> Box<Self> {
        Box::new(Self::new())
    }

    // Add node to front
    fn push_front(&mut self, mut node: Box<Node<T>>) {
        node.next = self.head;
        node.prev = None;

        let node = Some(Box::leak(node).into());
        match self.head {
            None => self.tail = node,
            Some(head) => unsafe { (*head.as_ptr()).prev = node }
        }

        self.head = node;
        self.len += 1;
    }

    fn push_back(&mut self, mut node: Box<Node<T>>) {
        node.next = None;

        let node = Some(Box::leak(node).into());
        match self.tail {
            None => { self.head = node; }
            Some(tail) => unsafe { (*tail.as_ptr()).next = node; }
        }

        self.tail = node;
        self.len += 1;
    }
}

#[test]
fn test_ptr_base() {
    let node_on_stack = Node::<i32> { element: 100, next: None, prev: None };
    let i32_on_stack = 100;
    let i32_on_stack2 = 100;
    let node_on_heap = Node::new_box(100);
    println!("ptr_v: {:?}, ptr_vv: {:?}", &i32_on_stack as *const i32, &i32_on_stack2 as *const i32);
    println!("ptr_node: {:?}", &node_on_stack as *const Node<i32>);
    println!("pn.element: {:?} pn.next: {:?}", &node_on_stack.element as *const i32, &node_on_stack.next as *const Option<NonNull<Node<i32>>>);
    println!("ptr_hn: {:?} hn: {:?} heap: {:?}", &node_on_heap as *const Box<Node<i32>>, node_on_heap, node_on_heap.as_ref() as *const Node<i32>);
    println!("elem: {:?}", node_on_heap.into());
}

#[test]
fn test_ptr_base2() {
    let mut l = LinkedList::<i32>::new();
    println!("LinkedList: {:?}", l);
    l.push_front(Node::new_box(100));
    l.push_front(Node::new_box(99));
    println!("LinkedList: {:?}", l);
}

