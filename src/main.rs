// Copyright 2022-2022 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use global_hotkey::{
    hotkey::{Code, HotKey, Modifiers},
    GlobalHotKeyEvent, GlobalHotKeyManager, HotKeyState,
};

use rodio::cpal::traits::{DeviceTrait, HostTrait};
use rodio::OutputStream;
use std::io;
use std::io::BufReader;
use winit::event_loop::{ControlFlow, EventLoopBuilder};

fn main() {
    println!("デバイスを取得中です...\n");
    let device_names = get_devices();
    let mut input;
    loop {
        println!("\n使用するデバイスを番号で入力してください。");
        input = get_input().parse::<usize>().unwrap();

        if device_names.len() < input || input <= 0 {
            println!("値が不正です。");
            continue;
        }
        break;
    }
    println!("\nホットキーはCONTROL + SHIFT + D\n終了はウィンドウ内でCTRL + C");

    run(&device_names[input - 1]);
}

fn run(device: &str) {
    let event_loop = EventLoopBuilder::new().build().unwrap();
    let hotkeys_manager = GlobalHotKeyManager::new().unwrap();
    let hotkey = HotKey::new(Some(Modifiers::SHIFT | Modifiers::CONTROL), Code::KeyD);
    hotkeys_manager.register(hotkey).unwrap();

    let global_hotkey_channel = GlobalHotKeyEvent::receiver();

    event_loop
        .run(move |_event, event_loop| {
            event_loop.set_control_flow(ControlFlow::Poll);

            if let Ok(event) = global_hotkey_channel.try_recv() {
                if hotkey.id() == event.id && event.state == HotKeyState::Released {
                    hotkeys_manager.unregister(hotkey).unwrap();
                    play("audio/sample.wav", device); //スピーカー (2- Logitech G733 Gaming Headset)
                    hotkeys_manager.register(hotkey).unwrap();
                }
            }
        })
        .unwrap();
}

fn play(path: &str, device_name: &str) {
    let host = rodio::cpal::default_host();
    let device = host
        .output_devices()
        .unwrap()
        .find(|device| device.name().unwrap() == device_name)
        .unwrap();

    let (_stream, handle) = OutputStream::try_from_device(&device).unwrap();
    let sink = rodio::Sink::try_new(&handle).unwrap();

    let file = std::fs::File::open(path).unwrap();
    let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
    sink.append(source);

    sink.sleep_until_end();
}

fn get_devices() -> Vec<String> {
    let host = rodio::cpal::default_host();
    let devices = host.output_devices().unwrap();
    let mut device_array = vec![];
    let mut counter = 1;

    for device in devices {
        let dev: rodio::Device = device.into();
        let dev_name: String = dev.name().unwrap();
        println!("{counter}: {dev_name}");
        device_array.push(dev_name);
        counter += 1;
    }
    return device_array;
}

fn get_input() -> String {
    let mut word = String::new();
    io::stdin().read_line(&mut word).ok();
    return word.trim().to_string();
}
