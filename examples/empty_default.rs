// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//
// 空のWindowを表示する
//
use bevy::prelude::*;

fn main() {
    App::new().add_plugins(DefaultPlugins).run();
}
