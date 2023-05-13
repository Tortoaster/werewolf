use strum_macros::{Display, EnumIter};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Display, EnumIter)]
pub enum Role {
    Villager,
    Hustler,
    Seer,
    Werewolf,
    Cub,
}
