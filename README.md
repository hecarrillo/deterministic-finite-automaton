# deterministic-finite-automata
A deterministic finite automaton (DFA) is a mathematical model used in computer science and theoretical linguistics to recognize patterns in string inputs. It is a type of finite state machine that operates by reading a sequence of symbols from an input string and moving through a series of states based on those symbols. The DFA accepts or rejects the input string based on whether the final state it reaches after processing the input is an accepting state or not. In this project, several DFAs have been developed to check string inputs for various patterns and purposes. These DFAs have been implemented using Rust and can be used to validate string inputs for specific patterns or conditions. There's also the option to validate a string with a custom DFA.

## Custom DFA using a JSON config file
- Allows the user to create their own DFA using a JSON config file (src/input_dfa.json), which can be used to validate a string input. 
- The JSON config file contains the following fields:
  - `tokens`: The tokens or alphabet accepted for the DFA
  - `states`: The states of the DFA
  - `initial_state`: The initial state of the DFA
  - `accepted_states`: The accepting states of the DFA
  - `transitions`: The transition functions of the DFA, where each transition is represented by having the state as the key and the transition function as the value. For each input token, the transition function returns the next state.

<p align="center">

|JSON config|DFA generated|
|---|---|
|<img width="300" alt="image" src="https://user-images.githubusercontent.com/55115748/224799394-a4df0902-cc16-4a49-9efa-f9bdea82f463.png">|<img width="300" alt="image" src="https://user-images.githubusercontent.com/55115748/224801709-06a6e70d-02bf-4de1-9593-6829cc3e0b41.png">|

### Execution example for string validation
<img width="500" alt="Screenshot 2023-03-13 at 12 59 53" src="https://user-images.githubusercontent.com/55115748/224802843-e83ce2a7-d953-4bd5-b0a8-fc6bca33587f.png">

</p>

## Pre-configured DFAs to check common strings

### Scientific Notation
- Accepted string examples: 123e+3, 2.3E3, 123, 345.4e-3
- Rejected string examples: 234.234.3, 5-2, 45e34.3
<p align="center">
<img width="700" alt="image" src="https://user-images.githubusercontent.com/55115748/205177923-18912a95-349b-4d3f-bde4-0f7440f7eb58.png">
</p>

### Pair number of 0's without successive 1's

- Alphabet(Sigma): {1,0}
- Accepted string examples: 1001, 101010101, 00, 00001
- Rejected string examples: 1100, 1011, 101010, 000
<p align="center">
<img width="700" alt="image" src="https://user-images.githubusercontent.com/55115748/205178407-7559351a-2a1b-4295-ba02-f132095a474b.png">
</p>

### Successive Letters
- Alphabet(Sigma): {a, b, c, d}
- Accepted string examples: aa, abcdd, abcc, aaaaab
- Rejected string examples: abcd, a, abc, ba
<p align="center">
<img width="700" alt="image" src="https://user-images.githubusercontent.com/55115748/205179347-f149c789-bf61-4a46-a575-b4e092d99000.png">
</p>

### Date Notation 
Note: The format should be day -> month -> year
- Accepted string examples: 12/12/12, 3.4.2022, 31-1-2000
- Rejected string examples: 32/12/2000, 3-13-2010, 234-3
<p align="center">
<img width="700" alt="image" src="https://user-images.githubusercontent.com/55115748/205179273-d50313e2-cc89-4081-a7cf-b8f08298c9ff.png">
</p>

## How to Run
Clone the Repo and simply `cargo run` on the project's root folder.
### Execution Example:

<img width="500" alt="Screenshot 2023-03-13 at 12 57 02" src="https://user-images.githubusercontent.com/55115748/224802311-61b6c5b7-e695-4fe5-8901-88ab20a4210d.png">
<img width="500" alt="Screenshot 2023-03-13 at 12 58 28" src="https://user-images.githubusercontent.com/55115748/224802604-6723b27f-3aca-49ea-8aea-78a08cdafd27.png">

All of these examples have different state machines, but share the same structure in order to build the States and Transition Functions between them.
</br>
Built in Rust. Implementation inspired by <a href="https://hoverbear.org/blog/rust-state-machine-pattern/">Pretty State Machine Patterns in Rust by Anna Hoverbear</a>


