# The 25 Days of Christmas

Ho, ho, ho!

This is a simple implementation for the challenges at
https://adventofcode.com/2018. Enjoy!

## Details

Each solution was created via `cargo init day_NN --vcs none`, and
(usually) have an independent `src/part1.rs` and `src/part2.rs`, so you
can easily see what was changed to finish the second half.

I focused on a balance between code readability and writing speed: I
want to get these done quickly, but I also want to come back at some
point and figure out what I did.

There is a README for each day, which is scraped directly from the
website itself to describe the problem being solved. All rights, credit,
blame, etc. goes to that website and its author(s).

## Comments

### Day 1

Did this with a throw-away bit of javascript. Shame on me! The future is
rust!

### Day 2

In hindsight, Rust already has a perfectly good `bool_to_int` in the
form of `b.into()`. (or `rev_bool_to_int` as `(!b).into()`) :shrug:

### Day 3

Part 2 should have been done with [k-d
trees](https://en.wikipedia.org/wiki/K-d_tree) or something. It's just
that build/running with `--release` is so fast, you can get away with
brute forcing the problem.

OTOH, using a [lalrpop](https://github.com/lalrpop/lalrpop) grammar for
the input is probably overkill, since it's O(n) right at the top of the
problem. Makes it easy to read, tho.

### Day 4

I'm generally trying to stick to the `std` crate, but `itertools` was
too good to pass up for this one.

### Day 5

Rust has fully bought into UTF8, so there's no easy byte -> byte
conversion for ASCII flipping upper/lowercase. (The uppercase of a
character might be a multi-char symbol, each char of which might be
multibyte). Guess I'll write my own...

### Day 6

Still should use k-d or something for part 2 instead of brute force.

### Day 7

This is a bit over-engineered; but it is, I think, the optimal
algorithm.

### Day 8

Loved this one, probably because I love recursive problems ([see day
8](https://www.google.com/search?q=recursion)).

### Day 9

Part 2 runs so slow: I should have implemented a circular buffer or
something. `cargo run --release` to get past it.

### Day 10

I assumed that the message would show at or near the local minimum, and
was right. Not sure what I would have done if that wasn't true. My
y-axis was flipped, tho, so the message appears upside-down.

### Day 11

Should have used [Summed-area
tables](https://en.wikipedia.org/wiki/Summed-area_table) for part 2. As
it is, I have just a little bit of result re-use but it's mostly brute
force. `--release` to the rescue again, I guess.

### Day 12

After the debacles on day 9 and 11, implemented my own windowed 2-way
infinite buffer, which is probably overkill.

### Day 13

This is **way** too much code. Provided an alternate solution (which
doesn't bother parsing the track) in `alt_part2.rs`, but it's really
still too long.

### Day 14

I probably should have been using slice comparison for earlier problems.
The `last_dig` check is purely for opimization, but significantly speeds
up the calculation. (Down to ~3.5 seconds on my box in debug)

### Day 15

Compulsively found the pathfinding algorithm with optimal worst-case:
the modified _A\*_ here; but lost readability, writing time, best-case
and avarage-case time, sanity in trade. Even missed some edge cases
(which fortunately didn't come up in my problem input).
