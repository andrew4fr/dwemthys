macro_rules! window_component_getters {
    () => {
        fn get_console(&mut self)      -> &mut OffscreenConsole          { &mut self.console     }
        fn get_bound(&self)           ->      Bound            { self.bound           }
        fn get_bg_color(&self)         ->      Color            { self.bg_color }
        fn get_mut_messages(&mut self) -> &mut Vec<String> { &mut self.messages    }
        fn get_max_messages(&self)     ->      usize             { self.max_messages     }
        fn get_messages(&self)         ->      Vec<String> { self.messages.clone() }
    }
}

macro_rules! window_component_def {
    ($name:ident) => {
        pub struct $name {
            pub console:          OffscreenConsole,
            pub bg_color: Color,
            bound:               Bound,
            messages:             Vec<String>,
            max_messages:         usize,
        }
    }
}

macro_rules! window_component_init {
    ($name:ident, $color:expr, $max_messages:expr) => {
        pub fn new(b: Bound) -> $name {
            let h = b.max.y - b.min.y + 1;
            let w  = b.max.x - b.min.x + 1;
            let console = OffscreenConsole::new(w, h);

            $name {
                console:          console,
                bg_color: $color,
                bound:           b,
                messages:         vec!(),
                max_messages:     $max_messages
            }
        }
    }
}
