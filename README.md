# AOC in rust

This is my attempt to learn the rust language, by solving Advent of Code
problems.

## Dependencies

I try to keep the dependencies as few as possible when solving the tasks,
but for the support code there is no rules.

Solutions which use foregin crates are:

- 2015-04:
  - `md-5`: compute md5
  - `num_cpus`: Find number of threads for threading
- 2015-12:
  - `serde_json`: To read json

## How to run

The system fetches and caches inputs from the AoC website. To do this it
needs a session cookie. You can find this by logging into the AoC website
and looking in your cookie store. It will search for these values in
environment variables starting with `AOC_SESSION_` and you can have multiple.

Different sessions get different outputs, so you can check that your solution
is robust by setting multiple and seeing if it passes all the inputs.

## How we check answers

This is sort of like how you check inputs, but we first check if there is
any cached output that we know is correct.

```
POST /<year>/day/<day>/answer
level: <1 or 2>
answer: <the answer>
```


Look for that is the right answer?

Look for it in the about page?
