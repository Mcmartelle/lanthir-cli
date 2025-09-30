# Lanthir CLI

Command line flow chart follower/runner and logger.

```
Usage: lanthir [OPTIONS]

Options:
  -i, --input <INPUT>
      --log-path <LOG_PATH>
  -l, --log <LOG>            [possible values: true, false]
      --verbose
  -h, --help                 Print help
  -V, --version              Print version
```

## Supported File Types

- Mermaid .mmd
  - Fully featured flowchart spec
  - Only the "Flowchart" portion of mermaid's syntax is supported
  - The mermaid way of escaping double quote marks `"` is not convenient.
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
  - [Blog post about OATS](https://www.martelle.dev/2025-07-01_on_notes_and_todo_lists.html)
  - Example:
  ```OATS
  ~ Do this first
  ~ And then this
  ~ And then this
  
  & Do this
  & And this
  & And this
  & And this, but in any order

  & The extra newline above me
  & Means this chunk of "&" nodes
  & will be done after those four
  
  | Do this
  | Or this
  | Or this, but just one

  | Again, newlines can break up
  | Chunks of "&" and "|" nodes
  
  ? Do this or don't, it's optional
  
  ~ Then do this
  = This text will be copied to the system clipboard when you get to "~ Then do This"
  = It can be multiline as well.
  =   leading spaces after the first space will be copied
  ```

