# Ready Set Bool

This is an educational project in the 42 cursus. It's about boolean logic, sets,
group theory and more. I chose Rust for this project as the language was free
and the examples were written specifically for it.

<br />
<p align="center">
  <a href="https://en.wikipedia.org/wiki/George_Boole">
    <img src="https://github.com/Taiwing/ready_set_boole/blob/master/resources/Boole.jpg?raw=true" alt="George Boole" title="George Boole" style="width: 50%;"/>
  </a>
</p>

## Setup

This project includes library files, an empty main function and a big test
suite. The main function is for live testing by the corrector of the project.
The test suite is to make sure I did not screw up too badly.

```shell
# clone it
git clone https://github.com/Taiwing/ready_set_boole.git
# build it and run the test suite
cd ready_set_boole && cargo test
```

## Boolean Evaluation (exercise 3)

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

For example `"1011||="` can be represented like this:

```
Logical Equivalence ('=')  
├──Disjunction ('|')  
│  ├──Disjunction ('|')  
│  │  ├──True ('1')  
│  │  └──True ('1')  
│  └──False ('0')  
└──True ('1')  
```

Or like this: `((T ∨ T) ∨ ⊥) ⇔ T = T`

Here the Logical Equivalence operation is the highest and will be fully
evaluated last since it depends on every other operation result.

## Truth Table (exercise 4)

The other boolean-themed exercises introduce boolean variables. The 4th one is
about printing the truth table of a given formula. This gives the result for
each possible value set. The `print_truth_table()` function is to be implemented
with a maximum time complexity of O(2^n). This is because a simple implentation
of this function will replace each variable by a given literal value and call
the `eval_formula()` on the result. Since a boolean has two possible values the
number of operations will double for each additional variable in the formula.

> The `print_truth_table()` function accepts every operator from the above table
> except for the literals '0' and '1' which are replaced by letters from 'A' to
> 'Z' (each representing a different variable).

### example:

```rust
let formula = "AB|";
print_truth_table(formula);
```

Will print:

| A | B | = |
|---|---|---|
| 0 | 0 | 0 |
| 0 | 1 | 1 |
| 1 | 0 | 1 |
| 1 | 1 | 1 |

Which is simply the truth table for the Disjunction (OR) operator. But if we add
an other variable, the table will be twice as big:

```rust
let formula = "AB|C&";
print_truth_table(formula);
```

| A | B | C | = | *Formula*       |
|---|---|---|---|-----------------|
| 0 | 0 | 0 | 0 | (⊥ ∨ ⊥) ∧ ⊥ = ⊥ |
| 0 | 0 | 1 | 0 | (⊥ ∨ ⊥) ∧ T = ⊥ |
| 0 | 1 | 0 | 0 | (⊥ ∨ T) ∧ ⊥ = ⊥ |
| 0 | 1 | 1 | 1 | (⊥ ∨ T) ∧ T = T |
| 1 | 0 | 0 | 0 | (T ∨ ⊥) ∧ ⊥ = ⊥ |
| 1 | 0 | 1 | 1 | (T ∨ ⊥) ∧ T = T |
| 1 | 1 | 0 | 0 | (T ∨ T) ∧ ⊥ = ⊥ |
| 1 | 1 | 1 | 1 | (T ∨ T) ∧ T = T |

## Alternative Forms (exercises 5 and 6)

The next exercises are about implementing functions that convert the given
propositional formulas into predefined forms through rewriting rules. In the
implementation of the following functions an
[AST](https://en.wikipedia.org/wiki/Abstract_syntax_tree) has been chosen to
represent the formulas because it is one of the easiest ways to apply the
rewriting rules below.

The `negation_normal_form()` function takes a formula and converts it into
[NNF](https://en.wikipedia.org/wiki/Negation_normal_form). The NNF makes every
negation 'go down' the formula tree so that they can only take variables as
operands. It also replaces every Exclusive Disjunction, Material Condition and
Logical Equivalence by equivalent propositions only using Variables, Negations,
Disjunctions and Conjunctions.

The `conjunctive_normal_form()` function converts the given formula into
[CNF](https://en.wikipedia.org/wiki/Conjunctive_normal_form). CNF has the same
requirements as NNF. So every CNF proposition is also NNF but not the reverse
since it adds an other condition. A CNF proposition is a conjunction of multiple
clauses where a clause is itself a disjunction of literals. This means that in
reverse polish notation every Conjunctive operator must be at the end of the
formula.

This is done using the following rewriting rules:

* Double Negation: *¬¬A ⇔ A*
* De Morgan's Law 1: *¬(A ∨ B) ⇔ ¬A ∧ ¬B*
* De Morgan's Law 2: *¬(A ∧ B) ⇔ ¬A ∨ ¬B*
* Exclusive Disjunction: *A ⊕ B ⇔ (A ∧ ¬B) ∨ (¬A ∧ B)*
* Material Condition: *A ⇒ B ⇔ ¬A ∨ B*
* Logical Equivalence: *(A ⇔ B) ⇔ (A ⇒ B) ∧ (B ⇒ A)*
* Conjunctive Distributivity: *A ∧ (B ∨ C) ⇔ (A ∧ B) ∨ (A ∧ C)*
* Disjunctive Distributivity: *A ∨ (B ∧ C) ⇔ (A ∨ B) ∧ (A ∨ C)*

To make a given formula CNF, the dijunctive distributivity rule must be applied
in its general form, which is:
> *(P<sub>1</sub> ∧ P<sub>2</sub> ... ∧ P<sub>n</sub>)
> ∨ (Q<sub>1</sub> ∧ Q<sub>2</sub> ... ∧ Q<sub>m</sub>) ⇔
> (P<sub>1</sub> ∨ Q<sub>1</sub>) ∧ (P<sub>1</sub> ∨ Q<sub>2</sub>) ...
> ∧ (P<sub>1</sub> ∨ Q<sub>m</sub>) ∧ (P<sub>2</sub> ∨ Q<sub>1</sub>)
> ∧ (P<sub>2</sub> ∨ Q<sub>2</sub>) ... ∧ (P<sub>2</sub> ∨ Q<sub>m</sub>) ∧ ...
> ∧ (P<sub>n</sub> ∨ Q<sub>1</sub>) ∧ (P<sub>n</sub> ∨ Q<sub>2</sub>) ...
> ∧ (P<sub>n</sub> ∨ Q<sub>m</sub>)*

Where every P and Q is a disjunction of literals, meaning that they do not
contain any conjunction so that every operand is either a literal, a negation of
a literal or a disjunction.

### example:

```rust
// set string to a non CNF formula
let formula = "AD&C|";
// print --> "AC|DC|&"
println!("{}", conjunctive_normal_form(formula));
```

Before CNF:

```
Disjunction
├──Variable(C)
└──Conjunction
   ├──Variable(D)
   └──Variable(A)
```

After CNF:

```
Conjunction
├──Disjunction
│  ├──Variable(C)
│  └──Variable(D)
└──Disjunction
   ├──Variable(C)
   └──Variable(A)
```
