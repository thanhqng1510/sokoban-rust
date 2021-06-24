use ggez::audio::Source;
use ggez::{Context, GameError};


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

    pub fn load_music(&mut self, context: &mut Context, level: u8) {
        let ext = vec![ ".wav", ".mp3", ".ogg", ".flac" ];

        if !ext.iter().any(|&s| {
            if let Ok(source) = Source::new(context, format!("/sounds/musics/ingame_music_{}{}", level, s)) {
                self.music_sound.ingame_music = Some(source);
                return true;
            }
            false
        }) { panic!(); }

        if !ext.iter().any(|&s| {
            if let Ok(source) = Source::new(context, format!("/sounds/musics/victory_music_{}{}", level, s)) {
                self.music_sound.victory_music = Some(source);
                return true;
            }
            false
        }) { panic!(); }
    }
}
