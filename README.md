This is a Rust implementation of "Lox", a language described in [Crafting Interpreters](https://craftinginterpreters.com/).

This project was made to compensate for a botched class in compilers that was given in my final semester in college.  
Since I learned nothing due to the professors never showing up, this project exists.

This rust implementation is complete up to variable resolution. I do not implement classes. Unfortunately due to the need to refocus my efforts on looking for a job, this project must be dropped.

Technical notes:

- my big mistake is not implementing visitor patterns described in the book. Reference to the generics of a visitor pattern is found here: [Visitor - Rust Design Patterns](https://rust-unofficial.github.io/patterns/patterns/behavioural/visitor.html).
- not implementing visitor patterns is an easy way to force yourself into hacking and writing bad code when you're dealing with the variable resolution pass
- I advise anyone using my implementation as a reference to find a different reference.

Overall I learned a lot, but still have a ways to go.