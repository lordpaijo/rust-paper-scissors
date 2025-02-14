# RUST, PAPER, SCISSORS!

This is my first rust-written project: a rock, paper, and scissors game. I have a lot of fun with it but don't expect me to get a job. It was built using cargo.

![skrinsut](https://github.com/lordpaijo/rust-paper-scissors/blob/Main/screenshots/ss_0.png)

## Dependencies
 - cargo (duh)
 - rand
 - colored

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

Finished installing or building, you can now play the game. If you installed the already published package, then you can execute it by typing `rps_paijo` on your terminal just like any other CLI apps. Or if you built it from source, then do the followings:

```sh
$ cd rust-paper-scissors/
$ cargo run

```

## Playing
I don't think I have to tell you how to play the game in general, it's the same as any other rock, paper, and scissors. Though I want to address some additional features which will be separated into two forms, arguments and commands.

### Arguments
Arguments can be called by typing them after calling the game. Like flags or tags in any other program, they can be called using `-` or `--` before them. For example, `--help` will give you the help page which what's isnide is the list of available arguments.

```sh
  -r, --rounds <ROUNDS>
      --auto
  -h, --help         Print help
  -V, --version      Print version

```
### Commands
As for the commands, they are special in game commands that do simple yet useful things... yeah that's dumb.

```
 - exit / quit : Quitting your game's session without any results.
 - restart / reset : Restart your game's session back to 0.
 - end : Ending your game's session with a result, either you win, lose, or draw.

```

And that's everything, I hope... Enjoy playing!

## FAQs
### What's the purpose of this project?
--- 11/2/2025 21:32 P.M ---
I made this project as a way to learn rust and its environments since I've found it interesting. And learning by doing some projects like this helps a lot for me. It's also a way for me to have fun! So don't take it too serious.

### Why Rust?
--- 11/2/2025 21:38 P.M ---
Rust is interesting, and IT IS interesting. It feels like the JavaScript of the C Family. Though I'm using it not because the reasons why others are using it, like safety or so. I can still make an unsafe program because I'm stupid anyway. But I'm using rust because it's either C++ or Rust, and the bet chose rust...

### How to Contribute?
--- 11/2/2025 21:41 P.M ---
I haven't think of about this, sooner or later ;)

## Thanks......
