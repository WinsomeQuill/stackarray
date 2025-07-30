# Rust Stack - Safe Bindings for C Stack Implementation

Безопасные Rust привязки для C реализации стека. Эта библиотека предоставляет типобезопасную обёртку над C стеком, позволяющую работать с любыми типами данных.

## Особенности

- ✅ Типобезопасность Rust
- ✅ Поддержка любых типов данных
- ✅ Динамический размер стека (задается на этапе компиляции)
- ✅ Безопасные привязки к C коду
- ✅ Подробная документация

## Установка

### Через Git (рекомендуется)

Добавьте в ваш `Cargo.toml`:

```toml
[dependencies]
rust-stack = { git = "https://github.com/WinsomeQuill/stackarray" }
```

### Через crates.io (когда будет опубликовано)

```toml
[dependencies]
rust-stack = "0.1.0"
```

## Использование

### Базовое использование

```rust
use rust_stack::Stack;

fn main() {
    // Создаем стек для i32 с максимальным размером 1024
    let mut stack = Stack::<i32, 1024>::new();
    
    let mut a = 10;
    let mut b = 20;
    
    // Добавляем элементы
    stack.push(&mut a);
    stack.push(&mut b);
    
    // Получаем элемент по индексу
    println!("Top: {:?}", stack.get(1));
    
    // Извлекаем элементы
    println!("Popped: {:?}", stack.pop().map(|v| *v));
    println!("Popped: {:?}", stack.pop().map(|v| *v));
}
```

### Работа с разными типами и размерами

```rust
use rust_stack::Stack;

// Маленький стек для строк
let mut small_stack = Stack::<String, 5>::new();
let mut s1 = String::from("Hello");
small_stack.push(&mut s1);

// Большой стек для пользовательских структур
#[derive(Debug, Clone)]
struct Person {
    name: String,
    age: u32,
}

let mut big_stack = Stack::<Person, 1000>::new();
let mut person = Person {
    name: String::from("Alice"),
    age: 25,
};
big_stack.push(&mut person);
```

### Проверка состояния стека

```rust
let mut stack = Stack::<i32, 10>::new();

println!("Empty: {}", stack.is_empty());  // true
println!("Full: {}", stack.is_full());    // false

// Заполняем стек
for i in 0..10 {
    let mut value = i;
    stack.push(&mut value);
}

println!("Empty: {}", stack.is_empty());  // false
println!("Full: {}", stack.is_full());    // true
```

## API

### `Stack<T, N>`

Основная структура стека.

- `T` - тип элементов стека
- `N` - максимальный размер стека (константа времени компиляции)

### Методы

- `new() -> Self` - создает новый пустой стек
- `push(&mut self, value: &mut T) -> bool` - добавляет элемент в стек
- `pop(&mut self) -> Option<&'static mut T>` - извлекает элемент из стека
- `get(&self, index: usize) -> Option<&T>` - получает элемент по индексу
- `is_empty(&self) -> bool` - проверяет, пуст ли стек
- `is_full(&self) -> bool` - проверяет, полон ли стек

## Примеры

Запустите примеры:

```bash
cargo run --example basic_usage
```

## Лицензия

MIT License 