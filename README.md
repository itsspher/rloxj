This is a Rust implementation of "Lox", a language described in [Crafting Interpreters](https://craftinginterpreters.com/).

This project was made to compensate for a botched class in compilers that was given in my final semester in college.  
Since I learned nothing due to the professors never showing up, this project exists.

This rust implementation is complete up to functions. Variable resolution and classes, the last two features, will be implemented at a later date.

Technical notes:

- my big mistake is not implementing visitor patterns described in the book. Reference to the generics of a visitor pattern is found here: [Visitor - Rust Design Patterns](https://rust-unofficial.github.io/patterns/patterns/behavioural/visitor.html).
- not implementing visitor patterns is an easy way to force yourself into hacking and writing bad code when you're dealing with the variable resolution pass
- hacking and writing "bad code" is a good way to find useful (anti)patterns like `Rc<RefCell<T>>`.