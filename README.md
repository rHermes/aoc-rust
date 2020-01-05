# AOC in rust

This is my attempt to learn the rust language, by solving Advent of Code
problems.

## How to run

The system fetches and caches inputs from the AoC website. To do this it
needs a session cookie. You can find this by logging into the AoC website
and looking in your cookie store. It will search for these values in
environment variables starting with `AOC_SESSION_` and you can have multiple.

Different sessions get different outputs, so you can check that your solution
is robust by setting multiple and seeing if it passes all the inputs.
