use bevy::{core::FixedTimestep, prelude::*};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum GlobalState {
    FirstState,
    SecondState,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Init state
        .add_state(GlobalState::FirstState)
        // First state systems
        .add_system_set(
            SystemSet::on_enter(GlobalState::FirstState)
                .with_system(first_state::enter_system)
                .with_run_criteria(FixedTimestep::step(1.0)),
        )
        .add_system_set(
            SystemSet::on_update(GlobalState::FirstState)
                .with_system(first_state::update_system)
                .with_run_criteria(FixedTimestep::step(1.0)),
        )
        // Second state systems
        .add_system_set(
            SystemSet::on_enter(GlobalState::SecondState)
                .with_system(second_state::enter_system)
                .with_run_criteria(FixedTimestep::step(1.0)),
        )
        .add_system_set(
            SystemSet::on_update(GlobalState::SecondState)
                .with_system(second_state::update_system)
                .with_run_criteria(FixedTimestep::step(1.0)),
        )
        .run()
}

mod first_state {
    pub fn enter_system() {
        println!("First state: Enter");
    }

    pub fn update_system() {
        println!("First state: Update");
    }
}

mod second_state {
    pub fn enter_system() {
        println!("Second state: Enter");
    }

    pub fn update_system() {
        println!("Second state: Update");
    }
}