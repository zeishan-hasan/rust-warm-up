use rand::Rng;

fn roll_dice() -> u32 {
    let dice_1:u32 = rand::thread_rng().gen_range(1..6);
    let dice_2:u32 = rand::thread_rng().gen_range(1..6);
    let sum = dice_1 + dice_2;
    
    eprintln!("The player rolled: {dice_1} + {dice_2} = {sum}");
    sum
}

fn main() {
    #[derive(PartialEq)] // This is to perform binary operation '==' with the enum Status
    enum Status {CONTINUE, WON, LOST}
    let mut point: u32 = 0;
    let mut sum_of_dice: u32 = roll_dice();
    let mut game_status: Status;
    
    match sum_of_dice {
        7 | 11 => game_status = Status::WON,
        2 | 3 | 12 => game_status = Status::LOST,
        _ => {
            game_status = Status::CONTINUE;
            point = sum_of_dice;
            eprintln!("Player point: {point}");
        },
    }
    
    while Status::CONTINUE == game_status {
        sum_of_dice = roll_dice();
        if sum_of_dice == point {
            game_status = Status::WON;
        } else if sum_of_dice == 7 {
            game_status = Status::LOST;
        }
    }
    
    if Status::WON == game_status {
        eprintln!("Player won!");
    } else {
        eprintln!("Player lost!");
    }
}
