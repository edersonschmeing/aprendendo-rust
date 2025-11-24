use std::alloc::{alloc, dealloc, Layout};
use std::ptr;

#[repr(C)]
struct Node<T> {
    value: T,
    next: *mut Node<T>,
    prev: *mut Node<T>,
}

pub struct DoublyLinkedList<T> {
    head: *mut Node<T>,
    tail: *mut Node<T>,
    len: usize,
}

impl<T> DoublyLinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: ptr::null_mut(),
            tail: ptr::null_mut(),
            len: 0,
        }
    }

    fn layout() -> Layout {
        Layout::new::<Node<T>>()
    }

    unsafe fn allocate_node(value: T) -> *mut Node<T> {
        let layout = Self::layout();
        let raw = alloc(layout) as *mut Node<T>;

        if raw.is_null() {
            std::alloc::handle_alloc_error(layout);
        }

        ptr::write(
            raw,
            Node {
                value,
                next: ptr::null_mut(),
                prev: ptr::null_mut(),
            },
        );

        raw
    }

    unsafe fn free_node(ptr_node: *mut Node<T>) {
        let layout = Self::layout();
        ptr::drop_in_place(ptr_node);
        dealloc(ptr_node as *mut u8, layout);
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn push_front(&mut self, value: T) {
        unsafe {
            let node = Self::allocate_node(value);

            (*node).next = self.head;

            if !self.head.is_null() {
                (*self.head).prev = node;
            } else {
                self.tail = node;
            }

            self.head = node;
            self.len += 1;
        }
    }

    pub fn push_back(&mut self, value: T) {
        unsafe {
            let node = Self::allocate_node(value);

            (*node).prev = self.tail;

            if !self.tail.is_null() {
                (*self.tail).next = node;
            } else {
                self.head = node;
            }

            self.tail = node;
            self.len += 1;
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if self.head.is_null() {
            return None;
        }
        unsafe {
            let old = self.head;

            if !(*old).next.is_null() {
                self.head = (*old).next;
                (*self.head).prev = ptr::null_mut();
            } else {
                self.head = ptr::null_mut();
                self.tail = ptr::null_mut();
            }

            self.len -= 1;

            let value = ptr::read(&(*old).value);
            Self::free_node(old);
            Some(value)
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        if self.tail.is_null() {
            return None;
        }
        unsafe {
            let old = self.tail;

            if !(*old).prev.is_null() {
                self.tail = (*old).prev;
                (*self.tail).next = ptr::null_mut();
            } else {
                self.head = ptr::null_mut();
                self.tail = ptr::null_mut();
            }

            self.len -= 1;

            let value = ptr::read(&(*old).value);
            Self::free_node(old);
            Some(value)
        }
    }

    pub fn clear(&mut self) {
        while self.pop_front().is_some() {}
    }

    pub fn print_forward(&self)
    where
        T: std::fmt::Debug,
    {
        unsafe {
            let mut cur = self.head;
            print!("[ ");
            while !cur.is_null() {
                print!("{:?} ", (*cur).value);
                cur = (*cur).next;
            }
            println!("]");
        }
    }

    pub fn print_backward(&self)
    where
        T: std::fmt::Debug,
    {
        unsafe {
            let mut cur = self.tail;
            print!("[ ");
            while !cur.is_null() {
                print!("{:?} ", (*cur).value);
                cur = (*cur).prev;
            }
            println!("]");
        }
    }
}

fn main() {
    let mut list = DoublyLinkedList::new();

    list.push_back(10);
    list.push_back(20);
    list.push_back(30);

    list.print_forward();

    println!("pop_front = {:?}", list.pop_front());
    list.print_forward();

    list.push_front(5);
    list.print_forward();

    println!("pop_back = {:?}", list.pop_back());
    list.print_forward();

    list.clear();
    println!("lista vazia? {}", list.is_empty());
}
