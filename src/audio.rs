use ggez::{audio, audio::SoundSource, Context};
use specs::{World, WorldExt};
use std::collections::HashMap;

#[derive(Default)]
pub struct AudioStore {
    pub sounds: HashMap<String, audio::Source>,
}

impl AudioStore {
    pub fn play(&mut self, sound: &str) {
        let _ = self
            .sounds
            .get_mut(sound)
            .expect("Expected sound")
            .play_detached();
    }
}

pub fn initialize_sounds(world: &mut World, context: &mut Context) {
    let mut audio_store = world.write_resource::<AudioStore>();
    let sounds = ["correct", "incorrect", "wall"];

    for sound in sounds.iter() {
        let sound_name = sound.to_string();
        let sound_path = format!("/sounds/{}.wav", sound);
        let source = audio::Source::new(context, sound_path).expect("Expected sound");

        audio_store.sounds.insert(sound_name, source);
    }
}
