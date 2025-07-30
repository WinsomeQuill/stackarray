mod c {
    #![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

use libc::c_void;
use std::marker::PhantomData;
use std::ptr;

/// Безопасная обёртка над стеком из C
pub struct Stack<T, const N: usize> {
    inner: c::Stack,
    buffer: [*mut c_void; N],
    _marker: PhantomData<T>,
}

impl<T, const N: usize> Stack<T, N> {
    pub fn new() -> Stack<T, N> {
        let mut buffer = [ptr::null_mut::<c_void>(); N];
        let mut inner = c::Stack {
            sp: -1,
            buffer: buffer.as_mut_ptr(),
        };

        unsafe {
            c::init_stack(&mut inner, buffer.as_mut_ptr() as *mut c_void);
        }

        Stack {
            inner,
            buffer,
            _marker: PhantomData,
        }
    }

    pub fn push(&mut self, value: &mut T) -> bool {
        let ptr = value as *mut T as *mut c_void;
        unsafe { c::push(&mut self.inner, ptr) != 0 }
    }

    pub fn pop(&mut self) -> Option<&'static mut T> {
        let ptr = unsafe { c::pop(&mut self.inner) };
        if ptr.is_null() {
            None
        } else {
            Some(unsafe { &mut *(ptr as *mut T) })
        }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        let ptr = unsafe { c::get(&self.inner as *const _ as *mut _, index as i32) };
        if ptr.is_null() {
            None
        } else {
            Some(unsafe { &*(ptr as *const T) })
        }
    }

    pub fn is_empty(&self) -> bool {
        unsafe { c::is_empty(&self.inner as *const _ as *mut _) != 0 }
    }

    pub fn is_full(&self) -> bool {
        unsafe { c::is_full(&self.inner as *const _ as *mut _) != 0 }
    }
}

impl<T, const N: usize> Default for Stack<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

fn main() {
    // Тест с i32
    let mut int_stack = Stack::<i32, 1024>::new();
    let mut a = 10;
    let mut b = 20;
    println!("=== i32 Stack ===");
    println!("Pushing {}", a);
    int_stack.push(&mut a);
    println!("Pushing {}", b);
    int_stack.push(&mut b);
    println!("Top value: {:?}", int_stack.get(1));
    println!("Popped: {:?}", int_stack.pop().map(|v| *v));
    println!("Popped: {:?}", int_stack.pop().map(|v| *v));

    // Тест с String
    let mut string_stack = Stack::<String, 1>::new();
    let mut s1 = String::from("Hello");
    let mut s2 = String::from("World");
    println!("\n=== String Stack ===");
    println!("Pushing {}", s1);
    string_stack.push(&mut s1);
    println!("Pushing {}", s2);
    string_stack.push(&mut s2);
    println!("Top value: {:?}", string_stack.get(1));
    println!("Popped: {:?}", string_stack.pop().map(|v| v.clone()));
    println!("Popped: {:?}", string_stack.pop().map(|v| v.clone()));
}
