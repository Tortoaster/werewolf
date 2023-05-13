use crate::{
    param::Parameter,
    player::{HouseNr, Player},
    role::Role,
    state::State,
};

pub trait Action {
    type Params: Parameter;

    fn from_params(params: Self::Params) -> Self;

    fn perform(self, actor: &mut Player, state: &mut State);
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Vision {
    target: HouseNr,
}

impl Action for Vision {
    type Params = HouseNr;

    fn from_params(target: Self::Params) -> Self {
        Vision { target }
    }

    fn perform(self, actor: &mut Player, state: &mut State) {
        actor.message(&format!(
            "You see {}.",
            state
                .get_players_at(self.target)
                .map(|p| format!("{}", p.role()))
                .collect::<Vec<_>>()
                .join(", ")
        ))
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct CubKill {
    target: HouseNr,
    role: Role,
}

impl Action for CubKill {
    type Params = (HouseNr, Role);

    fn from_params(params: Self::Params) -> Self {
        CubKill {
            target: params.0,
            role: params.1,
        }
    }

    fn perform(self, actor: &mut Player, state: &mut State) {
        for player in state.get_players_at_mut(self.target) {
            if player.role() == self.role {
                player.kill();
                actor.set_role(Role::Werewolf);
            }
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct SleepAt {
    target: HouseNr,
}

impl Action for SleepAt {
    type Params = HouseNr;

    fn from_params(target: Self::Params) -> Self {
        SleepAt { target }
    }

    fn perform(self, actor: &mut Player, _: &mut State) {
        actor.go_to(self.target);
    }
}
