# Scripting reference

## Functions
### Process
This is the function that greatness calls to process the files. It should return the final contense of the file. It takes in the contense of the file, and then the filename.
#### Example
```rust
// Input: Hello, my name is astrid {{ lastname }}.

fn process(data, filename) {
    data.trim();
    data = data.replace("{{ lastname }}", "greenwood");
    return data;
}

// Output (written to file): Hello, my name is astrid greenwood.
```
