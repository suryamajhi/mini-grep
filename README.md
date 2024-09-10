# mini-grep

Practice project in rust
Just match limited regular expressions.
## Usage
```bash
echo <input_text> | your_program -E <pattern>
```
Example:
```bash
echo -n "cat" | ./target/debug/mini-grep -E "(cat|dog)" 
```