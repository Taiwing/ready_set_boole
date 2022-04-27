# Ready Set Bool

This is an educational project in the 42 cursus. It's about boolean logic, sets,
group theory and more. I chose Rust for this project as the language was free
and the examples were written specifically for it.

This project includes library files, an empty main function and a big test
suite. The main function is for live testing by the corrector of the project.
The test suite is to make sure I did not screw up too badly.

## Setup

```shell
# clone it
git clone https://github.com/Taiwing/ready_set_boole.git
# build it and run the test suite
cd ready_set_boole && cargo test
```

## Boolean Evaluation

The third exercise of the subject is about implementing a function which takes
a formula in
[reverse polish notation](https://en.wikipedia.org/wiki/Reverse_Polish_notation),
evaluates it and returns the result in O(n) time. This means that the execution
time must grow linearily to the size of the input. Simply put, evaluation of a
formula 10 times bigger than an other one should take no more than 10 times
longer.

This is done with a simple boolean stack. The formula string is read character
by character. Each literal symbol (true or false) is directly pushed to the
stack. When an operator character is found, one value is poped from the stack
if it's a negation and two for every other operation. The actual operation is
then performed and the result is pushed to the stack.

### Operations

| Symbol | Mathematical Equivalent | Meaning                     |
|:------:|:-----------------------:|-----------------------------|
| 0      | ⊥                       | False                       |
| 1      | T                       | True                        |
| !      | ¬                       | Negation (NOT)              |
| &      | ∧                       | Conjunction (AND)           |
| \|     | ∨                       | Disjunction (OR)            |
| ^      | ⊕                       | Exclusive Disjunction (XOR) |
| >      | ⇒                       | Material Condition          |
| =      | ⇔                       | Logical Equivalence         |

#### example:

```rust
// Write the formula in Reverse Polish Notation
let formula = "10&"; // This means: True AND False
// Prints "result: false"
println!("result: {}", eval_formula(formula));
```

More Complex formulas are hard to read and are better understood when
represented in a tree format, which is easy to do from RPN string.

For example "1011||=" can be represented like this:

Logical Equivalence ('=')  
├──Disjunction ('|')  
│  ├──Disjunction ('|')  
│  │  ├──True ('1')  
│  │  └──True ('1')  
│  └──False ('0')  
└──True ('1')  

Here the Logical Equivalence operation is the highest and will be fully
evaluated last since it depends on every other operation result.

The other boolean-themed exercises introduce variables into the formula. The 4th
one is about printing the truth table of a given formula. This gives the result
of the evaluated formula for each possible variable value set.

### example:

We replace each literal value for the above example with a different variable.

```rust
let formula = "ABCD||=";
print_truth_table(formula);
```

Which will print:

| A | B | C | D | = |
|---|---|---|---|---|
| 0 | 0 | 0 | 0 | 1 |
| 0 | 0 | 0 | 1 | 0 |
| 0 | 0 | 1 | 0 | 0 |
| 0 | 0 | 1 | 1 | 0 |
| 0 | 1 | 0 | 0 | 0 |
| 0 | 1 | 0 | 1 | 0 |
| 0 | 1 | 1 | 0 | 0 |
| 0 | 1 | 1 | 1 | 0 |
| 1 | 0 | 0 | 0 | 0 |
| 1 | 0 | 0 | 1 | 1 |
| 1 | 0 | 1 | 0 | 1 |
| 1 | 0 | 1 | 1 | 1 |
| 1 | 1 | 0 | 0 | 1 |
| 1 | 1 | 0 | 1 | 1 |
| 1 | 1 | 1 | 0 | 1 |
| 1 | 1 | 1 | 1 | 1 |
