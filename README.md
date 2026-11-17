# wordnum

a simple program that translates english enumeration to integers.

---

### why???

so, a few friends ran a little competition. and I decided to participate because why not lol?

I am already working on the development of imi-lang (currently paused due to other reasons). and I got more used to implementing parsers, so this seemed like a good opportunity to train my skills.

> the rules for the challenge are [*here*](./rules.md) btw

---

# Installation

as always, just install/build it using [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html).

clone the repo:

```sh
git clone https://github.com/sunless2day/wordnum
```

`cd` into it and run:

```sh
cargo install --path .
```

that's it. wordnum is now on your PATH (only if cargo's binary folder, usually `~/.cargo/bin`, is in your PATH too), so you can run it from anywhere.

if you just want to try it without installing anything, `cargo run -- two` works too.

it should work for most OSes, including Linux, Windows and Mac.

---

# Usage

it is very straightforward, really.

### `wordnum` has two modes.

##### you can:

1. pass it CLI arguments, like `wordnum one hundred and sixty-seven`. in that case, it will return a single number and exit the program.
2. simply running the program without passing any arguments will set it to a REPL mode. you can exit it with `CTRL+C` or `CTRL+D` whenever you want.

### how big can it go?

the rules only asked for up to 999 billion. I went a bit further, because why not (again).

it handles anything from `zero` all the way up to `nine hundred ninety-nine quadrillion nine hundred ninety-nine trillion ...` (you get the idea), which is just under one quintillion.

negative numbers work too, just put `minus` or `negative` in front.

you can also say `a hundred` or `a million` instead of `one hundred` or `one million`, like a normal person would. but only at the very start (after `minus`, if there is one) and right before a scale word. `a twenty` is not a thing.

no decimals though. `one hundred point two` re-prompts the user (or simply errors outside the REPL), as the rules allow.

---

# examples

```
❯ wordnum one hundred and sixty seven
167
```

```
❯ wordnum minus one million two hundred and fifty nine thousand six hundred and seventy four
-1,259,674
```

```
❯ wordnum negative eight
-8
```

```
❯ wordnum
>>> five hundred
500
>>> three million, eight hundred, and sixty-eight
3,000,868
>>> a hundred and five
105
>>> a million five hundred
1,000,500
```

even though the rules never asked for it, `wordnum` also handles bad input. I’d rather get an error than have `five thirteen` quietly turn into `18` (that would be very cursed).

```
❯ wordnum
>>> balls
error: unknown word 'balls'
>>> one million sixty billion
error: 'billion' can't come after 'million'
>>> five eighty
error: unexpected 'eighty'
>>> eighty twelve 
error: unexpected 'twelve'
>>> a twenty
error: unexpected 'a'
>>> two million a thousand
error: unexpected 'a'
>>> three hundred and zero
error: unexpected 'zero'
>>> three hundred and one
301
>>> that worked!
error: unknown word 'that'
>>> okay, not this 
error: unknown word 'okay'
>>> ugh...
error: unknown word 'ugh...'
```

yeah, that's it. I think there isn't much more to add. okay, bye
