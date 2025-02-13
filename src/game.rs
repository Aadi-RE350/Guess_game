use rand::random_range;

pub enum Level {
    Easy,
    Medium,
    Hard,
}

pub struct Guess{
    pub x : i32,
    level : Level,
}

impl Guess {
    pub fn new(level:i32)->Self{
        let diffculty = {
            match level {
                1 => Level::Easy,
                2 => Level::Medium,
                3 => Level::Hard,
                _ => {
                        println!("\n\nInvalid choice! Defaulting to Easy. Select 1 (Easy), 2 (Medium), or 3 (Hard).");
                        Level::Easy
                    },
            }
        };
        Guess{
            x : random_range(0..=100),
            level : diffculty
        }
    }

    pub fn check(&self,guess:i32,chances:&i32)->bool{
        if self.x == guess {
            println!("Congratulations! You guessed the correct number ({} attempts left).",chances);
            true
        }
        else if self.x < guess {
            println!("\n\nIncorrect! The number is less than {}",guess);
            false
        }
        else {
            println!("\n\nIncorrect! The number is greater than {}",guess);
            false
        }
    }

    pub fn print_level(&self) -> &str{
        match self.level {
            Level::Easy => "\n\nGreat! You have selected the Easy difficulty level.\n\n",
            Level::Medium => "\n\nGreat! You have selected the Medium difficulty level.\n\n",
            Level::Hard => "\n\nGreat! You have selected the Hard difficulty level.\n\n",
        }
    }
    
}