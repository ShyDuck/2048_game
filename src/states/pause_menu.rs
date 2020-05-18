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

use crate::states::exit;
use crate::states::field_choose;
use crate::states::main_menu;

const BUTTON_RETURN: &str = "return";
const BUTTON_NEW_GAME: &str = "new_game";
const BUTTON_TO_MENU: &str = "to_menu";
 

#[derive(Default, Debug)]
pub struct PauseMenuState{
    ui_root : Option<Entity>,
    button_return : Option<Entity>,
    button_new_game : Option<Entity>,
    button_to_menu : Option<Entity>,
 }

 impl SimpleState for PauseMenuState{
     fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>){
        let world = data.world;
        self.ui_root = 
            Some(world.exec(|mut creator: UiCreator<'_>| creator.create("ui/pause.ron", ())));
     }

     fn on_resume(&mut self, data: StateData<'_, GameData<'_, '_>>){
        let world = data.world;
        self.ui_root = 
            Some(world.exec(|mut creator: UiCreator<'_>| creator.create("ui/pause.ron", ())));
     }

     fn on_pause(&mut self, data: StateData<'_, GameData<'_, '_>>){
        if let Some(root_entity) = self.ui_root {
            data.world
                .delete_entity(root_entity)
                .expect("Failed to remove MainMenu");
        }
        self.ui_root = None;
        self.button_new_game = None;
        self.button_return = None;
        self.button_to_menu = None;
     }

     fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>){
        if let Some(root_entity) = self.ui_root {
            data.world
                .delete_entity(root_entity)
                .expect("Failed to remove MainMenu");
        }
        self.ui_root = None;
        self.button_new_game = None;
        self.button_return = None;
        self.button_to_menu = None;

     }

     fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans{
        let StateData {world, ..} = data;
        if self.button_new_game.is_none()
            || self.button_return.is_none()
            || self.button_to_menu.is_none(){
                world.exec(|ui_finder: UiFinder<'_>|{
                    self.button_new_game = ui_finder.find(BUTTON_NEW_GAME);
                    self.button_return = ui_finder.find(BUTTON_RETURN);
                    self.button_to_menu = ui_finder.find(BUTTON_TO_MENU);
                });
            }
        Trans::None
     }

     fn handle_event(&mut self, _data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans{
        match event{
            StateEvent::Window(event) => {
                if is_close_requested(&event) {
                    println!("[Trans::Push] PauseMenuState => ExitState, krestik");
                    return Trans::Push(Box::new(exit::ExitState::default()));
                }else if is_key_down(&event, VirtualKeyCode::Escape){
                    println!("[Trans::Pop] From PauseMenuState => GameState, esc");
                    return Trans::Pop;
                }else {
                    return Trans::None;
                }
            }

            StateEvent::Ui(UiEvent {
                event_type: UiEventType::Click,
                target,
            }) => {
                if Some(target) == self.button_return {
                    println!("[Trans::Pop] From PauseMenuState => GameState, button_return!");
                    return Trans::Pop;
                }
                if Some(target) == self.button_new_game {
                    println!("[Trans::Push] Pushing Field Choose from Pause MenuState, button_new_game!");
                    return Trans::Push(Box::new(field_choose::FieldChooseState::default()));
                }
                if Some(target) == self.button_to_menu {
                    println!("[Trans::Switch] PauseMenuState => MainMenuState, button_to_menu!");
                    return Trans::Switch(Box::new(main_menu::MainMenuState::default()));
                }
                return Trans::None;
            }

            _ => return Trans::None,
        }
     }
 }

