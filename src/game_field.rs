use ron;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum FieldSize{
    Four,
    Six,
    Empty,
}

impl Default for FieldSize{
    fn default() -> FieldSize{
        FieldSize::Four
    }
}

#[derive( Default, Deserialize, Serialize, Debug)]
pub struct Field{
    pub skip: bool,
    pub hard: bool,
    pub loose: bool,
    pub score: u32,
    pub size: FieldSize,
    pub field_4 : Option<[[u32; 4]; 4]>,
    pub field_6 : Option<[[u32; 6]; 6]>,
}


impl Field {
    pub fn default(size: FieldSize) -> Field{
        match size {
            FieldSize::Four => {
                Field {
                    skip: false,
                    hard : false,
                    loose : false,
                    score : 0,
                    size : FieldSize::Four,
                    field_4: Some ([[0,0,0,0],[0,0,0,0],[0,0,0,0],[0,0,0,2]]),
                    field_6: None,
                }
            }

            FieldSize::Six =>{
                Field {
                    skip: false,
                    hard : false,
                    loose : false,
                    score : 0,
                    size : FieldSize::Four,
                    field_4: None,
                    field_6: Some([
                        [0,0,0,0,0,0],
                        [0,0,0,0,0,0],
                        [0,0,0,0,0,0],
                        [0,0,0,0,0,0],
                        [0,0,0,0,0,0],
                        [0,0,0,0,0,2],
                    ]),
                }
            }
            
            FieldSize::Empty => Field::empty(),
        }
    }

    pub fn save(&mut self, file_name: &str){
        let ron_str = ron::ser::to_string_pretty(self, ron::ser::PrettyConfig::default()).expect("Some trouble with serialization field");
        std::fs::write(file_name, ron_str).expect("Some error with saving field");
    }

    pub fn read( file_name: &str) -> Field{
        ron::de::from_str(&std::fs::read_to_string(file_name).unwrap()).unwrap()
    }

    pub fn empty() -> Field {
        Field{
            skip: false,
            hard: false,
            loose: false,
            score: 0,
            size : FieldSize::Empty,
            field_4 : None,
            field_6 : None,
        }
    }    

}



pub enum Usermove{
    Left,
    Right,
    Up,
    Down,
}

pub fn do_game_step_6(step : &Usermove, field:&mut [[u32; 6]; 6]){
    match *step {
        Usermove::Left =>{
            for array in field{
                for  col in 0..6 {
                    for testcol in (col+1)..6 {
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
                for  col in (0..6).rev() {
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
            for col in 0..6 {
                for row in (0..6).rev() {
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
            for col in 0..6 {
                for row in 0..6{
                    for testrow in (row+1)..6 {
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

pub fn do_game_step_4(step : &Usermove, field:&mut [[u32; 4]; 4]){

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

pub fn spawn_6( field: &mut  [[u32;6];6]){
    
    let mut count = 0;
    'spawn_loop : loop{
        let x = rand::random::<usize>();
        if field[x % 6][(x/6)%6] == 0 {
            field[x % 6][(x/6)%6]= 2;
            break;
        }
        count +=1;
        if count > 100 {
            for i in 0..6 {
                for j in 0..6{
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

pub fn spawn_4( field: &mut  [[u32;4];4]){
   
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

//check did u loose or not
pub fn test_6(field: &mut [[u32;6];6]) -> bool{
    let mut test=field.clone();
    
    for i in [Usermove::Up,Usermove::Down,Usermove::Left,Usermove::Right].iter(){
        do_game_step_6(i, &mut test);
        if test != *field{
            return true;
        }
    }
    return false;
}

//check did u loose or not
pub fn test_4(field: &mut [[u32;4];4]) -> bool{
    let mut test=field.clone();
    
    for i in [Usermove::Up,Usermove::Down,Usermove::Left,Usermove::Right].iter(){
        do_game_step_4(i, &mut test);
        if test != *field{
            return true;
        }
    }
    return false;
}

//calculate score on 4x4 field
pub fn calc_score_4(field: &mut [[u32;4];4]) -> u32{
    
    let mut score = 0;
    for i in 0..4{
        for j in 0..4{
            score += field[i][j];
        }
    }
    return score;
}

//calculate score on 6x6 field
pub fn calc_score_6(field: &mut [[u32;6];6]) -> u32{
    
    let mut score = 0;
    for i in 0..6{
        for j in 0..6{
            score += field[i][j];
        }
    }
    return score;
}