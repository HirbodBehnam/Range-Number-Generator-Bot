# Range Number Generator Bot

A Telegram bot to generate a random numbers in some ranges based on given number. Please note that one input number generates same set of random numbers.

I mostly created this project in order to practice Rust and create a question number generator for each student at the BP class as TA.

## Usage

Simply, compile this project with `cargo build --release`.
Then, create a file named `ranges.txt` and place it next to the compiled binary. We will talk about this file in next section.
After, set the bot token via `TELOXIDE_TOKEN` environment variable and run the built binary.


## `rages.txt` File Format

The ranges should look like this

```
begin1 end1
begin2 end2
...
```

On each line place the one range of numbers you want to have. The end is exclusive but the begin is inclusive. For example:

```
1 5
5 8
8 10
1 10 
```

creates four numbers in ranges of `[1, 5)`, `[5, 8)`, `[8, 10)` and `[1 , 10)`. As can be seen, there is no restrictions on overlaps or the order of numbers. The only restriction is that the `end` must be more than `begin`.
