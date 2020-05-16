use amethyst::{
    StateData,
    GameData,
    ecs::prelude::*,
    SimpleState,
    SimpleTrans,
    StateEvent,
    Trans,
    input::{is_close_requested, is_key_down},
    ui::{UiCreator, UiEvent, UiEventType, UiFinder},
    winit::VirtualKeyCode,
};

use crate::states::exit;
use crate::states::game;
const BUTTON_4X4: &str = "4x4";
const BUTTON_6X6: &str = "6x6";
const BUTTON_BACK: &str = "back";

#[derive(Default, Debug)]
pub struct FieldChooseState{
    ui_root: Option<Entity>,
    button_4x4: Option<Entity>,
    button_6x6: Option<Entity>,
    button_back: Option<Entity>,
}

impl SimpleState for FieldChooseState{

    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>){
        let world = data.world;
        self.ui_root = Some(world.exec(|mut creator: UiCreator| creator.create("ui/choose.ron", ())));
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans{
        let StateData {world, ..} = data;
        if self.button_4x4.is_none()
            || self.button_6x6.is_none()
            || self.button_back.is_none(){
                    world.exec(|ui_finder : UiFinder| {
                    self.button_back = ui_finder.find(BUTTON_BACK);
                    self.button_4x4 = ui_finder.find(BUTTON_4X4);
                    self.button_6x6 = ui_finder.find(BUTTON_6X6);
                });
            }

        Trans::None
    }

    fn handle_event(&mut self,_data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans{
        
        match event {
            StateEvent::Window(event) => {
                if is_close_requested(&event) {
                    println!("[Trans::Push] MainState => ExitState, krestik");
                    return Trans::Push(Box::new(exit::ExitState::default()));
                }else if is_key_down(&event, VirtualKeyCode::Escape) {
                    println!("[Trans::Pop] Returning to MainMenuState, esc!");
                    return Trans::Pop;
                } else{
                    return Trans::None;
                }
            }

            StateEvent::Ui(UiEvent {
                event_type: UiEventType::Click,
                target,
            }) => {
                if Some(target) == self.button_4x4 {
                    println!("[Trans::Switch] Switching to GameState, 4x4!");
                    let new_field : game::Field = game::Field{
                        score: 0,
                        size: game::FieldSize::Four,
                        field_4: Some([[0, 0, 0, 0],[0, 0, 0, 0],[0, 0, 0, 0],[0, 0, 0, 0]]),
                        field_6: None,
                    };
                    std::fs::write("save.ron", ron::ser::to_string(&new_field).unwrap()).expect("some problem with writing new 4x4 field");
                    return Trans::Switch(Box::new(game::GameState::default()));
                }
                if Some(target) == self.button_6x6 {
                    println!("[Trans::Switch] Switching to GameState, 6x6!");
                    let new_field : game::Field = game::Field{
                        score: 0,
                        size: game::FieldSize::Four,
                        field_4: None,
                        field_6: Some([
                            [0, 0, 0, 0, 0, 0],
                            [0, 0, 0, 0, 0, 0],
                            [0, 0, 0, 0, 0, 0],
                            [0, 0, 0, 0, 0, 0],
                            [0, 0, 0, 0, 0, 0],
                            [0, 0, 0, 0, 0, 0],
                        ]),
                    };
                    std::fs::write("save.ron", ron::ser::to_string(&new_field).unwrap()).expect("some problem with writing new 6x6 field");
                    return Trans::Switch(Box::new(game::GameState::default()));
                }
                if Some(target) == self.button_back {
                    println!("[Trans::Pop] Returning to MainMenuState, button back!");
                    return Trans::Pop;
                }
                return Trans::None;
            }

            _ =>  return Trans::None,
        }
    }

    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>){
        if let Some(root_entity) = self.ui_root {
            data.world
                .delete_entity(root_entity)
                .expect("Failed to remove FieldChooseState");
        }

        self.ui_root = None;
        self.button_4x4 = None;
        self.button_6x6 = None;
        self.button_back = None;
        
    }
}