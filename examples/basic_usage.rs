use rust_stack::Stack;

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
    let mut string_stack = Stack::<String, 100>::new();
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

    // Тест с пользовательской структурой
    #[derive(Debug, Clone)]
    struct Person {
        name: String,
        age: u32,
    }

    let mut person_stack = Stack::<Person, 50>::new();
    let mut p1 = Person {
        name: String::from("Alice"),
        age: 25,
    };
    let mut p2 = Person {
        name: String::from("Bob"),
        age: 30,
    };

    println!("\n=== Person Stack ===");
    println!("Pushing {:?}", p1);
    person_stack.push(&mut p1);
    println!("Pushing {:?}", p2);
    person_stack.push(&mut p2);
    println!("Top value: {:?}", person_stack.get(1));
    println!("Popped: {:?}", person_stack.pop().map(|v| v.clone()));
    println!("Popped: {:?}", person_stack.pop().map(|v| v.clone()));
} 