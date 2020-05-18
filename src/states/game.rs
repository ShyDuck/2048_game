
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
    ui::{UiCreator, UiEvent, UiEventType, UiFinder, UiText, UiLabel},
    renderer::*,
    core::transform::Transform,
};

use std::fmt::{Debug};
use crate::states::exit;
use crate::states::pause_menu;
use serde::{Deserialize, Serialize};
use std::fs;
use ron;


const WIDTH : f32 = 1200.0;
const HEIGTH : f32 = 900.0;

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
    pub score: u32,
    pub size: FieldSize,
    pub field_4 : Option<[[u32; 4]; 4]>,
    pub field_6 : Option<[[u32; 6]; 6]>,
}

#[derive(Default)]
pub struct GameState{
    field : Field,
    size : u32,
    ui_root: Option<Entity>,
    ui_score: Option<Entity>,
    sprite_sheet_handle: Option<Handle<SpriteSheet>>,
    entities: Vec<Entity>,
    score: String,
}

impl SimpleState for GameState {

    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>){
       self.read_from_save();
       println!("starting new game with: {:?}", self.field);
        
       let world = data.world;
       self.ui_root = 
       Some(world.exec(|mut creator: UiCreator<'_>| creator.create("ui/game.ron", ())));
        
        self.sprite_sheet_handle.replace(load_sprite_sheet(world));
        
        initialize_camera(world);
        match self.field.field_4 {
            Some(_) => self.initialize_field_4(world),
            None => self.initialize_field_6(world),
        }
        
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans{
        let StateData{world, ..} = data;
        match self.ui_score{
            None => {
                world.exec(|ui_finder : UiFinder| {
                    self.ui_score = ui_finder.find("score");
                });
            }
            Some(_) => {
                let (b, mut c): (Entities, WriteStorage<UiText>) = world.system_data();
                for (_, text) in (&b, &mut c).join(){
                    text.text = self.score.clone();
                }
            }
        }

       
        
        Trans::None
    }

    fn on_pause(&mut self, data: StateData<'_, GameData<'_, '_>>){
        self.write_to_save();

        if let Some(root_entity) = self.ui_root {
            data.world
                .delete_entity(root_entity)
                .expect("Failed to remove MainMenu");
        }
        self.remove_entities(data.world);
        self.ui_root = None;
    }

    fn on_resume(&mut self, data: StateData<'_, GameData<'_, '_>>){
        self.read_from_save();

        match self.field.field_4 {
            Some(_) => self.initialize_field_4(data.world),
            None => self.initialize_field_6(data.world),
        }

        let world = data.world;
        self.ui_root = 
        Some(world.exec(|mut creator: UiCreator<'_>| creator.create("ui/game.ron", ())));
    }

    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>){
        match self.field.field_4{
            Some(_) => self.write_to_save(),
            None => match self.field.field_6{
                Some(_) => self.write_to_save(),
                None => (),
            },
        }

        if let Some(root_entity) = self.ui_root {
            data.world
                .delete_entity(root_entity)
                .expect("Failed to remove MainMenu");
        }
        self.remove_entities(data.world);
        self.ui_root = None;
    }

    fn handle_event(&mut self, data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans{
        match event {
            StateEvent::Window(event) => {
                if is_close_requested(&event) {
                    println!("[Trans::Push] GameState => ExitState, krestik");
                    return Trans::Push(Box::new(exit::ExitState::default()));
                }else if is_key_down(&event, VirtualKeyCode::Escape) {
                    println!("[Trans::Push] From GameState to PauseMenu");
                    return Trans::Push(Box::new(pause_menu::PauseMenuState::default()));
                }else if is_key_down(&event, VirtualKeyCode::P) {
                    self.field.score +=1;
                    match self.field.field_4 {
                        Some(mut array) => {
                            for i in 0..4{
                                for j in 0..4{
                                    array[i][j] *= 2;
                                }
                            }
                            self.draw_4(data.world, array);
                        }
                        None => (),
                    }
                    return Trans::None;
                }else if is_key_down(&event, VirtualKeyCode::Right) {
                    println!("[Trans::None] Right");
                    match self.field.field_4 {
                        Some(field) => {
                            let mut array = field.clone();
                            if test_4(&mut array){
                                do_game_step_4(&Usermove::Right, &mut array);
                                spawn_4(&mut array);
                                self.score = format!("YOUR SCORE : {}", calc_score(& mut array));
                                self.draw_4(data.world, array);
                
                            } else {
                                self.score = String::from("YOU LOOSE");
                            }
                        }
                        None => {unimplemented!()}
                    }
                    return Trans::None;
                }else if is_key_down(&event, VirtualKeyCode::Left) {
                    println!("[Trans::None] Left");
                    match self.field.field_4 {
                        Some(field) => {
                            let mut array = field.clone();
                            if test_4(&mut array){
                                do_game_step_4(&Usermove::Left, &mut array);
                                spawn_4(&mut array);
                                self.score = format!("YOUR SCORE : {}", calc_score(& mut array));
                                self.draw_4(data.world, array);
                            } else {
                                self.score = String::from("YOU LOOSE");
                            }
                        }
                        None => {unimplemented!()}
                    }
                    return Trans::None;
                }else if is_key_down(&event, VirtualKeyCode::Up) {
                    println!("[Trans::None] Up");
                    match self.field.field_4 {
                        Some(field) => {
                            let mut array = field.clone();
                            if test_4(&mut array){
                                do_game_step_4(&Usermove::Up, &mut array);
                                spawn_4(&mut array);
                                self.draw_4(data.world, array);
                                self.score = format!("YOUR SCORE : {}", calc_score(& mut array));
                            } else {
                                self.score = String::from("YOU LOOSE");
                            }
                        }
                        None => {unimplemented!()}
                    }
                    return Trans::None;
                }else if is_key_down(&event, VirtualKeyCode::Down) {
                    println!("[Trans::None] Down");
                    match self.field.field_4 {
                        Some(field) => {
                            let mut array = field.clone();
                            if test_4(&mut array){
                                do_game_step_4(&Usermove::Down, &mut array);
                                spawn_4(&mut array);
                                self.score = format!("YOUR SCORE : {}", calc_score(& mut array));
                                self.draw_4(data.world, array);
                            } else {
                                self.score = String::from("YOU LOOSE");
                            }
                        }
                        None => {unimplemented!()}
                    }
                    return Trans::None;
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
                transform.set_translation_xyz(-280.0 + j as f32 * 120.0, 230.0 - i as f32 *100.0, 0.0);
   
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

fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
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



enum Usermove{
    Left,
    Right,
    Up,
    Down,
}

fn do_game_step_4(step : &Usermove, field:&mut [[u32; 4]; 4]){
    println!("im do my work");
    match *step {
        Usermove::Left =>{
            for array in field{
                for  col in 0..4 {
                    for testcol in (col+1)..4 {
                        if array[testcol] != 0 {
                            if array[col] == 0 {
                                array[col] += array[testcol];
                                array[testcol] = 0;
                            }
                            else if array[col] == array[testcol] {
                                array[col] += array[testcol];
                                array[testcol] = 0;
                                break;
                            } else {
                                break
                            }
                        }
                    }
                }
            }
        } ,
        Usermove::Right=>{
            for array in field{
                for  col in (0..4).rev() {
                    for testcol in (0..col).rev() {
                        if array[testcol] != 0 {
                            if array[col] == 0 {
                                array[col] += array[testcol];
                                array[testcol] = 0;
                            }
                            else if array[col] == array[testcol] {
                                array[col] += array[testcol];
                                array[testcol] = 0;
                                break;
                            }else {
                                break;
                            }
                        }
                    }
                }
            }
        } ,
        Usermove::Down   =>{
            for col in 0..4 {
                for row in (0..4).rev() {
                    for testrow in (0..row).rev() {
                        if field[testrow][col] != 0 {
                            if field[row][col] == 0 {
                                field[row][col] += field[testrow][col];
                                field[testrow][col] = 0;
                            } else if field[row][col] == field[testrow][col] {
                                field[row][col] += field[testrow][col];
                                field[testrow][col] = 0;
                                break;
                            }else {
                                break;
                            }
 
                        }
                    }
                }
            }
        } ,
        Usermove::Up =>{
            for col in 0..4 {
                for row in 0..4{
                    for testrow in (row+1)..4 {
                        if field[testrow][col] != 0 {
                            if field[row][col] == 0 {
                                field[row][col] += field[testrow][col];
                                field[testrow][col] = 0;
                            } else if field[row][col] == field[testrow][col] {
                                field[row][col] += field[testrow][col];
                                field[testrow][col] = 0;
                                break;
                            }else {
                                break;
                            }
                        }
                    }
                }
            }
        },
    }
}

fn spawn_4( field: &mut  [[u32;4];4]){
    println!("im spawning");
    let mut count = 0;
    'spawn_loop : loop{
        let x = rand::random::<usize>();
        if field[x % 4][(x/4)%4] == 0 {
            field[x % 4][(x/4)%4]= 2;
            break;
        }
        count +=1;
        if count > 100 {
            for i in 0..4 {
                for j in 0..4{
                    if field[i][j] == 0 {
                        count = 0;
                        continue 'spawn_loop;
                    }
                }
            }
            break;
        }
    }
}

fn test_4(field: &mut [[u32;4];4]) -> bool{
    let mut test=field.clone();
    println!("im testing");
    for i in [Usermove::Up,Usermove::Down,Usermove::Left,Usermove::Right].into_iter(){
        do_game_step_4(i, &mut test);
        if test != *field{
            return true;
        }
    }
    return false;
}

fn calc_score(field: &mut [[u32;4];4]) -> u32{
    println!("im calculating");
    let mut score = 0;
    for i in 0..4{
        for j in 0..4{
            score += field[i][j];
        }
    }
    return score;
}

