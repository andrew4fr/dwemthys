extern crate dwemthys;

use dwemthys::character::Character;
use dwemthys::game::Game;
use dwemthys::npc::NPC;
use dwemthys::traits::Updates;
use tcod::input::KeyCode::{Escape};

fn main() {
    let mut game = Game::new();
    let mut ch = Character::new(40, 25, '@');
    let dog = Box::new(NPC::new(10, 10, 'd'));
    let cat = Box::new(NPC::new(40, 25, 'c'));
    let mut npcs: Vec<Box<dyn Updates>> = vec![dog, cat];

    game.render(&npcs, ch);
    while !game.exit {
        let keypress = game.wait_for_keypress();

        if keypress.pressed {
            match keypress.code {
                Escape => game.exit = true,
                _ => {}
            }
        }

        game.update(&mut npcs, &mut ch, keypress.code);
        game.render(&npcs, ch);
    }
}
