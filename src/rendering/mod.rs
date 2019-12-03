use tcod::console as console;
use tcod::{Console, RootConsole, OffscreenConsole, BackgroundFlag, Color};
use crate::util::{Bound, Point};
use tcod::input::Key;

pub struct TcodStatWindowComponent {
    pub console: OffscreenConsole,
    pub bg_color: Color,
    bound: Bound,
    messages: Vec<String>,
    max_messages: usize,
}

impl TcodStatWindowComponent {
    pub fn new(b: Bound) -> Self {
        let h = b.max.y - b.min.y + 1;
        let w = b.max.x - b.max.y + 1;
        let console = OffscreenConsole::new(w, h);
        TcodStatWindowComponent{
            console: console,
            bg_color: Color::RED,
            bound: b,
            messages: vec!(),
            max_messages: w as usize,
        }
    }

}

pub struct TcodInputWindowComponent {
    pub console: OffscreenConsole,
    pub bg_color: Color,
    bound: Bound,
    messages: Vec<String>,
    max_messages: usize,
}

impl TcodInputWindowComponent {
    pub fn new(b: Bound) -> Self {
        let h = b.max.y - b.min.y + 1;
        let w = b.max.x - b.max.y + 1;
        let console = OffscreenConsole::new(w, h);
        TcodInputWindowComponent{
            console: console,
            bg_color: Color::GREEN,
            bound: b,
            messages: vec!(),
            max_messages: w as usize,
        }
    }
}

pub struct TcodMessageWindowComponent {
    pub console: OffscreenConsole,
    pub bg_color: Color,
    bound: Bound,
    messages: Vec<String>,
    max_messages: usize,
}

impl TcodMessageWindowComponent {
    pub fn new(b: Bound) -> Self {
        let h = b.max.y - b.min.y + 1;
        let w = b.max.x - b.max.y + 1;
        let console = OffscreenConsole::new(w, h);
        TcodMessageWindowComponent{
            console: console,
            bg_color: Color::BLUE,
            bound: b,
            messages: vec!(),
            max_messages: w as usize,
        }
    }
}

pub struct TcodMapWindowComponent {
    pub console: OffscreenConsole,
    pub bg_color: Color,
    bound: Bound,
    messages: Vec<String>,
    max_messages: usize,
}

impl TcodMapWindowComponent {
    pub fn new(b: Bound) -> Self {
        let h = b.max.y - b.min.y + 1;
        let w = b.max.x - b.max.y + 1;
        let console = OffscreenConsole::new(w, h);
        TcodMapWindowComponent{
            console: console,
            bg_color: Color::BLACK,
            bound: b,
            messages: vec!(),
            max_messages: w as usize,
        }
    }
}

pub trait WindowComponent {
    fn get_bound(&self) -> Bound;
    fn get_bg_color(&self) -> Color;
    fn get_console(&mut self) -> &mut OffscreenConsole;

    fn get_mut_messages(&mut self) -> &mut Vec<String>;
    fn get_messages(&self) -> Vec<String>;
    fn get_max_messages(&self) -> usize;

    fn clear(&mut self) {
        let color = self.get_bg_color();
        let console = self.get_console();
        console.set_default_background(color);
        console.clear();
    }

    fn print_message(&mut self, x: i32, y: i32, alignment: tcod::TextAlignment, text: &str) {
        let console = self.get_console();
        console.print_ex(x, y, BackgroundFlag::Set, alignment, text);
    }

    fn buffer_message(&mut self, text: &str) {
        let max      = self.get_max_messages();
        let message  = String::from(text);
        let messages = self.get_mut_messages();

        messages.insert(0, message);
        messages.truncate(max);
    }

    fn flush_buffer(&mut self) {
        let max      = self.get_max_messages();
        let messages = self.get_mut_messages();

        for _ in 0..max {
            messages.insert(0, String::from(""));
        }
        messages.truncate(max);
    }
}

