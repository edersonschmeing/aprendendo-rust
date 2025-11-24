use std::cell::RefCell;
use std::rc::{Rc, Weak};

type Link<T> = Option<Rc<RefCell<Node<T>>>>;
type WeakLink<T> = Option<Weak<RefCell<Node<T>>>>;

#[derive(Debug)]
struct Node<T> {
    value: T,
    next: Link<T>,
    prev: WeakLink<T>,
}

impl<T> Node<T> {
    fn new(value: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            value,
            next: None,
            prev: None,
        }))
    }
}

#[derive(Debug)]
pub struct DoublyLinkedList<T> {
    head: Link<T>,
    tail: Link<T>,
    len: usize,
}

impl<T: Ord + Clone + std::fmt::Debug> DoublyLinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            len: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    // ----------------------------------
    // PUSH / POP
    // ----------------------------------

    pub fn push_back(&mut self, value: T) {
        let new_node = Node::new(value);

        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_node.clone());
                new_node.borrow_mut().prev = Some(Rc::downgrade(&old_tail));
                self.tail = Some(new_node);
            }
            None => {
                self.head = Some(new_node.clone());
                self.tail = Some(new_node);
            }
        }

        self.len += 1;
    }

    pub fn push_front(&mut self, value: T) {
        let new_node = Node::new(value);

        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(Rc::downgrade(&new_node));
                new_node.borrow_mut().next = Some(old_head);
                self.head = Some(new_node);
            }
            None => {
                self.tail = Some(new_node.clone());
                self.head = Some(new_node);
            }
        }

        self.len += 1;
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|old_tail| {
            if let Some(prev) = old_tail.borrow_mut().prev.take() {
                let prev = prev.upgrade().unwrap();
                prev.borrow_mut().next = None;
                self.tail = Some(prev);
            } else {
                self.head = None;
            }

            self.len -= 1;

            Rc::try_unwrap(old_tail).ok().unwrap().into_inner().value
        })
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            if let Some(next) = old_head.borrow_mut().next.take() {
                next.borrow_mut().prev = None;
                self.head = Some(next);
            } else {
                self.tail = None;
            }

            self.len -= 1;

            Rc::try_unwrap(old_head).ok().unwrap().into_inner().value
        })
    }

    // ----------------------------------
    // INSERT / REMOVE
    // ----------------------------------

    pub fn insert_at(&mut self, index: usize, value: T) {
        if index > self.len {
            panic!("índice fora do intervalo");
        }

        if index == 0 {
            self.push_front(value);
            return;
        }

        if index == self.len {
            self.push_back(value);
            return;
        }

        let mut current = self.head.clone().unwrap();

        // percorre até o nó na posição `index`
        for _ in 0..index {
            let next = {
                let cur_borrow = current.borrow();
                cur_borrow.next.clone().unwrap()
            };
            current = next;
        }

        let prev = current.borrow().prev.clone().unwrap().upgrade().unwrap();
        let new_node = Node::new(value);

        new_node.borrow_mut().prev = Some(Rc::downgrade(&prev));
        new_node.borrow_mut().next = Some(current.clone());

        prev.borrow_mut().next = Some(new_node.clone());
        current.borrow_mut().prev = Some(Rc::downgrade(&new_node));

        self.len += 1;
    }

    pub fn remove_at(&mut self, index: usize) -> T {
        if index >= self.len {
            panic!("índice fora do intervalo");
        }

        if index == 0 {
            return self.pop_front().unwrap();
        }

        if index == self.len - 1 {
            return self.pop_back().unwrap();
        }

        let mut current = self.head.clone().unwrap();

        for _ in 0..index {
            let next = {
                let cur_borrow = current.borrow();
                cur_borrow.next.clone().unwrap()
            };
            current = next;
        }

        let prev = current.borrow().prev.clone().unwrap().upgrade().unwrap();
        let next = current.borrow().next.clone().unwrap();

        prev.borrow_mut().next = Some(next.clone());
        next.borrow_mut().prev = Some(Rc::downgrade(&prev));

        self.len -= 1;

        Rc::try_unwrap(current).ok().unwrap().into_inner().value
    }

    pub fn clear(&mut self) {
        while self.pop_front().is_some() {}
    }

    // ----------------------------------
    // PRINT
    // ----------------------------------

    pub fn print_forward(&self) {
        let mut cur = self.head.clone();
        print!("[ ");
        while let Some(node) = cur {
            print!("{:?} ", node.borrow().value);
            cur = node.borrow().next.clone();
        }
        println!("]");
    }

    pub fn print_backward(&self) {
        let mut cur = self.tail.clone();
        print!("[ ");
        while let Some(node) = cur {
            print!("{:?} ", node.borrow().value);
            cur = node.borrow().prev.clone().and_then(|w| w.upgrade());
        }
        println!("]");
    }

    // ----------------------------------
    // SORT – insertion sort estável
    // ----------------------------------

    pub fn sort(&mut self) {
        if self.len <= 1 {
            return;
        }

        // começamos no segundo nó
        let mut i_opt = self.head.as_ref().unwrap().borrow().next.clone();

        while let Some(i) = i_opt.clone() {
            let key = i.borrow().value.clone();

            // ponteiro que anda para trás
            let mut j_opt = i.borrow().prev.clone().and_then(|w| w.upgrade());

            // desloca valores maiores para a direita
            while let Some(j) = j_opt.clone() {
                // se j.value <= chave, paramos
                if j.borrow().value <= key {
                    break;
                }

                // copiar j.value para j.next.value
                let value_j = j.borrow().value.clone();
                if let Some(next) = j.borrow().next.clone() {
                    next.borrow_mut().value = value_j;
                }

                j_opt = j.borrow().prev.clone().and_then(|w| w.upgrade());
            }

            // agora j_opt é o nó ANTES do ponto onde key deve entrar

            match j_opt {
                Some(j) => {
                    let next = j.borrow().next.clone().unwrap();
                    next.borrow_mut().value = key;
                }
                None => {
                    // inserir no head
                    self.head.as_ref().unwrap().borrow_mut().value = key;
                }
            }

            i_opt = i.borrow().next.clone();
        }
    }

}

#[cfg(not(test))]
fn main() {
    let mut list = DoublyLinkedList::new();

    list.push_back(3);
    list.push_back(1);
    list.push_back(4);
    list.push_back(2);

    println!("Lista original:");
    list.print_forward();

    list.sort();
    println!("Lista ordenada:");
    list.print_forward();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort() {
        let mut list = DoublyLinkedList::new();
        list.push_back(3);
        list.push_back(1);
        list.push_back(2);
        list.push_back(5);
        list.push_back(4);

        list.sort();

        let mut result = vec![];
        while let Some(v) = list.pop_front() {
            result.push(v);
        }

        assert_eq!(result, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_insert_remove_basic() {
        let mut l = DoublyLinkedList::new();
        l.push_back(10);
        l.push_back(20);
        l.push_back(30);
        l.insert_at(1, 15); // 10,15,20,30
        assert_eq!(l.remove_at(1), 15);
        assert_eq!(l.remove_at(0), 10);
        assert_eq!(l.remove_at(0), 20);
        assert_eq!(l.remove_at(0), 30);
        assert!(l.is_empty());
    }
}
