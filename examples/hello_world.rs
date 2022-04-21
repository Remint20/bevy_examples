// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//
// consoleにHello Worldを出力する
//
// ゲーム画面はでない
//
use bevy::prelude::*;

fn main() {
    App::new().add_system(hello_world_system).run();
}

fn hello_world_system() {
    println!("hello world");
}
