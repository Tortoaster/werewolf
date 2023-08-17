use std::marker::PhantomData;

use crate::{
    action::{Action, CubKill, SleepAt, Vision},
    form::Form,
    param::Parameter,
    player::Player,
    state::State,
};

pub type Standard = (SleepAt, Vision, CubKill);

#[derive(Debug)]
pub struct NightOrder<T = Standard> {
    phase: usize,
    phantom: PhantomData<T>,
}

trait ParallelActions {
    fn forms(player: &Player, state: &State) -> Vec<impl Form>;
}

impl<A: Action> ParallelActions for A {
    fn forms(player: &Player, state: &State) -> Vec<impl Form> {
        if A::performable_by(player, state) {
            vec![A::Params::create_form(state)]
        } else {
            Vec::new()
        }
    }
}

impl<A: Action, B: Action> ParallelActions for (A, B) {
    fn forms(player: &Player, state: &State) -> Vec<impl Form> {
        let mut forms = Vec::new();

        if A::performable_by(player, state) {
            forms.push(A::Params::create_form(state));
        }
        if B::performable_by(player, state) {
            forms.push(B::Params::create_form(state));
        }

        forms
    }
}

trait SequentialActions {
    fn forms_at_phase(phase: usize, player: &Player, state: &State) -> Vec<impl Form>;
}

impl<T: ParallelActions, U: ParallelActions, V: ParallelActions> SequentialActions for (T, U, V) {
    fn forms_at_phase(phase: usize, player: &Player, state: &State) -> Vec<impl Form> {
        match phase {
            0 => T::forms(player, state),
            1 => U::forms(player, state),
            2 => V::forms(player, state),
            _ => None,
        }
    }
}
