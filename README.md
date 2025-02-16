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

```sh
  -r, --rounds <ROUNDS>                    
  -a, --auto                               
  -s, --skip <SKIP>                        
      --skip-rounds <SKIP_ROUNDS>          
      --skip-even                          
      --skip-odd                           
      --skip-prime                         
      --set-start-round <SET_START_ROUND>  
      --set-even-round                     
      --set-odd-round                      
      --set-prime-round                    
  -h, --help                               Print help
  -V, --version                            Print version
```

| Arguments | Parameters | Types | Functions | Syntax |
|-----------|------------|-------|-----------|--------|
| `-r` `--roounds` | `<ROUNDS>` | `u64` | Sets round limit | `rps_paijo --rounds <ROUNDS>` |
| `-a` `--auto` |  | `bool` | Sets mode to auto (bot vs bot) | `rps_paijo --auto` |
| `-s` `--skip` | `<ROUND>` | `u64` | Skips the one round | `rps_paoijo --skip <ROUND>` |
| `--skip-rounds` | `<ROUNDS>` | `u64` | Skips the inputted rounds | `rps_paijo --skip-rounds <ROUNDS>` |

### Commands
As for the commands, they are special in-game commands that do simple yet useful things... yeah that's dumb.

```
 - exit / quit : Quitting your game's session without any results.
 - restart / reset : Restart your game's session back to 0.
 - end : Ending your game's session with a result, either you win, lose, or draw.
```

And that's everything, I hope... Enjoy playing!

## Thanks......
