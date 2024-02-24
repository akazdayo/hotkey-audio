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
    let hotkey_list = [
        Code::Digit1,
        Code::Digit2,
        Code::Digit3,
        Code::Digit4,
        Code::Digit5,
        Code::Digit6,
        Code::Digit7,
        Code::Digit8,
        Code::Digit9,
        Code::Digit0,
    ];
    let audio_path = (0..10)
        .map(|i| format!("audio/{}.wav", i))
        .collect::<Vec<_>>();
    let mut hotkeys = vec![];

    let event_loop = EventLoopBuilder::new().build().unwrap();
    let hotkeys_manager = GlobalHotKeyManager::new().unwrap();
    for i in 0..hotkey_list.len() {
        hotkeys.push(HotKey::new(
            Some(Modifiers::SHIFT | Modifiers::CONTROL),
            hotkey_list[i],
        ));
        if let Err(err) = hotkeys_manager.register(hotkeys[i]) {
            eprintln!("Failed to register hotkey: {:?}", err);
        }
    }

    let global_hotkey_channel = GlobalHotKeyEvent::receiver();

    event_loop
        .run(move |_event, event_loop| {
            event_loop.set_control_flow(ControlFlow::Poll);

            if let Ok(event) = global_hotkey_channel.try_recv() {
                for i in 0..10 {
                    if hotkeys[i].id() == event.id && event.state == HotKeyState::Released {
                        hotkeys_manager.unregister(hotkeys[i]).unwrap();
                        play(audio_path[i].as_str(), device);
                        hotkeys_manager.register(hotkeys[i]).unwrap();
                    }
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
    println!("{}", path);

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
