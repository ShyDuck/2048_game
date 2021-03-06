
use amethyst::{
    SimpleState,
    StateData,
    GameData,
    Trans,
    StateEvent,
    input::{is_close_requested, is_key_down},
    SimpleTrans,
    ecs::prelude::*,
    ui::{UiCreator, UiEvent, UiEventType, UiFinder},
    winit::VirtualKeyCode,
    audio::{AudioSink},
};

use crate::states::field_choose;
use crate::states::exit;
use crate::states::game;
use crate::states::leader;
use crate::states::help;
use crate::game_field::{Field, FieldSize};

//This State represent main menu
const BUTTON_START: &str = "start";
const BUTTON_CONTINUE: &str = "continue";
const BUTTON_LEADER_BOARD: &str = "leader_board";
const BUTTON_EXIT: &str = "exit";


#[derive(Default, Debug)]
pub struct MainMenuState{
    ui_root: Option<Entity>,
    button_start: Option<Entity>,
    button_continue: Option<Entity>,
    button_leader_board: Option<Entity>,
    button_exit: Option<Entity>,
}

use crate::audio;

impl SimpleState for MainMenuState{
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>){
        let world = data.world;
        audio::initialise_audio(world);
        self.ui_root = 
            Some(world.exec(|mut creator: UiCreator<'_>| creator.create("ui/menu.ron", ())));
    }
    
    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans{
       let StateData {world, ..} = data;
        if self.button_start.is_none()
            || self.button_continue.is_none()
            || self.button_exit.is_none()
            || self.button_leader_board.is_none()
            || self.button_exit.is_none(){
                world.exec(|ui_finder: UiFinder<'_>|{
                    self.button_start = ui_finder.find(BUTTON_START);
                    self.button_continue = ui_finder.find(BUTTON_CONTINUE);
                    self.button_leader_board = ui_finder.find(BUTTON_LEADER_BOARD);
                    self.button_exit = ui_finder.find(BUTTON_EXIT);
                });
            }
        Trans::None
    }
    
    fn handle_event(&mut self, data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans{


        match event {
            StateEvent::Window(event) => {
                if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                    println!("[Trans::Push] MainState => ExitState, esc or krestik");
                    return Trans::Push(Box::new(exit::ExitState::default()));
                }else if is_key_down(&event, VirtualKeyCode::F1) {
                    println!("[Trans::Push] MainState => HelpState, f1");
                    return Trans::Push(Box::new(help::HelpState::default()));
                }else if is_key_down(&event, VirtualKeyCode::PageDown) {
                    println!("make music tishe");
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
                }else {
                    Trans::None
                }
            }

            StateEvent::Ui(UiEvent {
                event_type: UiEventType::Click,
                target,
            }) => {
                if Some(target) == self.button_start {
                    println!("[Trans::Push] Switching to FieldChooseState, button_start!");
                    return Trans::Push(Box::new(field_choose::FieldChooseState::default()));
                }
                if Some(target) == self.button_continue {
                    
                    let field = Field::read("save.ron");
                    match field.size{
                        FieldSize::Four => {
                            println!("[Trans::Switch] Switching to GameState, LoadingGame!");
                            return Trans::Switch(Box::new(game::GameState::default()));
                        }
                        FieldSize::Six => {
                            println!("[Trans::Switch] Switching to GameState, LoadingGame!");
                            return Trans::Switch(Box::new(game::GameState::default()));
                        }
                        FieldSize::Empty => return Trans::None,
                    }
                }
                if Some(target) == self.button_leader_board {
                    println!("[Trans::Push] Switching to LeaderBoardState!");
                    return Trans::Push(Box::new(leader::LeaderState::default()));
                }
                if Some(target) == self.button_exit {
                    println!("[Trans::Push] MainState => ExitState, button exit");
                    return Trans::Push(Box::new(exit::ExitState::default()));
                }
                Trans::None
            }
            _ => Trans::None,
        }
    }

    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        // after destroying the current UI, invalidate references as well (makes things cleaner)
        if let Some(root_entity) = self.ui_root {
            data.world
                .delete_entity(root_entity)
                .expect("Failed to remove MainMenu");
        }

        self.ui_root = None;
        self.button_start = None;
        self.button_continue = None;
        self.button_leader_board = None;
        self.button_exit = None;
    }

    fn on_pause(&mut self, data: StateData<'_, GameData<'_, '_>>){
        if let Some(root_entity) = self.ui_root {
            data.world
                .delete_entity(root_entity)
                .expect("Failed to remove MainMenu");
        }

        self.ui_root = None;
        self.button_start = None;
        self.button_continue = None;
        self.button_leader_board = None;
        self.button_exit = None;
    }

    fn on_resume(&mut self, data: StateData<'_, GameData<'_, '_>>){
        let world = data.world;

        self.ui_root = 
            Some(world.exec(|mut creator: UiCreator<'_>| creator.create("ui/menu.ron", ())));
    }

}

