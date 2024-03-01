use keymapping::{GERMAN_KEYMAP, US_KEYMAP};
use regex::Regex;
use tuning_systems::{Tone, TuningSystem};
use wasm_bindgen::prelude::*;

#[cfg(feature = "mini-alloc")]
#[global_allocator]
static ALLOC: mini_alloc::MiniAlloc = mini_alloc::MiniAlloc::INIT;

#[cfg(feature = "console_error_panic_hook")]
pub(crate) fn set_panic_hook() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen(start)]
pub(crate) fn main() {
    #[cfg(debug_assertions)]
    debug("main");
    #[cfg(feature = "console_error_panic_hook")]
    set_panic_hook();
}

#[wasm_bindgen]
extern "C" {
    #[cfg(debug_assertions)]
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    #[cfg(debug_assertions)]
    #[wasm_bindgen(js_namespace = console)]
    fn debug(s: &str);

    fn createTone(
        index: usize,
        frequency: f64,
        cents: f64,
        name: String,
        tuning_system: JsValue,
    ) -> JsValue;
}

#[wasm_bindgen]
pub fn get_tone(tuning: &str, index: usize) -> JsValue {
    #[cfg(debug_assertions)]
    debug("get_tone");
    let tuning: Result<TuningSystem, _> = tuning.parse();
    let tone: Tone = match tuning {
        Ok(tuning) => Tone::new(tuning, index),
        Err(_) => panic!("unknown tuning system"),
    };

    createTone(
        index,
        tone.frequency(),
        tone.cents(),
        tone.name,
        JsValue::NULL,
    )
}

#[wasm_bindgen]
pub fn set_octave_size(size: usize) {
    #[cfg(debug_assertions)]
    debug("set_octave_size");
    tuning_systems::set_octave_size(size)
}

#[wasm_bindgen]
pub fn set_step_size(size: usize) {
    #[cfg(debug_assertions)]
    debug("set_step_size");
    tuning_systems::set_step_size(size)
}

#[wasm_bindgen]
pub fn get_tuning_size(tuning: &str) -> usize {
    #[cfg(debug_assertions)]
    debug("get_tuning_size");
    let tuning: Result<TuningSystem, _> = tuning.parse();
    match tuning {
        Ok(tuning) => tuning.size(),
        Err(_) => panic!("unknown tuning system"),
    }
}

#[wasm_bindgen]
pub fn from_keymap(key: &str) -> i32 {
    #[cfg(debug_assertions)]
    debug("from_keymap");
    US_KEYMAP.get(key).unwrap_or(&-1).clone()
}

#[wasm_bindgen]
pub fn convert_notes(notes: Vec<JsValue>) -> String {
    format!(
        "L: 1/1 \n[{}]",
        notes
            .iter()
            .map(|note| {
                let note = note.as_string().unwrap();
                let re = Regex::new(r"([A-G])([#b]*)(N1|\d+)").unwrap();
                if let Some(cap) = re.captures(&note) {
                    let pitch = &cap[1];
                    let mut accidental = cap[2].to_string();
                    let octave_str = &cap[3];
                    accidental = accidental.replace("#", "^").replace("b", "_");
                    let mut formatted_octave = String::new();
                    if octave_str != "N1" {
                        let octave_number = octave_str.parse::<i32>().unwrap();
                        if octave_number == 4 {
                            formatted_octave = "".to_string();
                        } else if octave_number < 4 {
                            formatted_octave = ",".repeat((4 - octave_number) as usize);
                        } else if octave_number > 4 {
                            formatted_octave = "'".repeat((octave_number - 4) as usize);
                        }
                    } else {
                        formatted_octave = ",,,,,,".to_string();
                    }
                    return format!("{}{}{}", accidental, pitch, formatted_octave);
                }
                note
            })
            .collect::<Vec<String>>()
            .join("")
    )
}