impl WindowComponent for TcodStatWindowComponent {
    fn get_bound(&self) -> Bound { self.bound }
    fn get_bg_color(&self) -> Color { self.bg_color }
    fn get_console(&mut self) -> &mut OffscreenConsole { &mut self.console }

    fn get_mut_messages(&mut self) -> &mut Vec<String> { &mut self.messages }
    fn get_messages(&self) -> Vec<String> { self.messages.clone() }
    fn get_max_messages(&self) -> usize { self.max_messages }
}

impl WindowComponent for TcodInputWindowComponent {
    fn get_bound(&self) -> Bound { self.bound }
    fn get_bg_color(&self) -> Color { self.bg_color }
    fn get_console(&mut self) -> &mut OffscreenConsole { &mut self.console }

    fn get_mut_messages(&mut self) -> &mut Vec<String> { &mut self.messages }
    fn get_messages(&self) -> Vec<String> { self.messages.clone() }
    fn get_max_messages(&self) -> usize { self.max_messages }
}

impl WindowComponent for TcodMessageWindowComponent {
    fn get_bound(&self) -> Bound { self.bound }
    fn get_bg_color(&self) -> Color { self.bg_color }
    fn get_console(&mut self) -> &mut OffscreenConsole { &mut self.console }

    fn get_mut_messages(&mut self) -> &mut Vec<String> { &mut self.messages }
    fn get_messages(&self) -> Vec<String> { self.messages.clone() }
    fn get_max_messages(&self) -> usize { self.max_messages }
}

impl WindowComponent for TcodMapWindowComponent {
    fn get_bound(&self) -> Bound { self.bound }
    fn get_bg_color(&self) -> Color { self.bg_color }
    fn get_console(&mut self) -> &mut OffscreenConsole { &mut self.console }

    fn get_mut_messages(&mut self) -> &mut Vec<String> { &mut self.messages }
    fn get_messages(&self) -> Vec<String> { self.messages.clone() }
    fn get_max_messages(&self) -> usize { self.max_messages }
}

pub trait RenderingComponent {
    fn before_render_new_frame(&mut self);
    fn render_object(&mut self, pos: Point, symbol: char);
    fn after_render_new_frame(&mut self);
    fn wait_for_keypress(&mut self) -> Key;
    fn attach_window(&mut self, window: &mut Box<dyn WindowComponent>);
}

pub struct TcodRenderingComponent {
    pub console: RootConsole
}

impl TcodRenderingComponent {
    pub fn new(b: Bound) -> Self {
        let con = RootConsole::initializer()
            .size(b.max.x + 1, b.max.y + 1)
            .title("Rogalik")
            .init();

        TcodRenderingComponent{console: con}
    }
}

impl RenderingComponent for TcodRenderingComponent {
    fn before_render_new_frame(&mut self) {
        self.console.clear();
    }

    fn render_object(&mut self, position: Point, symbol: char) {
        self.console.put_char(position.x as i32, position.y as i32, symbol, BackgroundFlag::Set);
    }

    fn after_render_new_frame(&mut self) {
        self.console.flush();
    }

    fn wait_for_keypress(&mut self) -> Key {
        self.console.wait_for_keypress(true)
    }

    fn attach_window(&mut self, window: &mut Box<dyn WindowComponent>) {
        window.clear();
//        window.print_message(0, 0, TextAlignment::Left, "Sup foo!");
//        window.print_message(0, 1, TextAlignment::Left, "Nothin fool!");
        let bound = window.get_bound();
        let messages = window.get_messages();
        messages.iter().enumerate()
         .for_each(|(i, e)| window.print_message(0, i as i32, tcod::TextAlignment::Left, e));

        let console = window.get_console();
        console::blit(&*console, (0, 0), (bound.max.x + 1, bound.max.y + 1), &mut self.console, (bound.min.x, bound.min.y), 1f32, 1f32);
    }
}
