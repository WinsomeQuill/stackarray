use rust_stack::Stack;

fn main() {
    println!("=== Rust Stack Demo ===");
    
    // Тест с маленьким стеком (размер 2)
    let mut small_stack = Stack::<i32, 2>::new();
    let a = 10;
    let b = 20;
    let c = 30;
    
    println!("=== Small Stack (size 2) ===");
    println!("Pushing {}", a);
    small_stack.push(&a);
    println!("Pushing {}", b);
    small_stack.push(&b);
    println!("Pushing {} (should fail)", c);
    let result = small_stack.push(&c); // Будет stack overflow
    println!("Push result: {}", result);
    println!("Is full: {}", small_stack.is_full());
    
    // Тест с большим стеком
    let big_stack = Stack::<i32, 10000>::new();
    println!("\n=== Big Stack (size 10000) ===");
    println!("Is empty: {}", big_stack.is_empty());
    println!("Is full: {}", big_stack.is_full());
    
    // Тест с String
    let mut string_stack = Stack::<String, 100>::new();
    let s1 = String::from("Hello");
    let s2 = String::from("World");
    println!("\n=== String Stack ===");
    println!("Pushing {}", s1);
    string_stack.push(&s1);
    println!("Pushing {}", s2);
    string_stack.push(&s2);
    println!("Top value: {:?}", string_stack.get(1));
    println!("Popped: {:?}", string_stack.pop());
    println!("Popped: {:?}", string_stack.pop());
}