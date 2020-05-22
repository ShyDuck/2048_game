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

use crate::states::game;
use crate::states::exit;

const BUTTON_HARD : &str = "hard";
const BUTTON_EASY : &str = "easy";

pub struct DiffucultyState{
    pub field : game::Field,
    pub ui_root : Option<Entity>,
    pub button_hard : Option<Entity>,
    pub button_easy : Option<Entity>,
}

impl SimpleState for DiffucultyState{

    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>){
        self.ui_root = Some(data.world.exec(|mut creator: UiCreator| creator.create("ui/diffuculty.ron", ())));
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans{

        if self.button_hard.is_none()
            || self.button_easy.is_none(){
                data.world.exec(|ui_finder : UiFinder| {
                    self.button_hard = ui_finder.find(BUTTON_HARD);
                    self.button_easy = ui_finder.find(BUTTON_EASY);
                });
            }
        Trans::None
    }

    fn handle_event(&mut self, _data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans{

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
                if Some(target) == self.button_easy {
                    println!("[Trans::Switch] Switching to GameState, easy!");
                    self.field.hard = false;
                    let ron_str = ron::ser::to_string_pretty(&self.field, ron::ser::PrettyConfig::default()).expect("cant field -> ron, DiffucultyState");
                    std::fs::write("save.ron", ron_str).expect("cant write to save.ron, DiffucultyState");
                    return Trans::Switch(Box::new(game::GameState::default()));
                }
                if Some(target) == self.button_hard {
                    println!("[Trans::Switch] Switching to GameState, hard!");
                    self.field.hard = true;
                    let ron_str = ron::ser::to_string_pretty(&self.field, ron::ser::PrettyConfig::default()).expect("cant field -> ron, DiffucultyState");
                    std::fs::write("save.ron", ron_str).expect("cant write to save.ron, DiffucultyState");
                    return Trans::Switch(Box::new(game::GameState::default()));
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
        self.button_easy = None;
        self.button_hard = None;
    }
}

