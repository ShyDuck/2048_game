use amethyst::{
    StateData,
    GameData,
    ecs::prelude::*,
    SimpleState,
    SimpleTrans,
    StateEvent,
    Trans,
    input::{is_key_down},
    ui::{UiCreator, UiEvent, UiEventType, UiFinder},
    winit::VirtualKeyCode,
};

//This State asks user for exit confirmation
const BUTTON_YES: &str = "yes";
const BUTTON_NO: &str = "no";

#[derive(Default, Debug)]
pub struct ExitState{
    ui_root: Option<Entity>,
    button_yes: Option<Entity>,
    button_no: Option<Entity>,
}

impl SimpleState for ExitState{
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>){
        let world = data.world;
        self.ui_root = Some(world.exec(|mut creator: UiCreator| creator.create("ui/exit.ron", ())));
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans{
        let StateData {world, ..} = data;
        if self.button_yes.is_none()
            || self.button_no.is_none(){
                    world.exec(|ui_finder : UiFinder| {
                    self.button_yes = ui_finder.find(BUTTON_YES);
                    self.button_no = ui_finder.find(BUTTON_NO);
                });
            }
        Trans::None
    }

    fn handle_event(&mut self,_data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans{
        match event {
            StateEvent::Window(event) => {
                if is_key_down(&event, VirtualKeyCode::Escape) {
                    println!("[Trans::Pop] pressed esc in ExitState!");
                    return Trans::Pop;
                } else{
                    return Trans::None;
                }
            }

            StateEvent::Ui(UiEvent {
                event_type: UiEventType::Click,
                target,
            }) => {
                if Some(target) == self.button_yes {
                    println!("[Trans::Quit] ExitState, confirm exit, yes button!");
                    return Trans::Quit;
                }
                if Some(target) == self.button_no{
                    println!("[Trans::Pop] dont confirmed exit from Exit State!");
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
        self.button_yes = None;
        self.button_no = None;
        
    }
}