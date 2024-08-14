use aseprite::SpritesheetData;
use std::collections::HashMap;
use std::process::Command;

use crate::prelude::*;

pub fn load_animation_data(actor: &str) -> HashMap<String, Vec<Keyframe>> {
    let file = get_file(&format!("data/char/{}/{}.json", actor, actor));
    let mut hashmap: HashMap<String, Vec<Keyframe>> = HashMap::new();

    match file {
        Some(data) => {
            let sprite: SpritesheetData = serde_json::from_slice(data).unwrap();
            for tag in sprite.meta.frame_tags.unwrap() {
                let mut keyframes = Vec::new();
                for i in tag.from..tag.to {
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

// aseprite -b test.aseprite --sheet test.png --format json-array --data test.json --ignore-empty --sheet-pack --list-tags --filename-format '{title} #{tag} {frame}.{extension}'

pub fn update_aseprite_data(actor: &str) {
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
