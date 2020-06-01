use amethyst::{
    StateData,
    GameData,
    ecs::prelude::*,
    SimpleState,
    SimpleTrans,
    StateEvent,
    Trans,
    ui::{UiCreator, UiEvent, UiEventType, UiFinder},
    input::{is_key_down},
    winit::VirtualKeyCode,
};

#[derive(Default)]
pub struct  HelpState {
    ui_root : Option<Entity>,
    button_back : Option<Entity>,
}


impl SimpleState for HelpState {

    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>){
        self.ui_root = Some(data.world.exec(|mut creator: UiCreator| creator.create("ui/help.ron", ())));
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans{
        let StateData {world, ..} = data;
        if self.button_back.is_none(){
                    world.exec(|ui_finder : UiFinder| {
                    self.button_back = ui_finder.find("back");
                });
            }
        Trans::None
    }

    fn handle_event(&mut self, _data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
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
                if Some(target) == self.button_back {
                    println!("[Trans::Pop] HelpState => MenuState!");
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
                .expect("Failed to remove HelpState");
        }

        self.ui_root = None;
        self.button_back = None;
    }
}