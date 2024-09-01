use aseprite::SpritesheetData;
use serde_json::to_string_pretty;
use std::{io::Write, process::Command};

use crate::prelude::*;

pub fn load_character_data(actor: &str) -> CharacterData {
    let file = get_file(&format!("data/char/{}/data.json", actor));
    let mut char = CharacterData::default();
    match file {
        Some(data) => {
            char = serde_json::from_slice(data).unwrap();
        }
        None => eprintln!("ERROR -> load character data: could not find file"),
    }
    char
}

pub fn load_action_data(actor: &str) -> IndexMap<String, Action> {
    let file = get_file(&format!("data/char/{}/data.json", actor));
    let mut hashmap: IndexMap<String, Action> = IndexMap::new();

    match file {
        Some(data) => {
            let character: Actions = serde_json::from_slice(data).unwrap();
            for action in &character.actions {
                hashmap.insert(action.name.clone(), action.clone());
            }
        }
        None => eprintln!("ERROR -> load action data: could not find file"),
    }

    hashmap
}

pub fn load_animation_data(actor: &str) -> IndexMap<String, Vec<Keyframe>> {
    let file = get_file(&format!("data/char/{}/{}.json", actor, actor));
    let mut hashmap: IndexMap<String, Vec<Keyframe>> = IndexMap::new();

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

    let mut data: CharacterFile = serde_json::from_reader(&file).unwrap();
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

pub fn handle_arguments() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() > 1 && args[1] == "--data" {
        update_action_durations("ken");
        std::process::exit(0);
    }

    if args.len() > 1 && args[1] == "--ase" {
        update_aseprite_data("ken");
        std::process::exit(0);
    }

    if args.len() > 1 && args[1] == "--update" {
        update_aseprite_data("ken");
        update_action_durations("ken");
        std::process::exit(0);
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterFile {
    pub name: String,
    pub max_health: i32,
    pub forward_walk: i32,
    pub backward_walk: i32,
    pub jump_velocity: i32,
    pub jump_deceleration: i32,
    pub jump_forward: i32,
    pub jump_backward: i32,
    pub origin: Vec2,
    pub pushbox: Boxes,
    pub actions: Vec<Action>,
}

pub fn save_action_data(action: &ActionData, character: &CharacterData) {
    let actor = &character.name;
    let path = format!("assets/data/char/{}/data.json", actor);
    let file = std::fs::File::options()
        .read(true)
        .write(true)
        .open(path.clone())
        .unwrap();
    let mut data: CharacterFile = serde_json::from_reader(&file).unwrap();
    data.actions = action.to_vec();

    let mut file = std::fs::File::create(path).unwrap();
    file.write_all(to_string_pretty(&data).unwrap().as_bytes())
        .unwrap();
}
