
use amethyst::{
    SimpleState,
    StateData,
    GameData,
    SimpleTrans,
    Trans,
    StateEvent,
    input::{is_close_requested, is_key_down},
    winit::VirtualKeyCode,
};
use std::fmt::{Debug};
use crate::states::exit;
use serde::{Deserialize, Serialize};
use std::fs;
use ron;

#[derive(Deserialize, Serialize, Debug)]
pub enum FieldSize{
    Four,
    Six,
}


impl Default for FieldSize{
    fn default() -> FieldSize{
        FieldSize::Four
    }
}

#[derive( Default, Deserialize, Serialize, Debug)]
pub struct Field{
    size: FieldSize,
    field_4 : Option<[[u32; 4]; 4]>,
    field_6 : Option<[[u32; 6]; 6]>,
}

#[derive(Default)]
pub struct GameState{
    field : Field,
    size : u32,
}

impl SimpleState for GameState {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>){
       self.read_from_save();
       println!("{:?}", self.field);
    }

    fn on_pause(&mut self, _data: StateData<'_, GameData<'_, '_>>){
        self.write_to_save();
        println!("{:?}", self.field);
    }

    fn on_resume(&mut self, _data: StateData<'_, GameData<'_, '_>>){
        self.read_from_save();
        println!("{:?}", self.field);
    }

    fn on_stop(&mut self, _data: StateData<'_, GameData<'_, '_>>){
        match self.field.field_4{
            Some(_) => self.write_to_save(),
            None => match self.field.field_6{
                Some(_) => self.write_to_save(),
                None => (),
            },
        }
        
        println!("{:?}", self.field);
    }

    fn handle_event(&mut self, _data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans{
        match event {
            StateEvent::Window(event) => {
                if is_close_requested(&event) {
                    println!("[Trans::Push] GameState => ExitState, krestik");
                    return Trans::Push(Box::new(exit::ExitState::default()));
                }else if is_key_down(&event, VirtualKeyCode::P) {
                    return Trans::None;
                }else if is_key_down(&event, VirtualKeyCode::Escape) {
                    println!("[Trans::Push] From GameState to ExitState");
                    return Trans::Push(Box::new(exit::ExitState::default()));
                }else{
                    return Trans::None;
                }
            }
            _ => return Trans::None,
        }
        
    }

    
}

impl GameState{
    fn read_from_save(&mut self){

        let input_str = fs::read_to_string("save.ron").expect("cant open a save.ron");
        self.field  = ron::de::from_str(&input_str).unwrap();
        match self.field.field_4{
            Some(_) => self.size = 4,
            None => self.size = 6,
        }
    }

    fn write_to_save(&mut self){   
        let ron_str = ron::ser::to_string(&self.field).unwrap();
        fs::write("save.ron", ron_str.as_bytes()).expect("DONT WRITE BLYADINA");
        self.field.field_4 = None;
        self.field.field_6 = None;
    }
}