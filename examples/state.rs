// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::{
    core::FixedTimestep,
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    render::options::{Backends, WgpuOptions},
};

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    A,
    B,
}

fn main() {
    App::new()
        .insert_resource(WgpuOptions {
            backends: Some(Backends::VULKAN),
            ..Default::default()
        })
        .add_state(GameState::A)
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(LogDiagnosticsPlugin::default())
        // beforeとafterはLabelでバインドされる
        .add_system_set(
            SystemSet::on_enter(GameState::A)
                .label("A")
                .with_system(a_system),
        )
        .add_system_set(
            SystemSet::on_enter(GameState::A)
                .before("A")
                .with_system(a_before_system),
        )
        .add_system_set(
            SystemSet::on_enter(GameState::A)
                .after("A")
                .with_system(a_after_system),
        )
        .add_system_set(
            // with_run_criteriaを付けると on_enter, on_update関係無くタイムステップごとに動作する
            SystemSet::on_exit(GameState::B)
                .with_run_criteria(FixedTimestep::step(1.0))
                .with_system(b_system),
        )
        .run();
}

fn a_system() {
    println!("A run");
}

fn a_before_system() {
    println!("before A run");
}

fn a_after_system(mut state: ResMut<State<GameState>>) {
    println!("after A run");
    state.set(GameState::B).unwrap();
}

fn b_system(mut state: ResMut<State<GameState>>) {
    println!("B run");
    state.set(GameState::A).unwrap();
}
