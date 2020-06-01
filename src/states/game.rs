use amethyst::{
    assets::*,
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
    ui::{UiCreator, UiFinder, UiText, UiEvent, UiEventType},
    renderer::*,
    core::transform::Transform,
};

use std::fmt::{Debug};
use crate::game_field::*;
use crate::states::exit;
use crate::states::pause_menu;
use crate::states::end_game;
use crate::states::main_menu;




const WIDTH : f32 = 1200.0;
const HEIGTH : f32 = 950.0;
const BUTTON_MUSIC: &str = "music";
const BUTTON_END_GAME: &str = "end_game";
const BUTTON_RANDOM: &str = "random";

#[derive(Default, Debug)]
pub struct GameState{
    field : Field,
    ui_root: Option<Entity>,
    ui_score: Option<Entity>,
    button_music: Option<Entity>,
    button_end: Option<Entity>,
    button_random: Option<Entity>,
    sprite_sheet_handle: Option<Handle<SpriteSheet>>,
    entities: Vec<Entity>,
    score: String,
}

impl SimpleState for GameState {

    
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>){
       self.initialize(data.world);
       println!("{:?}", self.field);
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans{
        let StateData{world, ..} = data;
        match self.ui_score{
            None => {
                world.exec(|ui_finder : UiFinder| {
                    self.ui_score = ui_finder.find("score");
                    self.button_end = ui_finder.find(BUTTON_END_GAME);
                    self.button_music = ui_finder.find(BUTTON_MUSIC);
                    self.button_random = ui_finder.find(BUTTON_RANDOM);
                });
            }
            Some(_) => {
                let (b, mut c): (Entities, WriteStorage<UiText>) = world.system_data();
                for (entity, text) in (&b, &mut c).join(){
                    if Some(entity) == self.ui_score{
                        text.text = self.score.clone();
                    }
                }
            }
        }

       
        
        Trans::None
    }

    fn on_pause(&mut self, data: StateData<'_, GameData<'_, '_>>){
        self.field.save("save.ron");

        if let Some(root_entity) = self.ui_root {
            data.world
                .delete_entity(root_entity)
                .expect("Failed to remove MainMenu");
        }
        self.remove_entities(data.world);
        self.ui_root = None;
        self.ui_score = None;
        self.button_music = None;
        self.button_end = None;
        self.button_random = None;
    }

    fn on_resume(&mut self, data: StateData<'_, GameData<'_, '_>>){
        self.field = Field::read("save.ron");

        match self.field.field_4 {
            Some(_) => self.initialize_field_4(data.world),
            None => self.initialize_field_6(data.world),
        }

        let world = data.world;
        self.ui_root = Some(world.exec(|mut creator: UiCreator<'_>| creator.create("ui/game.ron", ())));
    }

    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>){
        match self.field.size {
            FieldSize::Four => { 
                if self.field.field_4.is_some() {
                    self.field.save("save.ron");
                }
            }
            FieldSize::Six =>{ 
                if self.field.field_6.is_some() {
                    self.field.save("save.ron");
                }
            }
            FieldSize::Empty => self.field.save("save.ron"),
        }

        if let Some(root_entity) = self.ui_root {
            data.world
                .delete_entity(root_entity)
                .expect("Failed to remove MainMenu");
        }
        self.remove_entities(data.world);
        self.ui_root = None;
        self.ui_score = None;
        self.button_music = None;
        self.button_end = None;
        self.button_random = None;
    }

    fn handle_event(&mut self, data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans{
        let user_move :Usermove;
        match event {
            StateEvent::Window(event) => {
                if is_close_requested(&event) {
                    println!("[Trans::Push] GameState => ExitState, krestik");
                    return Trans::Push(Box::new(exit::ExitState::default()));
                }else if is_key_down(&event, VirtualKeyCode::Escape) {
                    println!("[Trans::Push] From GameState to PauseMenu");
                    return Trans::Push(Box::new(pause_menu::PauseMenuState::default()));
                }else if is_key_down(&event, VirtualKeyCode::Right) {
                    println!("[Trans::None] Right");
                    user_move = Usermove::Right;
                   
                }else if is_key_down(&event, VirtualKeyCode::Left) {
                    println!("[Trans::None] Left");
                    user_move = Usermove::Left;
                    
                }else if is_key_down(&event, VirtualKeyCode::Up) {
                    println!("[Trans::None] Up");
                    user_move = Usermove::Up;
                    
                }else if is_key_down(&event, VirtualKeyCode::Down) {
                    println!("[Trans::None] Down");
                    user_move = Usermove::Down;
                }else if is_key_down(&event, VirtualKeyCode::Return){
                    if self.field.loose {
                        if !self.field.skip {
                            return Trans::Push(Box::new(end_game::EndGameState {
                                score : self.field.score, 
                                name: String::new(), 
                                hard: self.field.hard,
                                size : self.field.size.clone(),
                                ui_root : None, 
                                input : None, 
                                enter_button: None
                            }));
                        } else {
                            return Trans::Switch(Box::new(main_menu::MainMenuState::default()));
                        }
                    }
                    return Trans::None;
                }else {
                    return Trans::None;
                }
                self.game_turn(&user_move, data.world);
                return Trans::None;
            }

            StateEvent::Ui(UiEvent {
                event_type: UiEventType::Click,
                target,
            }) => {
                if Some(target) == self.button_end {
                    if !self.field.skip {
                        return Trans::Push(Box::new(end_game::EndGameState {
                            score : self.field.score, 
                            name: String::new(), 
                            hard: self.field.hard,
                            size : self.field.size.clone(),
                            ui_root : None, 
                            input : None, 
                            enter_button: None
                        }));
                    } else {
                        self.field = Field::empty();
                        return Trans::Switch(Box::new(main_menu::MainMenuState::default()));
                    }
                }if Some(target) == self.button_music {
                    println!("PLAY MUSCIC!");
                    return Trans::None;
                }if Some(target) == self.button_random {
                    println!("RANDOM OUR FIELD!");
                    self.random(data.world);
                    return Trans::None;
                }
                return Trans::None;
            }
            _ => return Trans::None,
        }
        
    }

    
}

impl GameState{

    fn random(&mut self, world: &mut World){
        let moves = [Usermove::Down, Usermove::Left, Usermove::Up, Usermove::Right ];
        self.field.skip = true;
        let rand_number = 50 + rand::random::<usize>() % 100;
        let rand_moves = rand::random::<usize>() % 3;
        for _ in 0..(rand_number/2) {
            self.game_turn(&moves[rand_moves], world);
            self.game_turn(&moves[rand_moves + 1], world);
        }

    }

    fn remove_entities(&mut self, world: &mut World){
        for entity in self.entities.iter(){
            world.delete_entity(*entity).expect("НЕ СМОГ УДАЛИТЬ КАРТИНКУ");
        }
        self.entities = Vec::<Entity>::new();
    }

    fn initialize_field_4(&mut self, world: &mut World){
        
        let array = self.field.field_4.unwrap();

        for i in 0..4{
            for j in 0..4{
                let pow = power_of_2(array[i][j]);
                let mut transform = Transform::default();
                transform.set_translation_xyz(-170.0 + j as f32 * 120.0, 200.0 - i as f32 *120.0, 0.0);
   
                let sprite_render = SpriteRender {
                    sprite_sheet: self.sprite_sheet_handle.clone().unwrap(),
                    sprite_number: pow as usize, 
                };

                let entity = world
                                .create_entity()
                                .with(sprite_render)
                                .with(transform)
                                .build();
                self.entities.push(entity);
            }
        }
    }

    fn initialize_field_6(&mut self, world: &mut World){
        
        let array = self.field.field_6.unwrap();

        for i in 0..6{
            for j in 0..6{
                let pow = power_of_2(array[i][j]);
                let mut transform = Transform::default();
                transform.set_translation_xyz(-250.0 + j as f32 * 100.0, 250.0 - i as f32 *100.0, 0.0);
   
                let sprite_render = SpriteRender {
                sprite_sheet: self.sprite_sheet_handle.clone().unwrap(),
                    sprite_number: pow as usize, 
                };

                let entity = world
                                .create_entity()
                                .with(sprite_render)
                                .with(transform)
                                .build();
                self.entities.push(entity);
            }
        }
    }

    fn draw_4(&mut self, world: &mut World, array: [[u32;4];4]){
        self.remove_entities(world);
        self.field.field_4.replace(array);
        self.initialize_field_4(world);
    }

    fn draw_6(&mut self, world: &mut World, array: [[u32;6];6]){
        self.remove_entities(world);
        self.field.field_6.replace(array);
        self.initialize_field_6(world);
    }

    
    fn initialize(&mut self, world: &mut World){
        self.field = Field::read("save.ron");
        self.ui_root = Some(world.exec(|mut creator: UiCreator<'_>| creator.create("ui/game.ron", ())));
        initialize_camera(world);
        if self.field.loose{
            self.score = format!("YOU LOOSE, SCORE : {}", self.field.score);
        } else {
            self.score = format!("YOUR SCORE : {}", self.field.score);
        }
        match self.field.field_4 {
            Some(_) => {
                self.sprite_sheet_handle.replace(load_sprite_sheet_4(world));
                self.initialize_field_4(world);
                self.field.score = calc_score_4(&mut self.field.field_4.unwrap());
            }
            None => {
                self.sprite_sheet_handle.replace(load_sprite_sheet_6(world));
                self.initialize_field_6(world);
                self.field.score = calc_score_6(&mut self.field.field_6.unwrap());
            }
        }
    }

    fn game_turn(&mut self, user_nove: &Usermove, world: &mut World){
        match self.field.field_4 {
            Some(field) => {
                let mut array = field.clone();
                if test_4(&mut array){
                    do_game_step_4(&user_nove, &mut array);
                    if self.field.hard {
                        spawn_4(&mut array);
                        spawn_4(&mut array);
                    } else {
                        spawn_4(&mut array);
                    }

                    self.field.score = calc_score_4(&mut array);
                    self.score = format!("YOUR SCORE : {}",  self.field.score);
                    self.draw_4(world, array);
    
                } else {
                    self.field.score = calc_score_4(&mut array);
                    let score_str = format!("YOU LOOSE, SCORE: {}", self.field.score);
                    self.field.loose = true;
                    self.score = score_str;
                }
            }
            None => {
                let mut array = self.field.field_6.clone().unwrap();
                if test_6(&mut array){
                    do_game_step_6(&user_nove, &mut array);
                    if self.field.hard {
                        spawn_6(&mut array);
                        spawn_6(&mut array);
                    } else {
                        spawn_6(&mut array);
                    }
                    self.field.score = calc_score_6(&mut array);
                    self.score = format!("YOUR SCORE : {}",  self.field.score);
                    self.draw_6(world, array);
    
                } else {
                    self.field.score = calc_score_6(&mut array);
                    let score_str = format!("YOU LOOSE, SCORE: {}", self.field.score);
                    self.field.loose = true;
                    self.score = score_str;
                }
            }
        }
    }
}



fn power_of_2(mut i : u32) -> u32{
    if i == 0 {
         return 0;
    }
    let mut counter = 0;
    while i!= 1 {
        i /=2;
        counter +=1;
    }
    return counter;
}

fn initialize_camera(world : &mut World){
    let mut transform = Transform::default();
    transform.set_translation_xyz(0.0, 0.0, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(WIDTH, HEIGTH))
        .with(transform)
        .build();
}



fn load_sprite_sheet_4(world: &mut World) -> Handle<SpriteSheet> {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/4_0.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };
    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "texture/4_0.ron",
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}

fn load_sprite_sheet_6(world: &mut World) -> Handle<SpriteSheet> {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/6_0.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };
    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "texture/6_0.ron",
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}



