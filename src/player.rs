use crate::role::Role;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct HouseNr(usize);

#[derive(Debug)]
pub struct Player {
    lives_at: HouseNr,
    now_at: HouseNr,
    role: Role,
    alive: bool,
}

impl Player {
    pub fn message(&self, msg: &str) {
        todo!()
    }

    pub fn set_role(&mut self, role: Role) {
        self.role = role;
    }

    pub fn role(&self) -> Role {
        self.role
    }

    pub fn kill(&mut self) {
        self.alive = false;
    }

    pub fn is_alive(&self) -> bool {
        self.alive
    }

    pub fn go_home(&mut self) {
        self.now_at = self.lives_at;
    }

    pub fn go_to(&mut self, nr: HouseNr) {
        self.now_at = nr;
    }

    pub fn now_at(&self) -> HouseNr {
        self.now_at
    }
}
