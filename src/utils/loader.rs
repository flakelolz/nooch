use std::process::Command;

// NOTE: aseprite -b test.aseprite --sheet test.png --format json-array --data test.json --ignore-empty --sheet-pack --list-tags --filename-format '{title} #{tag} {frame}.{extension}'

pub fn update_aseprite_data() {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args([
                "aseprite",
                "-b",
                "ken.aseprite",
                "--sheet",
                "ken.png",
                "--format",
                "json-array",
                "--data",
                "ken.json",
                "--ignore-empty",
                // "--sheet-pack",
                "--list-tags",
                "--filename-format",
                "'{title} #{tag} {frame}.{extension}'",
            ])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .args([
                "aseprite",
                "-b",
                "ken.aseprite",
                "--sheet",
                "ken.png",
                "--format",
                "json-array",
                "--data",
                "ken.json",
                "--ignore-empty",
                // "--sheet-pack",
                "--list-tags",
                "--filename-format",
                "'{title} #{tag} {frame}.{extension}'",
            ])
            .output()
            .expect("failed to execute process")
    };
}
