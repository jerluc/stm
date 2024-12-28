use std::{
    fs::{self, File},
    io::{BufRead, BufReader, Read},
    path::PathBuf,
    str::FromStr,
};

use serde::{Deserialize, Serialize};
use steamlocate::SteamDir;

/*
|NUL|shortcuts|NUL|
 |NUL|0|NUL|
  |STX|appid|NUL||NUL||NUL||NUL||NUL|
  |SOH|AppName|NUL|APP NAME WITHOUT QUOTES|NUL|
  |SOH|Exe|NUL|"PATH TO EXE"|NUL|
  |SOH|StartDir|NUL|"DIRECTORY TO START IN"|NUL|
  |SOH|icon|NUL|"PATH TO ICON"|NUL|
  |SOH|ShortcutPath|NUL||NUL|
  |SOH|LaunchOptions|NUL||NUL|
  |STX|IsHidden|NUL||NUL||NUL||NUL||NUL|
  |STX|AllowDesktopConfig|NUL||SOH||NUL||NUL||NUL|
  |STX|AllowOverlay|NUL||SOH||NUL||NUL||NUL|
  |STX|OpenVR|NUL||NUL||NUL||NUL||NUL|
  |STX|Devkit|NUL||NUL||NUL||NUL||NUL|
  |SOH|DevkitGameID|NUL||NUL|
  |STX|LastPlayTime|NUL||NUL||NUL||NUL||NUL|
  |NUL|tags|NUL|
   |SOH|0|NUL|favorite|NUL|
  |BS|
 |BS|
 |NUL|1|NUL|
  ...
 |BS|
|BS|


*/

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct Shortcut {
    #[serde(rename = "AppID")]
    app_id: u32,
    app_name: String,
    exe: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct ShortcutsFile {
    shortcuts: Vec<Shortcut>,
}

const USER_ID: u32 = 86600240;

fn read_shortcuts(path: PathBuf) -> keyvalues_serde::Result<ShortcutsFile> {
    println!("Shortcuts path = {}", path.display());
    let f = File::open(path)?;
    let mut s: Vec<u8> = Vec::new();
    let mut reader = BufReader::new(f);
    reader.read_to_end(&mut s)?;
    // let contents: &[u8] = s.as_slice();
    let st = String::from_utf8(s).expect("Non-UTF-8");
    println!("STRING = {}", st);
    keyvalues_serde::from_str(st.to_string().as_str())
}

fn main() {
    if let Ok(steam) = SteamDir::locate() {
        println!("Steam dir = {}", steam.path().display());

        let shortcuts_path = steam
            .path()
            .join("userdata")
            .join(format!("{}", USER_ID))
            .join("config")
            .join("shortcuts.vdf");

        let shortcuts = read_shortcuts(shortcuts_path).expect("Failed to read shortcuts");
        println!("{}", shortcuts.shortcuts[0].app_name);
    }
}
