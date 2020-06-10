use amethyst::{
    StateData,
    GameData,
    ecs::prelude::*,
    SimpleState,
    SimpleTrans,
    StateEvent,
    Trans,
    input::{is_key_down},
    ui::{UiCreator, UiFinder, UiText},
    winit::VirtualKeyCode,
    audio::{AudioSink},
};

use serde::{Deserialize, Serialize};
use crate::game_field::{FieldSize};


//Prints Leader board list
//TO_DO: Make it not ugly
//Denis: I tak norm
const TEXT_4E: &str = "text_4_e";
const TEXT_4H: &str = "text_4_h";
const TEXT_6E: &str = "text_6_e";
const TEXT_6H: &str = "text_6_h";

#[derive(Default)]
pub struct LeaderState{
    ui_root: Option<Entity>,
    text_4_e: Option<Entity>,
    text_6_e: Option<Entity>,
    text_4_h: Option<Entity>,
    text_6_h: Option<Entity>,
    leader_board: LeaderBoard,
}   

impl SimpleState for LeaderState{
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>){
        self.ui_root = Some(data.world.exec(|mut creator: UiCreator| creator.create("ui/leader.ron", ())));
        let son_str = std::fs::read_to_string("leader.ron").expect("Cant read a leader.ron, LeaderState");
        self.leader_board = ron::de::from_str(&son_str).expect("Some trouble with leader.ron deserealization");
    }

    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>){
        if let Some(root_entity) = self.ui_root {
            data.world
                .delete_entity(root_entity)
                .expect("Failed to remove FieldChooseState");
        }

        self.ui_root = None;
        self.text_4_e = None;
        self.text_4_h = None;
        self.text_6_e = None;
        self.text_6_h = None;
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans{
        if self.text_4_e.is_none()
            || self.text_4_h.is_none()
            || self.text_6_e.is_none()
            || self.text_6_h.is_none(){
                data.world.exec(|ui_finder : UiFinder| {
                    self.text_4_e = ui_finder.find(TEXT_4E);
                    self.text_4_h = ui_finder.find(TEXT_4H);
                    self.text_6_e = ui_finder.find(TEXT_6E);
                    self.text_6_h = ui_finder.find(TEXT_6H);
                });
                let (b, mut c): (Entities, WriteStorage<UiText>) = data.world.system_data();
                for (entity, text) in (&b, &mut c).join(){
                    if Some(entity) == self.text_4_e{
                        text.text = format!("Field 4x4, Normal difuculty:\n1.{}: {}\n2.{}: {}\n3.{}: {}",
                            self.leader_board.easy_list.0[0].name, self.leader_board.easy_list.0[0].score,
                            self.leader_board.easy_list.0[1].name, self.leader_board.easy_list.0[1].score,
                            self.leader_board.easy_list.0[2].name, self.leader_board.easy_list.0[2].score,
                        );
                    }else if Some(entity) == self.text_4_h{
                        text.text = format!("Field 4x4, Hard difuculty:\n1.{}: {}\n2.{}: {}\n3.{}: {}",
                            self.leader_board.hard_list.0[0].name, self.leader_board.hard_list.0[0].score,
                            self.leader_board.hard_list.0[1].name, self.leader_board.hard_list.0[1].score,
                            self.leader_board.hard_list.0[2].name, self.leader_board.hard_list.0[2].score,
                        );
                    }else if Some(entity) == self.text_6_e{
                        text.text = format!("Field 6x6, Normal difuculty:\n1.{}: {}\n2.{}: {}\n3.{}: {}",
                            self.leader_board.easy_list.1[0].name, self.leader_board.easy_list.1[0].score,
                            self.leader_board.easy_list.1[1].name, self.leader_board.easy_list.1[1].score,
                            self.leader_board.easy_list.1[2].name, self.leader_board.easy_list.1[2].score,
                        );
                    }else if Some(entity) == self.text_6_h{
                        text.text = format!("Field 6x6, Hard difuculty:\n1.{}: {}\n2.{}: {}\n3.{}: {}",
                            self.leader_board.hard_list.1[0].name, self.leader_board.hard_list.1[0].score,
                            self.leader_board.hard_list.1[1].name, self.leader_board.hard_list.1[1].score,
                            self.leader_board.hard_list.1[2].name, self.leader_board.hard_list.1[2].score,
                        );
                    }
                }
            }
        Trans::None
    }

    fn handle_event(&mut self, data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans{
        match event {
            StateEvent::Window(event) => {
                if is_key_down(&event, VirtualKeyCode::Escape) {
                    println!("[Trans::Pop] pressed esc in LeaderBoard!");
                    return Trans::Pop;
                } 
                else if is_key_down(&event, VirtualKeyCode::PageDown) {
                    let mut sink = data.world.write_resource::<AudioSink>();
                    let volume = sink.volume();
                    if volume < 0.01 {
                        sink.set_volume(0.0);
                    } else {
                        sink.set_volume(volume -0.01);
                    }
                    println!("make music tishe: {}", volume -0.01);
                    return Trans::None;
                }else if is_key_down(&event, VirtualKeyCode::PageUp) {
                    let mut sink = data.world.write_resource::<AudioSink>();
                    let volume = sink.volume();
                    if volume > 0.99 {
                        sink.set_volume(1.0);
                    } else {
                        sink.set_volume(volume + 0.01);
                    }
                    println!("make music louder: {}", volume +0.01);
                    return Trans::None;
                }
            }
            _ => (),
        }
        Trans::None
    }
}


#[derive( Default, Deserialize, Serialize, Debug)]
pub struct Leader{
    pub name: String,
    pub score: u32,
    pub hard: bool,
    pub size: FieldSize,
}

#[derive( Default, Deserialize, Serialize, Debug)]
pub struct LeaderBoard{
    pub hard_list : (Vec<Leader>,Vec<Leader>),
    pub easy_list : (Vec<Leader>,Vec<Leader>),
}

impl LeaderBoard{
    pub fn add_leader(&mut self, leader : Leader){
        println!("Leader:{:?}", leader);
        let vector: &mut Vec<Leader>;
        match leader.hard {
            false => {
                match leader.size{
                    FieldSize::Four => vector = &mut self.easy_list.0,
                    FieldSize::Six => vector = &mut self.easy_list.1,
                    FieldSize::Empty => unreachable!(),
                }
            }
            true => {
                match leader.size{
                    FieldSize::Four => vector = &mut self.hard_list.0,
                    FieldSize::Six => vector = &mut self.hard_list.1,
                    FieldSize::Empty => unreachable!(),
                }
            }
        }
        vector.push(leader);
        vector.sort_by(|a , b| b.score.cmp(&a.score));
        println!("{:?}", vector);
        vector.pop();
        println!("{:?}", vector);
    }
}




