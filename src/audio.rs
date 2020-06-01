use amethyst::{
    assets::{Loader},
    audio::{AudioSink, FlacFormat, SourceHandle},
    ecs::{World, WorldExt},
};

use std::{iter::Cycle, vec::IntoIter};
const AUDIO_MUSIC: &[&str] = &[
    "audio/What You Know.flac",
    "audio/Undercover Martyn.flac",
    "audio/I Can Talk.flac",
    ];


pub struct Music {
    pub music: Cycle<IntoIter<SourceHandle>>,
}

/// Loads an ogg audio track.
fn load_audio_track(loader: &Loader, world: &World, file: &str) -> SourceHandle {
    loader.load(file, FlacFormat, (), &world.read_resource())
}

/// Initialise audio in the world. This includes the background track and the
/// sound effects.
pub fn initialise_audio(world: &mut World) {
    let music = {
        let loader = world.read_resource::<Loader>();

        let mut sink = world.write_resource::<AudioSink>();
        sink.set_volume(0.1); // Music is a bit loud, reduce the volume.

        let music = AUDIO_MUSIC
            .iter()
            .map(|file| load_audio_track(&loader, &world, file))
            .collect::<Vec<_>>()
            .into_iter()
            .cycle();
        let music = Music { music };
        music
    };

    // Add sound effects to the world. We have to do this in another scope because
    // world won't let us insert new resources as long as `Loader` is borrowed.
    world.insert(music);
}





