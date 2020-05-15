
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
};

use crate::states::field_choose;
use crate::states::exit;
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

impl SimpleState for MainMenuState{
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>){
        let world = data.world;

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
    

    fn handle_event(&mut self, _data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans{


        match event {
            StateEvent::Window(event) => {
                if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                    println!("[Trans::Push] MainState => ExitState, esc or krestik");
                    return Trans::Push(Box::new(exit::ExitState::default()));
                }else {
                    Trans::None
                }
            }

            StateEvent::Ui(UiEvent {
                event_type: UiEventType::Click,
                target,
            }) => {
                if Some(target) == self.button_start {
                    println!("[Trans::Switch] Switching to GameState, StartNewGame!");
                    return Trans::Push(Box::new(field_choose::FieldChooseState::default()));
                }
                if Some(target) == self.button_continue {
                    println!("[Trans::Switch] Switching to GameState, LoadingGame!");
                    return Trans::None;
                }
                if Some(target) == self.button_leader_board {
                    println!("[Trans::Switch] Switching to LeaderBoardState!");
                    return Trans::None;
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

