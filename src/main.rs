mod game;

fn main() {
    loop {
        let restart = game::run();
        if !restart { break; }
    }
}
