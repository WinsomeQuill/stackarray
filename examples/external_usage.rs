// Пример использования rust-stack как внешней зависимости
// Этот файл демонстрирует, как будет выглядеть код в другом проекте

use rust_stack::Stack;

fn main() {
    println!("=== External Usage Demo ===");
    
    // Тест с разными типами данных
    test_int_stack();
    test_string_stack();
    test_custom_struct();
    test_stack_limits();
}

fn test_int_stack() {
    println!("\n--- Integer Stack Test ---");
    let mut stack = Stack::<i32, 5>::new();
    
    for i in 1..=5 {
        println!("Pushing {}", i);
        stack.push(&i);
    }
    
    println!("Stack is full: {}", stack.is_full());
    
    while let Some(value) = stack.pop() {
        println!("Popped: {}", value);
    }
}

fn test_string_stack() {
    println!("\n--- String Stack Test ---");
    let mut stack = Stack::<String, 3>::new();
    
    let words = vec!["Hello", "World", "Rust"];
    
    for word in words {
        let s = String::from(word);
        println!("Pushing '{}'", s);
        stack.push(&s);
    }
    
    while let Some(value) = stack.pop() {
        println!("Popped: '{}'", value);
    }
}

fn test_custom_struct() {
    println!("\n--- Custom Struct Test ---");
    
    #[derive(Debug)]
    struct Point {
        x: f64,
        y: f64,
    }
    
    let mut stack = Stack::<Point, 4>::new();
    
    let points = vec![
        Point { x: 1.0, y: 2.0 },
        Point { x: 3.0, y: 4.0 },
        Point { x: 5.0, y: 6.0 },
    ];
    
    for point in points {
        println!("Pushing {:?}", point);
        stack.push(&point);
    }
    
    while let Some(point) = stack.pop() {
        println!("Popped: {:?}", point);
    }
}

fn test_stack_limits() {
    println!("\n--- Stack Limits Test ---");
    let mut stack = Stack::<u8, 2>::new();
    
    println!("Initial state:");
    println!("  Empty: {}", stack.is_empty());
    println!("  Full: {}", stack.is_full());
    
    // Добавляем элементы
    let a = 10u8;
    let b = 20u8;
    let c = 30u8;
    
    println!("Pushing {}", a);
    assert!(stack.push(&a));
    
    println!("Pushing {}", b);
    assert!(stack.push(&b));
    
    println!("Pushing {} (should fail)", c);
    assert!(!stack.push(&c)); // Должно вернуть false
    
    println!("Final state:");
    println!("  Empty: {}", stack.is_empty());
    println!("  Full: {}", stack.is_full());
} 