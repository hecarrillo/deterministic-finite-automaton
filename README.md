# deterministic-finite-automata
Deterministic Finite Automata to enforce the patterns described by a regular language. The examples in this specific project include the following pattern checkers for strings: 
## Scientific Notation
- Accepted string examples: 123e+3, 2.3E3, 123, 345.4e-3
- Rejected string examples: 234.234.3, 5-2, 45e34.3
<p align="center">
<img width="700" alt="image" src="https://user-images.githubusercontent.com/55115748/205177923-18912a95-349b-4d3f-bde4-0f7440f7eb58.png">
</p>

## Pair number of 0's without successive 1's

- Alphabet(Sigma): {1,0}
- Accepted string examples: 1001, 101010101, 00, 00001
- Rejected string examples: 1100, 1011, 101010, 000
<p align="center">
<img width="700" alt="image" src="https://user-images.githubusercontent.com/55115748/205178407-7559351a-2a1b-4295-ba02-f132095a474b.png">
</p>

## Successive Letters
- Alphabet(Sigma): {a, b, c, d}
- Accepted string examples: aa, abcdd, abcc, aaaaab
- Rejected string examples: abcd, a, abc, ba
<p align="center">
<img width="700" alt="image" src="https://user-images.githubusercontent.com/55115748/205179347-f149c789-bf61-4a46-a575-b4e092d99000.png">
</p>

## Date Notation 
Note: The format should be day -> month -> year
- Accepted string examples: 12/12/12, 3.4.2022, 31-1-2000
- Rejected string examples: 32/12/2000, 3-13-2010, 234-3
<p align="center">
<img width="700" alt="image" src="https://user-images.githubusercontent.com/55115748/205179273-d50313e2-cc89-4081-a7cf-b8f08298c9ff.png">
</p>

## How to Run
Clone the Repo and simply `cargo run` on the project's root folder.
### Execution Example:
<img width="500" alt="image" src="https://user-images.githubusercontent.com/55115748/205180619-971c8d6c-31de-4645-ad57-4e852aba4caa.png">
<img width="500" alt="image" src="https://user-images.githubusercontent.com/55115748/205180681-b293d238-4674-4d21-a73d-fcc6bd9579e1.png">


All of these examples have different state machines, but share the same structure in order to build the States and Transition Functions between them.
</br>
Built in Rust. Implementation inspired by <a href="https://hoverbear.org/blog/rust-state-machine-pattern/">Pretty State Machine Patterns in Rust by Anna Hoverbear</a>


