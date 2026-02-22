# Overview
Minigrep is a project based on the RustBooks [mini grep project](https://doc.rust-lang.org/book/ch12-00-an-io-project.html).
I added some personal changes to how it works and performs.

## Example commands

### Using cargo

The following syntax is how to run the minigrep program
```bash
cargo run -- {searchquery} {filepath} {case-sensitive}
```

Using the syntax above, here is a working example:
```bash
cargo run -- somebody poem.txt true


Outputs:
-- Setup ---
Search query: somebody
Filepath: poem.txt
Case sensitive: true

-- Search results found ---
How dreary to be somebody!
```


### Using release compiled binary

Search example
```bash
./minigrep somebody poem.txt
-- Setup ---
Search query: somebody
Filepath: poem.txt
Case sensitive: false

-- Search results found ---
How dreary to be somebody!

```


If you search for something that does not exist
```bash
./minigrep javascript poem.txt
-- Setup ---
Search query: javascript
Filepath: poem.txt
Case sensitive: false

-- No search results found ---⏎    
```
