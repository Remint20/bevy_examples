// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//
// System実行順序の確認
//
// 現在: Failed to acquire next swap chain texture!のバグが発生している。
//

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    render::settings::{Backends, WgpuSettings},
};

fn main() {
    App::new()
        .insert_resource(WgpuSettings {
            backends: Some(Backends::VULKAN),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_startup_system(startup_system)
        .add_startup_stage(
            "startup_stage",
            SystemStage::single(normal_startup_stage_system),
        )
        .add_startup_stage_before(
            "startup_stage",
            "before",
            SystemStage::single(before_startup_stage_system),
        )
        .add_startup_stage_after(
            "startup_stage",
            "after",
            SystemStage::single(after_startup_stage_system),
        )
        .add_stage("stage", SystemStage::single(normal_stage_system))
        .add_stage_before("stage", "before", SystemStage::single(before_stage_system))
        .add_stage_after("stage", "after", SystemStage::single(after_stage_system))
        .add_system(normal_system)
        .run();
}

fn startup_system() {
    println!("startup system run");
}

fn normal_system() {
    println!("normal system run")
}

fn normal_startup_stage_system() {
    println!("normal startup stage system run");
}

fn before_startup_stage_system() {
    println!("before startup statge system run");
}

fn after_startup_stage_system() {
    println!("after startup statge system run");
}

fn normal_stage_system() {
    println!("normal stage system run");
}

fn before_stage_system() {
    println!("before statge system run");
}

fn after_stage_system() {
    println!("after statge system run");
}
