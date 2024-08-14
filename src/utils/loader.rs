use aseprite::SpritesheetData;
use serde_json::to_string_pretty;
use std::{io::Write, process::Command};

use crate::prelude::*;

pub fn load_action_data(actor: &str) -> HashMap<String, Action> {
    let file = get_file(&format!("data/char/{}/data.json", actor));
    let mut hashmap: HashMap<String, Action> = HashMap::new();

    match file {
        Some(data) => {
            let character: ActorData = serde_json::from_slice(data).unwrap();
            for action in &character.actions {
                hashmap.insert(action.name.clone(), action.clone());
            }
        }
        None => eprintln!("ERROR -> load action data: could not find file"),
    }

    hashmap
}

pub fn load_animation_data(actor: &str) -> HashMap<String, Vec<Keyframe>> {
    let file = get_file(&format!("data/char/{}/{}.json", actor, actor));
    let mut hashmap: HashMap<String, Vec<Keyframe>> = HashMap::new();

    match file {
        Some(data) => {
            let sprite: SpritesheetData = serde_json::from_slice(data).unwrap();
            for tag in sprite.meta.frame_tags.unwrap() {
                let mut keyframes = Vec::new();
                for i in tag.from..=tag.to {
                    let frame = sprite.frames[i as usize].frame;
                    let duration = sprite.frames[i as usize].duration / 16;
                    keyframes.push(Keyframe {
                        x: frame.x as f32,
                        y: frame.y as f32,
                        w: frame.w as f32,
                        h: frame.h as f32,
                        duration,
                    });
                }
                hashmap.insert(tag.name.clone(), keyframes);
            }
        }
        None => eprintln!("ERROR -> load animation data: could not find file"),
    }

    hashmap
}

pub fn update_action_durations(actor: &str) {
    let path = format!("assets/data/char/{}/data.json", actor);
    let file = std::fs::File::options()
        .read(true)
        .open(path.clone())
        .unwrap();

    let mut data: ActorData = serde_json::from_reader(&file).unwrap();
    let anim = load_animation_data(actor);

    println!("Updating {} action data...", actor);
    for action in &mut data.actions {
        let total = anim
            .get(&action.name)
            .unwrap()
            .iter()
            .fold(0, |a, b| a + b.duration);

        action.total = total;

        println!("{} -> {}: {}", actor, action.name, total);
    }

    let mut file = std::fs::File::create(path).unwrap();
    file.write_all(to_string_pretty(&data).unwrap().as_bytes())
        .unwrap();
}

// aseprite -b test.aseprite --sheet test.png --format json-array --data test.json --ignore-empty --sheet-pack --list-tags --filename-format '{title} #{tag} {frame}.{extension}'
pub fn update_aseprite_data(actor: &str) {
    println!("Updating {} animation data and sprite sheet...", actor);
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args([
                "/c",
                "cd",
                &format!("assets/data/char/{}", actor),
                "&",
                "aseprite",
                "-b",
                "ken.aseprite",
                "--sheet",
                &format!("{}.png", actor),
                "--format",
                "json-array",
                "--data",
                &format!("{}.json", actor),
                "--ignore-empty",
                "--split-tags",
                // "--sheet-pack",
                "--list-tags",
                "--filename-format",
                "'{title} #{tag} {frame}.{extension}'",
            ])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .output()
            .expect("failed to execute process")
    };
}
