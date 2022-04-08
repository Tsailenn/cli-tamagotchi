#[derive(Debug, Clone, Default)]
pub struct Tamagotchi {
    display: [[usize; 8]; 8],
    hunger: i8,
    health: i8,
    poisoning: i8,
    state: u8,
}

// 0b0000_0000
// dead, sick, hungry, poisoned _ dead bits

impl Tamagotchi {
    pub fn new() -> Self {
        Tamagotchi {
            hunger: 10,
            health: 10,
            poisoning: 10,
            ..Default::default()
        }
    }

    pub fn update(&mut self, hu: i8, he: i8, po: i8) {
        if (self.is_dead()) {
            return;
        }

        self.hunger = self.hunger.saturating_add(hu);
        self.health = self.health.saturating_add(he);
        if (he > 0) {
            self.add_poison();
        }
        self.poisoning = self.poisoning.saturating_add(po);

        //hunger check
        if self.hunger <= 0 {
            self.state = self.state | 0b1000_0000;
        } else if self.hunger < 5 {
            self.state = self.state | 0b0010_0000;
        } else {
            self.state = self.state & 0b1101_0000;
        }
        self.hunger = match self.hunger {
            n if n < 0 => 0,
            n if n > 10 => 10,
            n => n,
        };

        //health check
        if self.health <= 0 {
            self.state = self.state | 0b1000_0000;
        } else if self.health < 5 {
            self.state = self.state | 0b0100_0000;
        } else {
            self.state = self.state & 0b1011_1111;
        }
        self.health = match self.health {
            n if n < 0 => 0,
            n if n > 10 => 10,
            n => n,
        };

        //health check
        if self.poisoning <= 0 {
            self.state = self.state | 0b1000_0000;
        } else if self.poisoning < 5 {
            self.state = self.state | 0b0001_0000;
        } else {
            self.state = self.state & 0b1110_1111;
        }
        self.poisoning = match self.poisoning {
            n if n < 0 => 0,
            n if n > 10 => 10,
            n => n,
        };

        self.update_death();
        self.draw();
    }

    fn update_death(&mut self) {
        let si = (self.state & 0b0100_0000) >> 6;
        let hu = (self.state & 0b0010_0000) >> 5;
        let po = (self.state & 0b0001_0000) >> 4;

        if si + hu + po >= 2 {
            self.state = self.state | 0b1000_0000;
        }
    }

    fn add_poison(&mut self) {
        self.poisoning = self.poisoning.saturating_add(-3);
    }

    pub fn draw(&mut self) {
        /*
        non-dead states
         */
        if self.state & 0b0100_0000 != 0 {
            //draw sick, correlation: eyes
            self.display[2][1] = 0; //String::from("-");
            self.display[2][2] = 0; //String::from("-");
            self.display[3][1] = 1; //String::from("0");
            self.display[3][2] = 1; //String::from("0");

            self.display[2][5] = 0; //String::from("-");
            self.display[2][6] = 0; //String::from("-");
            self.display[3][5] = 1; //String::from("0");
            self.display[3][6] = 1; //String::from("0");
        } else {
            self.display[2][1] = 1; //String::from("0");
            self.display[2][2] = 1; //String::from("0");
            self.display[3][1] = 1; //String::from("0");
            self.display[3][2] = 1; //String::from("0");

            self.display[2][5] = 1; //String::from("0");
            self.display[2][6] = 1; //String::from("0");
            self.display[3][5] = 1; //String::from("0");
            self.display[3][6] = 1; //String::from("0");
        }

        if self.state & 0b0010_0000 != 0 {
            //draw hungry, correlation: mouth
            for i in 1..7 {
                self.display[5][i] = 1; //String::from("0");
            }

            for i in 2..6 {
                self.display[6][i] = 0; //String::from("-");
            }
        } else {
            self.display[5][1] = 1; //String::from("0");
            self.display[5][6] = 1; //String::from("0");

            for i in 2..6 {
                self.display[6][i] = 1; //String::from("0");
            }
        }

        if self.state & 0b0001_0000 != 0 {
            //draw poisoned, correlation: tongue
            self.display[6][3] = 1; //String::from("0");
            self.display[6][4] = 1; //String::from("0");

            self.display[7][3] = 1; //String::from("0");
            self.display[7][4] = 1; //String::from("0");
        } else {
            self.display[7][3] = 0; //String::from("-");
            self.display[7][4] = 0; //String::from("-");
        }
    }

    pub fn is_dead(&self) -> bool {
        self.state & 0b1000_0000 != 0
    }

    pub fn print(&self) {
        println!("hunger: {:#?}", self.hunger);
        println!("sick: {:#?}", self.health);
        println!("unpoison level: {:#?}", self.poisoning);

        for i in 0..self.display.len() {
            println!(
                "{:#?} {:#?} {:#?} {:#?} {:#?} {:#?} {:#?} {:#?}",
                self.display[i][0],
                self.display[i][1],
                self.display[i][2],
                self.display[i][3],
                self.display[i][4],
                self.display[i][5],
                self.display[i][6],
                self.display[i][7]
            );
        }
        println!("is dead: {:#?}", self.is_dead());
    }
}
/*
0 1 2 3 4 5 6 7
# # # # # # # # 0
# # # # # # # # 1
# 0 0 # # 0 0 # 2
# 0 0 # # 0 0 # 3
# # # # # # # # 4
# 0 # # # # 0 # 5
# # 0 0 0 0 # # 6
# # # # # # # # 7
*/
