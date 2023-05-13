use strum::IntoEnumIterator;

use crate::{
    form::{Form, Select},
    player::HouseNr,
    role::Role,
    state::State,
};

pub trait Parameter {
    fn create_form(state: &State) -> impl Form<Output = Self>;
}

impl Parameter for HouseNr {
    fn create_form(state: &State) -> impl Form<Output = Self> {
        let options = state
            .players
            .iter()
            .filter_map(|p| p.is_alive().then_some(p.now_at()))
            .collect();

        Select::from(options)
    }
}

impl Parameter for Role {
    fn create_form(_: &State) -> impl Form<Output = Self> {
        let options = Role::iter().collect();

        Select::from(options)
    }
}

impl<P: Parameter, Q: Parameter> Parameter for (P, Q) {
    fn create_form(state: &State) -> impl Form<Output = Self> {
        P::create_form(state).and(Q::create_form(state))
    }
}
