extern crate dwemthys;

use dwemthys::game::Game;
use dwemthys::actor::Actor;
use tcod::input::KeyCode::{Escape};

fn main() {
    let mut game = Game::new();

    let mut ch = Actor::heroine(game.map_window.get_bound());

    let mut npcs: Vec<Box<Actor>> = vec![
        Box::new(Actor::dog(10, 10, game.map_window.get_bound())),
        Box::new(Actor::cat(40, 25, game.map_window.get_bound())),
        Box::new(Actor::kobold(20, 20, game.map_window.get_bound())),
    ];

    game.render(&npcs, &ch);
    while !game.exit {
        let keypress = game.wait_for_keypress();

        if keypress.pressed {
            match keypress.code {
                Escape => game.exit = true,
                _ => {}
            }
        }

        game.update(&mut npcs, &mut ch);
        game.render(&npcs, &ch);
    }
}
