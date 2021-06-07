use ggez::audio::Source;


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
}
