use ggez::audio::Source;
use ggez::{Context};
use crate::constant::SUPPORTED_SOUND_FILE_EXT;


pub struct MusicSound {
    pub ingame_music: Option<Source>,
    pub victory_music: Option<Source>
}

impl MusicSound {
    pub fn new() -> Self {
        MusicSound {
            ingame_music: None,
            victory_music: None
        }
    }
}

pub struct SoundLibrary {
    pub music_sound: MusicSound
}

impl SoundLibrary {
    pub fn new() -> Self {
        SoundLibrary { music_sound: MusicSound::new() }
    }

    pub fn clear(&mut self) {
        self.music_sound.ingame_music = None;
        self.music_sound.victory_music = None;
    }

    pub fn load_music(&mut self, context: &mut Context, level: u8) {
        if !SUPPORTED_SOUND_FILE_EXT.iter().any(|&s| {
            if let Ok(source) = Source::new(context, format!("/sounds/musics/lvl_{}_ingame_music.{}", level, s)) {
                self.music_sound.ingame_music = Some(source);
                return true;
            }
            false
        }) { panic!(); }

        if !SUPPORTED_SOUND_FILE_EXT.iter().any(|&s| {
            if let Ok(source) = Source::new(context, format!("/sounds/musics/lvl_{}_victory_music.{}", level, s)) {
                self.music_sound.victory_music = Some(source);
                return true;
            }
            false
        }) { panic!(); }
    }
}
