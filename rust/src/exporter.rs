use std::fs::File;
use std::io::Write;

use postflop_solver::{PostFlopGame, card_from_str, holes_to_strings};


pub struct Exporter<'a> {
    pub game: &'a mut PostFlopGame,
    pub cards: Vec<&'static str>,
    file: File,
}

impl<'a> Exporter<'a> {
    pub fn new(game: &'a mut PostFlopGame, cards: Vec<&'static str>) -> Result<Self, std::io::Error> {
        let file = File::create("result.txt")?;
        Ok(Exporter {
            game,
            cards,
            file,
        })
    }

    fn print<T: std::fmt::Debug>(&mut self, arg: T, indent: usize) {
        let indent_str: String = (0..indent).map(|_| '\t').collect();
        let to_print = format!("{:?}", arg);

        if let Err(err) = self.file.write_all(format!("{}{}\n", indent_str, to_print).as_bytes()) {
            eprintln!("Error writing to file: {}", err);
        }
    }

    pub fn export_init(&mut self) {
        let oop_cards = self.game.private_cards(0);
        let ip_cards = self.game.private_cards(1);

        let oop_cards_str = holes_to_strings(oop_cards).unwrap();
        let ip_cards_str = holes_to_strings(ip_cards).unwrap();

        self.print(oop_cards_str, 0);
        self.print(ip_cards_str, 0);
    }

    pub fn export(&mut self, indent: usize) {
        if self.game.is_terminal_node() {
            return;
        }

        self.print(self.game.current_player(), indent);
        if self.game.is_chance_node() {
            self.export_chance(indent);
        } else {
            self.export_player(indent);
        }
    }

    fn export_chance(&mut self, indent: usize) {
        let history = self.game.history().to_owned();

        for card in self.cards.to_owned() {
            let card_index = card_from_str(card).unwrap();
            if self.game.possible_cards() & (1 << card_index) == 0 {
                // Card not available
                continue;
            }

            self.print(card, indent);
            self.export_child(card_index as usize, indent);
            self.game.apply_history(&history);
        }
    }

    fn export_player(&mut self, indent: usize) {
        let history = self.game.history().to_owned();
        let current_player = self.game.current_player();

        self.game.cache_normalized_weights();
        self.print(self.game.strategy(), indent);
        self.print(self.game.expected_values_detail(current_player), indent);

        for (index, action) in self.game.available_actions().iter().enumerate() {
            self.print(format!("{:?}", action), indent);
            self.export_child(index, indent);
            self.game.apply_history(&history);
        }
    }

    fn export_child(&mut self, action: usize, indent: usize) {
        self.game.play(action);
        self.export(indent + 1);
    }
}
