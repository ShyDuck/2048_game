use amethyst::{
    assets::{Loader},
    audio::{ FlacFormat, SourceHandle},
    ecs::{World, WorldExt},
};

use std::{iter::Cycle, vec::IntoIter};

// Repeate 3 song forever
//TO_DO : controling music volume 
//Make it possible to turn it off

const AUDIO_MUSIC: &[&str] = &[
    "audio/track_1.mp3",
    "audio/track_2.mp3",
    "audio/track_3.mp3",
    ];
pub struct Music {
    pub music: Cycle<IntoIter<SourceHandle>>,
}

fn load_audio_track(loader: &Loader, world: &World, file: &str) -> SourceHandle {
    loader.load(file, FlacFormat, (), &world.read_resource())
}


pub fn initialise_audio(world: &mut World) {
    let music = {
        let loader = world.read_resource::<Loader>();


        let music = AUDIO_MUSIC
            .iter()
            .map(|file| load_audio_track(&loader, &world, file))
            .collect::<Vec<_>>()
            .into_iter()
            .cycle();
        let music = Music { music };
        music
    };

    world.insert(music);
}





