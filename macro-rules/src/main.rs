/*
Macro rules in Rust allow you to define patterns that can be used to generate code. They are a powerful way to create reusable code snippets and can help reduce boilerplate. In this example, we will define a simple macro called `print_hello` that takes a name as an argument and prints a greeting.

Example in C++:
```cpp
#include <iostream>
#define PRINT_HELLO(name) std::cout << "Hello, " << name << "!" << std::endl;

int main() {
  PRINT_HELLO("Alice");
  PRINT_HELLO("Bob");
  return 0;
}
```
*/

// Define the macro using the `macro_rules!` macro. The name of the macro is `print_hello`.
macro_rules! print_hello {
    // This pattern matches a single expression (the name) and generates code to print a greeting.
    ($name:expr) => {
        // expr means that the macro expects an expression as an argument.
        println!("Hello, {}!", $name); // Rust's println! macro is used to print the greeting, and {} is a placeholder for the name. Generally like when use fmt in C++. The $name is the variable that will be replaced with the actual argument passed to the macro.
    };
}

fn main() {
    // String literals
    print_hello!("Alice");
    print_hello!("Bob");
}
