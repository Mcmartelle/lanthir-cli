# Lanthir CLI

Command line flow chart follower/runner and logger.


## Supported File Types

- Mermaid .mmd
  - Fully featured flowchart spec
  - Only the "Flowchart" portion of mermaid's syntax is supported
  - Example:
  ```Mermaid
  flowchart TD
    Start-->A
    A["run[echo #quot;hello#quot;]"]-->B(Hooray the command succeeded)
    B-->C["run[echo #quot;world#quot;]"]
    C-->|foo|D(Fooby)
    C--bar-->E(Barry)
    D & E --> End
  ```
- Checklist .ckl .txt
  - Very simple format with line separated tasks and copiable strings quoted with triple backticks
  - Example:
  ```CKL
  hello
  world
  copy this! ```blah blah blah``` to your clipboard
  ```cool beans```
  ```
- OATS .oats
  - Simple but expressive format with Or And Then Sequences (OATS)
  - Example:
  ```OATS
  ~ Do this first
  ~ And then this
  ~ And then this
  
  & Do this
  & And this
  & And this
  & And this, but in any order
  
  | Do this
  | Or this
  | Or this, but just one
  
  ? Do this or don't, it's optional
  
  ~ Then do this
  = This text will be copied to the system clipboard when you get to "~ Then do This"
  = It can be multiline as well.
  ```

