use macroquad::audio::*;

/// Plays a sound effect on loop with a given volume.
pub fn play_sound_looped(sound: Sound, volume: f32) {
    stop_sound(sound);
    play_sound(
        sound,
        PlaySoundParams {
            looped: true,
            volume,
        },
    );
}
