use std::alloc::{alloc, dealloc, realloc, Layout};
use std::cmp::Ordering;
use std::fmt::Debug;
use std::mem;
use std::ops::{Deref, DerefMut};
use std::ptr;

pub struct Vector<T> {
    ptr: *mut T,      // Ponteiro para o buffer de dados
    cap: usize,       // Capacidade alocada
    len: usize,       // Quantidade de elementos armazenados
}

impl<T> Vector<T> {
    /// Cria um novo vetor com capacidade inicial `capacity`
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0);
        let layout = Layout::array::<T>(capacity).unwrap();
        let ptr = unsafe { alloc(layout) as *mut T };
        Self { ptr, cap: capacity, len: 0 }
    }

    /// Adiciona um valor ao final do vetor
    pub fn push(&mut self, value: T) {
        if self.len == self.cap {
            self.resize(self.cap * 2);
        }
        unsafe {
            ptr::write(self.ptr.add(self.len), value);
        }
        self.len += 1;
    }

    /// Remove o último elemento do vetor
    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }
        self.len -= 1;
        let result = unsafe { ptr::read(self.ptr.add(self.len)) };
        if self.len > 0 && self.len <= self.cap / 2 {
            self.resize(self.cap / 2);
        }
        Some(result)
    }

    /// Redimensiona o vetor para uma nova capacidade
    fn resize(&mut self, new_cap: usize) {
        let new_layout = Layout::array::<T>(new_cap).unwrap();
        let old_layout = Layout::array::<T>(self.cap).unwrap();
        unsafe {
            let new_ptr = realloc(self.ptr as *mut u8, old_layout, new_layout.size()) as *mut T;
            if new_ptr.is_null() {
                panic!("Falha ao realocar memória");
            }
            self.ptr = new_ptr;
        }
        self.cap = new_cap;
    }

    /// Retorna a quantidade de elementos armazenados
    pub fn len(&self) -> usize {
        self.len
    }

    /// Retorna a capacidade do vetor
    pub fn capacity(&self) -> usize {
        self.cap
    }

    /// Acesso direto (unsafe)
    pub unsafe fn get_unchecked(&self, index: usize) -> &T {
        &*self.ptr.add(index)
    }

    /// Acesso direto mutável (unsafe)
    pub unsafe fn get_unchecked_mut(&mut self, index: usize) -> &mut T {
        &mut *self.ptr.add(index)
    }

    /// Depuração: imprime todos os elementos (requer Debug)
    pub fn print_debug(&self)
    where
        T: Debug,
    {
        print!("[");
        for i in 0..self.len {
            unsafe {
                print!("{:?}{}", *self.ptr.add(i), if i + 1 < self.len { ", " } else { "" });
            }
        }
        println!("]");
    }
}

impl<T: PartialEq> Vector<T> {
    /// Busca linear por um valor
    pub fn find(&self, value: &T) -> Option<usize> {
        for i in 0..self.len {
            unsafe {
                if *self.ptr.add(i) == *value {
                    return Some(i);
                }
            }
        }
        None
    }
}

impl<T: Ord> Vector<T> {
    /// Ordena os elementos usando insertion sort
    pub fn sort(&mut self) {
        for i in 1..self.len {
            let mut j = i;
            unsafe {
                while j > 0 && *self.ptr.add(j - 1) > *self.ptr.add(j) {
                    let tmp = ptr::read(self.ptr.add(j));
                    ptr::write(self.ptr.add(j), ptr::read(self.ptr.add(j - 1)));
                    ptr::write(self.ptr.add(j - 1), tmp);
                    j -= 1;
                }
            }
        }
    }

    /// Busca binária (requer vetor ordenado)
    pub fn binary_search(&self, value: &T) -> Option<usize> {
        let mut left = 0;
        let mut right = self.len;
        while left < right {
            let mid = (left + right) / 2;
            unsafe {
                match (*self.ptr.add(mid)).cmp(value) {
                    Ordering::Equal => return Some(mid),
                    Ordering::Less => left = mid + 1,
                    Ordering::Greater => right = mid,
                }
            }
        }
        None
    }

    /// Ordena com chave derivada (sort_by_key)
    pub fn sort_by_key<K: Ord, F: FnMut(&T) -> K>(&mut self, mut f: F) {
        for i in 1..self.len {
            let mut j = i;
            unsafe {
                while j > 0 && f(&*self.ptr.add(j - 1)) > f(&*self.ptr.add(j)) {
                    let tmp = ptr::read(self.ptr.add(j));
                    ptr::write(self.ptr.add(j), ptr::read(self.ptr.add(j - 1)));
                    ptr::write(self.ptr.add(j - 1), tmp);
                    j -= 1;
                }
            }
        }
    }
}

impl<T> Drop for Vector<T> {
    fn drop(&mut self) {
        unsafe {
            for i in 0..self.len {
                ptr::drop_in_place(self.ptr.add(i));
            }
            let layout = Layout::array::<T>(self.cap).unwrap();
            dealloc(self.ptr as *mut u8, layout);
        }
    }
}
   
pub fn demo_vector_primitive_pointers() {
    
    println!("\n Demo Vector");
    let mut vec = Vector::new(4); // Cria vetor com capacidade inicial de 4
    
    vec.push(10); // Insere valores
    vec.push(20);
    vec.push(30);
    vec.push(40);
    vec.print_debug(); // Mostra conteúdo, tamanho e capacidade
    vec.push(50); // Adiciona valor, forçando redimensionamento para o dobro
    vec.print_debug();
    vec.pop(); // Remove último elemento (50)
    vec.pop(); // Remove (40)
    vec.print_debug(); // Deve reduzir capacidade automaticamente
    //if let Some(v) = vec.get(1) {
    //    println!("Elemento no índice 1: {}", v); // Acessa elemento por índice
    //}
}