

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot{
    lat: i32,
    long: i32,
    direzione: Direction,
}


const DIREZIONI: &[Direction] = &[
    Direction::North,
    Direction::East,
    Direction::South,
    Direction::West,
];

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Self{
            lat: x,
            long: y,
            direzione : d,
        }
    }

    #[must_use]
    pub fn turn_right(self) -> Self {
        let new_direction = (self.direzione as usize + 1) % DIREZIONI.len();
        Robot::new(self.lat, self.long, DIREZIONI[new_direction].clone())
        
        /*let direzioni = vec![Direction::North, Direction::East, Direction::South, Direction::West];
        if direzioni.iter().position(|x| *x == self.direzione).unwrap()== 3_usize{
            
            Robot::new(self.lat,self.long,direzioni[0].clone())
        }
        else{
            Robot::new(self.lat,self.long,direzioni[direzioni.iter().position(|x| *x == self.direzione).unwrap() + 1_usize].clone())
        }*/
        /*match self.direzione{
            Direction::North =>  Robot::new(self.lat,self.long,Direction::East),//self.direzione = Direction::East,
            Direction::East =>  Robot::new(self.lat,self.long,Direction::South),//self.direzione = Direction::South,
            Direction::South => Robot::new(self.lat,self.long,Direction::West),//self.direzione = Direction::West,
            Direction::West => Robot::new(self.lat,self.long,Direction::North),//self.direzione = Direction::North,
        }*/
    }

    #[must_use]
    pub fn turn_left(self) -> Self {
        let new_direction = (self.direzione as usize + DIREZIONI.len() - 1) % DIREZIONI.len();
        Robot::new(self.lat, self.long, DIREZIONI[new_direction].clone())
        /*let direzioni = vec![Direction::North, Direction::East, Direction::South, Direction::West];
        //let index = direzioni.iter().position(|x| *x == self.direzione).unwrap();
        if direzioni.iter().position(|x| *x == self.direzione).unwrap() == 0_usize{
            Robot::new(self.lat,self.long,direzioni[3].clone())
        }
        else{
        Robot::new(self.lat,self.long,direzioni[direzioni.iter().position(|x| *x == self.direzione).unwrap() - 1_usize].clone())
        }*/
        /*match self.direzione{
            Direction::North =>  Robot::new(self.lat,self.long,Direction::West),//self.direzione = Direction::East,
            Direction::East =>  Robot::new(self.lat,self.long,Direction::North),//self.direzione = Direction::South,
            Direction::South => Robot::new(self.lat,self.long,Direction::East),//self.direzione = Direction::West,
            Direction::West => Robot::new(self.lat,self.long,Direction::South),//self.direzione = Direction::North,
        }*/
    }

    #[must_use]
    pub fn advance(self) -> Self {
        
        let (lat, long) = match self.direzione {
            Direction::North => (self.lat, self.long + 1),
            Direction::East => (self.lat + 1, self.long),
            Direction::South => (self.lat, self.long - 1),
            Direction::West => (self.lat - 1, self.long),
        };
        Robot::new(lat, long, self.direzione)
        /*match self.direzione{
            Direction::North => Robot::new(self.lat,self.long +1,self.direzione),//self.long += 1,
            Direction::East => Robot::new(self.lat+1,self.long,self.direzione),//self.lat += 1,
            Direction::South => Robot::new(self.lat,self.long -1,self.direzione),//self.long += -1,
            Direction::West => Robot::new(self.lat-1,self.long ,self.direzione),//self.lat += -1,
        }*/
    }

    #[must_use]
    pub fn instructions(self, instructions: &str) -> Self {
        let mut robot = Robot::new(self.lat,self.long,self.direzione);
        for c in instructions.chars(){
            match c{
                'R' => robot = robot.turn_right(),
                'L' => robot = robot.turn_left(),
                'A' => robot = robot.advance(),
                _ => println!("errorerror"),
            };
        }
        robot
    }

    pub fn position(&self) -> (i32, i32) {
        (self.lat,self.long)
    }

    pub fn direction(&self) -> &Direction {
        &self.direzione
    }
}

