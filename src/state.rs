use crate::player::{HouseNr, Player};

#[derive(Debug)]
pub struct State {
    pub players: Vec<Player>,
}

impl State {
    pub fn get_players_at(&self, nr: HouseNr) -> impl Iterator<Item = &Player> {
        self.players.iter().filter(move |p| p.now_at() == nr)
    }

    pub fn get_players_at_mut(&mut self, nr: HouseNr) -> impl Iterator<Item = &mut Player> {
        self.players.iter_mut().filter(move |p| p.now_at() == nr)
    }

    pub fn end_day(&mut self) {
        for player in &mut self.players {
            player.go_home();
        }
    }
}
