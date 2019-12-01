extern crate dwemthys;

use dwemthys::character::Character;
use dwemthys::game::Game;
use dwemthys::npc::NPC;
use dwemthys::traits::Updates;
use dwemthys::movement::{RandomMovementComponent, TcodUserMovementComponent};
use tcod::input::KeyCode::{Escape};

fn main() {
    let mut game = Game::new();

    let chmc = Box::new(TcodUserMovementComponent{window_bound: game.window_bound});
    let mut ch = Character::new(40, 25, '@', chmc);

    let cmc = Box::new(RandomMovementComponent{window_bound: game.window_bound});
    let dmc = Box::new(RandomMovementComponent{window_bound: game.window_bound});

    let dog = Box::new(NPC::new(10, 10, 'd', dmc));
    let cat = Box::new(NPC::new(40, 25, 'c', cmc));

    let mut npcs: Vec<Box<dyn Updates>> = vec![dog, cat];

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
