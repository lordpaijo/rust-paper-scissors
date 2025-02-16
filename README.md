# RUST, PAPER, SCISSORS!

This is my first rust-written project: a rock, paper, and scissors game. I have a lot of fun with it but don't expect me to get a job. It was built using cargo.

![skrinsut](https://github.com/lordpaijo/rust-paper-scissors/blob/Main/screenshots/ss_0.png)

## Dependencies
 - cargo (duh)
 - rand
 - colored
 - clap

## Installing and Running

The package is available at [Crates.io](https://crates.io/crates/rps_paijo). You can install it via [Cargo](https://crates.io/), or by building it yourself. (Make sure you have cargo installed already.)

### Cargo
```sh
$ cargo install rps_paijo
```

### Build from source
Clone the repo 👹.
```sh
$ git clone https://github.com/lordpaijo/rust-paper-scissors.git # https
$ git clone git@github.com:lordpaijo/rust-paper-scissors.git     # ssh
$ cd rust-paper-scissors/
```

Now let's get rusty...
```
$ cargo update # updating the project dependencies...
$ cargo build # buzzzbuzzbuzzz, here comes the slow-ass build...
```

If you have finished installing or building, you can now play the game. If you installed the already published package, then you can execute it by typing `rps_paijo` on your terminal just like any other CLI app. Or if you built it from the source, then do the following:

```sh
$ cd rust-paper-scissors/
$ cargo run
```

## Playing
I don't think I have to tell you how to play the game in general, it's the same as any other rock, paper, and scissors. However, I want to address some additional features which will be separated into two forms, arguments and commands.

### Arguments
Arguments can be called by typing them after calling the game. Like flags or tags in any other program, they can be called using `-` or `--` before them. For example, `--help` will give you the help which what's inside it is the page of the list of available arguments.

| Arguments | Parameters | Types | Functions | Syntax |
|-----------|------------|:-----:|-----------|--------|
| `-h` `--help` |  | `bool` | Prints the help page | `rps_paijo --help` |
| `-V` `--version` |  | `bool` | Prints the game's version | `rps_paijo --version` |
| `-r` `--rounds` | `<ROUNDS>` e.g.: 15 | `u64` | Sets the round limit | `rps_paijo --rounds <ROUNDS>` |
| `-a` `--auto` |  | `bool` | Sets mode to auto (bot vs bot) | `rps_paijo --auto` |
| `-b` `--boost` | `<BOOST>` e.g.: 3 | `i32` | Increases both player and bot's score | `rps_paijo --boost <BOOST>` |
| `--boost-player` | `<BOOST_PLAYER>` e.g.: 2 | `i32` | Increases only the player's score | `rps_paijo --boost-player <BOOST_PLAYER>` |
| `--boost-bot` | `<BOOST_BOT>` e.g.: 1 | `i32` | Increases only the bot's score | `rps_paijo --boost-bot <BOOST_BOT>` |
| `-H` `--handicap` | `<HANDICAP>` e.g.: 2 | `i32` | Decreases both player and bot's score | `rps_paijo --handicap <HANDICAP>` |
| `--handicap-player` | `<HANDICAP_PLAYER>` e.g.: 1 | `i32` | Decreases only the player's score | `rps_paijo --handicap-player <HANDICAP_PLAYER>` |
| `--handicap-bot` | `<HANDICAP_BOT>` e.g.: 1 | `i32` | Decreases only the bot's score | `rps_paijo --handicap-bot <HANDICAP_BOT>` |
| `-s` `--skip` | `<SKIP ROUND>` e.g.: 5 | `u64` | Skips a single round | `rps_paijo --skip <SKIP ROUND>` |
| `--skip-rounds` | `<SKIP ROUNDS>` e.g.: (3, 5, 8) | `Vec<u64>` | Skips multiple specified rounds | `rps_paijo --skip-rounds <SKIP ROUNDS>` |
| `--skip-even` |  | `bool` | Skips rounds with even numbers | `rps_paijo --skip-even` |
| `--skip-odd` |  | `bool` | Skips rounds with odd numbers | `rps_paijo --skip-odd` |
| `--skip-prime` |  | `bool` | Skips rounds that are prime numbers | `rps_paijo --skip-prime` |
| `--skip-all` |  | `bool` | Skips all rounds after round 1 | `rps_paijo --skip-all` |
| `--set-start-round` | `<START ROUND>` e.g.: 12 | `u64` | Starts the game at a specific round | `rps_paijo --set-start-round <START ROUND>` |
| `--set-even-round` |  | `bool` | Only plays rounds with even numbers | `rps_paijo --set-even-round` |
| `--set-odd-round` |  | `bool` | Only plays rounds with odd numbers | `rps_paijo --set-odd-round` |
| `--set-prime-round` |  | `bool` | Only plays rounds that are prime numbers | `rps_paijo --set-prime-round` |

### Commands
As for the commands, they are special in-game commands that do simple yet useful things... yeah that's dumb.

| Commands | Functions |
|----------|-----------|
| `restart` / `reset` | Restarts the game |
| `end` | Ends the game with a result |
| `exit` | Exits the game without showing any results |

And that's everything, I hope... Enjoy playing!

## Thanks...............................................
