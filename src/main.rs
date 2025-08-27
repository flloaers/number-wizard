use rand::Rng;
use std::io;
use colored::*;

#[derive(Debug, Clone, Copy)]
enum Temperature {
    Freezing, 
    Cold, 
    Cool, 
    Warm,
    Hot, 
    Burning, 
    OnFire
}

impl Temperature {
    // Convert distance to temperature
    fn from_distance(distance: u32) -> Self {
        match distance {
            0 => Temperature::OnFire,       // Exact match
            1..=2 => Temperature::Burning,  // Very close
            3..=5 => Temperature::Hot,      // Close
            6..=10 => Temperature::Warm,    // Getting there
            11..=20 => Temperature::Cool,   // Somewhat not too far
            21..=50 => Temperature::Cold,   // Not very close 
            _ => Temperature::Freezing,     // Far away 
        }
    }

    fn message(&self) -> &str {
        match self {
            Temperature::Freezing => "🥶 Brrr! Colder than my breakfast porridge after a night in the blizzard!",
            Temperature::Cold => "❄️ Icy! You're sniffing the wrong cauldron, heh-heh!", 
            Temperature::Cool => "😐 Meh... Lukewarm. You'll need more than luck.", 
            Temperature::Warm => "🙂 Oooh, you're cooking now, Adventurer!",
            Temperature::Hot => "🥵 Ha! You're almost there, but not quite!",
            Temperature::Burning => "🔥 Aaah! So close I can almost smell victory!",
            Temperature::OnFire => "🎆 ON FIRE! The numbers explode in awe!",
        }
    }

    fn color_guess(&self, guess: u32) -> ColoredString {
        let text = guess.to_string();
        match self {
            Temperature::Freezing => text.cyan(),
            Temperature::Cold => text.blue(),
            Temperature::Cool => text.white(),
            Temperature::Warm => text.bright_yellow(),
            Temperature::Hot => text.yellow(),
            Temperature::Burning => text.red(),
            Temperature::OnFire => text.bright_red().bold(),
        }
    }
}

struct Game {
    target: u32, 
    min: u32,
    max: u32,
    attempts: u32,
    max_attempts: u32,
    guesses: Vec<u32>,
}

impl Game {
    fn new(min: u32, max: u32, max_attempts: u32) -> Self {
        let target = rand::rng().random_range(min..=max);
        // rand::rng creates a random number generator (RNG) local to the current thread.
        // No need to seed it manually.
        
        // Debug print
        //println!("Debug: target is {}", target);

        Self {
            target, 
            min, 
            max, 
            attempts: 0, 
            max_attempts,
            guesses: Vec::new(),
        }
    }

    fn make_guess(&mut self, guess: u32) -> Temperature {
        self.attempts += 1;
        self.guesses.push(guess);

        let distance = guess.abs_diff(self.target);

        Temperature::from_distance(distance)
    }

    fn print_guesses(&self) {
        if self.guesses.is_empty() { return; }

        print!("📍 Your guesses so far: ");
        for &g in &self.guesses {
            let temp = Temperature::from_distance(g.abs_diff(self.target));
            print!("{} ", temp.color_guess(g));
        }
        println!();
    }

    fn is_game_over(&self) -> bool {
        self.attempts >= self.max_attempts
    }

    fn attempts_left(&self) -> u32 {
        self.max_attempts - self.attempts 
    }

    fn attempts_warning(&self) -> Option<&str> {
        match self.attempts_left() {
            1 => Some("🚨 Last chance, Adventurer! You wouldn't want to disappoint the Number Wizard, would you?"),
            2 => Some("⚠️ Only 2 attempts left! The suspense sure is tickling my nose!"),
            3 => Some("⏰ Just 3 left! The numbers are getting restless!"),
            _ => None,
        }
    }

    fn desperate_hint(&self) {
        if self.attempts_left() == 2 {
            println!("🔮 Psst... The number rhymes with... No, no, I cannot tell you that!");
        } else if self.attempts_left() == 1 {
            if self.target % 2 == 0 {
                println!("🧙 It's... it's an even number. Don't tell anyone I told you!");
            } else {
                println!("🧙 It's... it's an odd number. This stays between us!");
            }
        }
    }

