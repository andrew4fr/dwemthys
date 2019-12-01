extern crate dwemthys;

use dwemthys::game::Game;
use dwemthys::actor::Actor;
use dwemthys::movement::{RandomMovementComponent, TcodUserMovementComponent};
use tcod::input::KeyCode::{Escape};

fn main() {
    let mut game = Game::new();

    let chmc = Box::new(TcodUserMovementComponent{window_bound: game.window_bound});
    let mut ch = Actor::new(40, 25, '@', chmc);

    let cmc = Box::new(RandomMovementComponent{window_bound: game.window_bound});
    let dmc = Box::new(RandomMovementComponent{window_bound: game.window_bound});

    let dog = Box::new(Actor::new(10, 10, 'd', dmc));
    let cat = Box::new(Actor::new(40, 25, 'c', cmc));

    let mut npcs: Vec<Box<Actor>> = vec![dog, cat];

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
