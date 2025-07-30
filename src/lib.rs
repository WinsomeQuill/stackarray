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
    /// Создает новый стек
    pub fn new() -> Self {
        let mut buffer = [ptr::null_mut::<c_void>(); N];
        let mut inner = c::Stack {
            sp: -1,
            buffer: buffer.as_mut_ptr(),
            max_size: N as i32,
        };

        unsafe {
            c::init_stack(&mut inner, buffer.as_mut_ptr() as *mut c_void, N as i32);
        }

        Stack {
            inner,
            buffer,
            _marker: PhantomData,
        }
    }

    /// Добавляет элемент в стек
    pub fn push(&mut self, value: &T) -> bool {
        let ptr = value as *const T as *mut c_void;
        unsafe { c::push(&mut self.inner, ptr) != 0 }
    }

    /// Извлекает элемент из стека
    pub fn pop(&mut self) -> Option<&'static T> {
        let ptr = unsafe { c::pop(&mut self.inner) };
        if ptr.is_null() {
            None
        } else {
            Some(unsafe { &*(ptr as *const T) })
        }
    }

    /// Получает элемент по индексу
    pub fn get(&self, index: usize) -> Option<&T> {
        let ptr = unsafe { c::get(&self.inner as *const _ as *mut _, index as i32) };
        if ptr.is_null() {
            None
        } else {
            Some(unsafe { &*(ptr as *const T) })
        }
    }

    /// Проверяет, пуст ли стек
    pub fn is_empty(&self) -> bool {
        unsafe { c::is_empty(&self.inner as *const _ as *mut _) != 0 }
    }

    /// Проверяет, полон ли стек
    pub fn is_full(&self) -> bool {
        unsafe { c::is_full(&self.inner as *const _ as *mut _) != 0 }
    }
}

impl<T, const N: usize> Default for Stack<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