    fn victory_message(&self) -> &str {
        match self.attempts {
            1 => "🤯 UNBELIEVABLE! You got it in ONE try! Are you a wizard too?!",
            2..=3 => "🧙 Extraordinary! Would you consider becoming my apprentice, Adventurer?",
            4..=6 => "👏 Well done! You've impressed the Number Wizard!",
            7..=9 => "⌛️ Phew! You made it, but just barely!",
            x if x == self.max_attempts => "😅 Whew! You snatched victory from the jaws of defeat!",
            _ => "🎉 Congratulations, Adventurer! You found my darling number!.",
        }
    }
}


fn get_player_input(min: u32, max: u32) -> Result<u32, String> {
    println!("🧙 Enter your guess, Adventurer: ");

    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .map_err(|_| "Huh? Magical interference! Try again, Adventurer!".to_string())?;
        // .map_err converts the error from std::io::Error to String. 
        // ? says: if the result is Ok, unwrap it & continue; it if is Err, return it immediately from the function.

    let guess = input.trim().parse::<u32>()
        .map_err(|_| "Meh, that's not even a number! Focus, Adventurer!".to_string())?;

    if guess < min || guess > max {
        return Err(format!("Know your bounds, Adventurer! Your guess must lie between {} and {}.", min, max));
    }

    Ok(guess)
}


fn print_separator_line(repeat_count: usize) {
    let s = ". ܁₊ ⊹ . ܁˖ . ܁";
    println!("{}", s.repeat(repeat_count));
}


fn main() {

    println!(
    r#"
    ⊹ ۪ 𖥔 ˑ ִ ֗ 𝟘 𖥔 𝟙 ִ ۫ ˑ⊹ ۪  ﾟ｡⋆☾  𝟚 𖥔 𝟛 ִ ֗ ִ ۫ ˑ｡⊹ ۪ 𖥔 ˑ ִ 𝟜 ִ ۫ ˑ


                  THE NUMBER WIZARD
                

      ⊹ ۪ 𖥔 ˑ ִ 𝟝 ִ ֗ 𝟞 𖥔 𝟟 ִ ۫ ˑ⊹ ۪  ﾟ｡⋆☾  𝟠 𖥔 𝟡 ִ ֗ ִ ۫ ˑ｡⊹ ۪ 𖥔 ˑ
    "#
    );


    let min = 0;
    let max = 99;
    let max_attempts = 10;


    println!("🧙 Ah-ha, hello there, Adventurer...");
    println!("Let me guess...\nYou came aaaaaaall this way to see the Number Wizard, right?");
    println!("That's me!");
    println!("✨ Tamer of Numbers!");
    println!("✨ Grand Imperial Cypherer!");
    println!("✨ Hoarder of Calculus Grimoires!");
    println!("✨ The One and Only Number Wizard!");
    println!("Oooh, and let me guess again...\nYou want to play my mind-blowing little *guessing game*, don't you? Heh-heh...");
    println!("I'm good at this, arent'I? Almost like I'm the *Master of Guessing*, heh-heh!");
    println!("Well then, let's see if YOU can guess as well as I do, Adventurer...\n");
    println!("💭 ...\n");
    println!("Right, I'm thinking of a lovely number. This darling lies between {} and {}.", min, max);
    println!("You have {} attempts. Good luck, Adventurer!", max_attempts);

    let mut game = Game::new(min, max, max_attempts);

    loop {
        println!("⏳ Attempts left: {}", game.attempts_left());

        // Mounting pressure: Display warning messages for low attempts.
        if let Some(warning) = game.attempts_warning() {
            println!("{}", warning);
        }

        game.print_guesses();

        // Show desperate hints when nearly out of attemps 
        game.desperate_hint();

        let guess = match get_player_input(game.min, game.max) {
            Ok(guess) => guess, 
            Err(error) => {
                println!("❌ {}\n", error);
                continue;
            }
        };

        let temperature = game.make_guess(guess);
        println!("{}", temperature.message());

        if matches!(temperature, Temperature::OnFire) {
            println!("{}", game.victory_message());
            break;
        }

        if game.is_game_over() {
            println!("🙀 Game over! My darling number was {}.", game.target);
            println!("🧙 Don't Worry, Adventurer. Even the greatest wizards need practice!");
            break;
        }

    print_separator_line(5);

    }


}
