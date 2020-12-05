## Advent Of Code Challenge

This repo contains my submission(s) for [Advent of code - 2020](https://adventofcode.com/2020).
The submissions are written in Rust, the one that I am actively learning ;).

Please note that the quality of code is NOT my number one priority but I try to write as good as I can while tackling the problem.
Unfortunately, I do not have enough time to iterate over each problem and improve the code.
But maybe during Christmas vacation when I get bored!

Therefore, there are also no tests present.

And, I have also not taken care of the error handling where the return type is `Result`.
Similar to the following should be in place where `unwrap()` is being used.

```
let var = match var {
    Ok(value) => value,
    Err(e) => return Err(e),
};
```
