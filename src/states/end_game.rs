use amethyst::{
    SimpleState,
    StateData,
    GameData,
    SimpleTrans,
    Trans,
    StateEvent,
    prelude::*,
    input::{is_close_requested, is_key_down},
    winit::VirtualKeyCode,
    ecs::prelude::*,
    ui::{UiCreator, UiFinder, UiText,  UiEvent, UiEventType,},
};


pub struct EndGameState {
    pub score: u32,
    pub name : String,
    pub hard: bool,
    pub size: FieldSize,
    pub ui_root: Option<Entity>,
    pub input: Option<Entity>,
    pub enter_button: Option<Entity>,
}

use crate::states::exit;
use crate::states::main_menu;
use crate::states::leader::{LeaderBoard, Leader};
use crate::game_field::{Field, FieldSize};

const ENTER_BUTTON: &str = "enter";
const INPUT_NAME: &str = "input_name";

impl SimpleState for EndGameState{
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>){
        self.ui_root = Some(data.world.exec(|mut creator: UiCreator<'_>| creator.create("ui/end_game.ron", ())));
        self.name = String::from("enter your name here");
    }

    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>){
        if let Some(root_entity) = self.ui_root {
            data.world
                .delete_entity(root_entity)
                .expect("Failed to remove MainMenu");
        }
        self.input = None;
        self.ui_root = None;
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans{
        let StateData {world, ..} = data;
        if self.enter_button.is_none()
            || self.input.is_none(){
                world.exec(|ui_finder : UiFinder| {
                    self.enter_button = ui_finder.find(ENTER_BUTTON);
                    self.input = ui_finder.find(INPUT_NAME);
                });
            }

        Trans::None
    }

    fn handle_event(&mut self, data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans{
        match event{
            StateEvent::Window(event) => {
                if is_close_requested(&event) {
                    println!("[Trans::Push] PauseMenuState => ExitState, krestik");
                    return Trans::Push(Box::new(exit::ExitState::default()));
                }else if is_key_down(&event, VirtualKeyCode::Escape){
                    println!("[Trans::Pop] From EndGameState => GameState, esc");
                    return Trans::Pop;
                }else {
                    return Trans::None;
                }
            }

            StateEvent::Ui(UiEvent {
                event_type: UiEventType::Click,
                target,
            }) => {
                if Some(target) == self.enter_button {
                    if self.name == String::from("enter your name here") || self.name == String::new(){
                        return Trans::None;
                    } else{
                        println!("[Trans::Switch] From EndGameState => MainMenuState, button_enter!");
                        //запись в файл
                        self.add_to_leader();

                        let field = Field::empty();
                        std::fs::write("save.ron", ron::ser::to_string_pretty(&field, ron::ser::PrettyConfig::default()).unwrap()).expect("cant load new save.ron, EndGameState");
                        return Trans::Switch(Box::new(main_menu::MainMenuState::default()));
                    }
                }
                return Trans::None;
            }  

            StateEvent::Ui(UiEvent {
                event_type: UiEventType::ValueChange,
                target: _,
            }) => {
                self.set_name(data.world);
                return Trans::None;
            }

            _ => return Trans::None,
        }
    }

}



impl EndGameState {
    fn set_name(&mut self, world: &mut World){
        let (b, mut c): (Entities, WriteStorage<UiText>) = world.system_data();
            for (entity, text) in (&b, &mut c).join(){
                if Some(entity) == self.input {
                    self.name = text.text.clone();
                }
            }
    }

    fn add_to_leader(&mut self){
        let ron_str = std::fs::read_to_string("leader.ron").expect("Cant read leader.ron from EndGameState");
        let mut leader_board : LeaderBoard = ron::de::from_str(&ron_str).expect("Some triuble with leader.ron, EndGameState");
        leader_board.add_leader(Leader{ name : self.name.clone(), score: self.score, hard: self.hard, size : self.size.clone()});
        std::fs::write("leader.ron", ron::ser::to_string_pretty(&leader_board, ron::ser::PrettyConfig::default()).unwrap().as_bytes()).expect("Cant save leaderboard, EndGameState");
    }
}