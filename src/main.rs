use std::io;

//make a program that takes the input given by the user to do something

fn main(){
    let (mut Player_Mana, mut Player_Strength, mut Player_Agility, mut Player_Points) = (2, 0, 1, 3);
    let mut Player_TPoints = addPoints(Player_Mana, Player_Strength, Player_Agility);
    let (Type_Selected, Player_Class) = startGame();
    if Type_Selected{
        createPlayer(Player_Class);
    }
    if !Type_Selected{
        println!("Did you intentionally break the game? YES or NO??");
        let Breakee = getYorN();
        if Breakee{
            println!("You don't deserve a reboot!");
            return;
        }
        else if !Breakee{
            println!("No worries, it happens in when developing. Keep in mind this is your only one though!");
            startGame();
        }
    }


    
}

fn startGame() -> (bool, u32){
    println!("Welcome to the game! Select your build: \n 1. Mage 2. Barbarian 3. Theif");
    let answer = getnumber();
    if answer.eq(&1){
        println!("You have chosen mage, is this correct? \n(Y/N):");
        if getYorN(){
            println!("You are a mage!");
        }

    }
    else if answer.eq(&2){
        println!("You have chosen barbarian, is this correct? \n(Y/N):");
        if getYorN(){
            println!("You are a barbarian!");
        }

    }
    else if answer.eq(&3){
        println!("You have chosen theif, is this correct? \n(Y/N):");
        if getYorN(){
            println!("You are a theif!")
        }

    }
    else{
        println!("Not an option! Rebooting in 3... 2.. 1.");
        return (false, 0);
    }
    println!("Ready to create your character? (Y/N)");
    (getYorN(), answer)
}

fn createPlayer(Player_Class:u32){
    

    if Player_Class.eq(&1){
        println!("You gain +1 to your magic power but you have no strength");
        
    }

    println!("You have 3 points you can use to upgrade");
    println!("Stats are: \n1.Mana: {} \n2.Strength: {} \n3.Agility:{}", Player_Mana, Player_Strength, Player_Agility);  
    println!("Which would you like to upgrade first?");
    let choice = getnumber();
    if choice.eq(&1){
            
        println!("How many points would you like to add to Mana? | Current Mana: {}", Player_Mana);
        //upgradePlayer(Player_Mana);
    }


}

fn addPoints(Mana: u32, Strength: u32, Agility: u32) -> u32{
    Mana + Strength + Agility

}
fn getYorN() -> bool{
    
    let mut answer = String::new();
    let mut report: bool = false;
    io::stdin().read_line(&mut answer).expect("Yes or No");
    let answer = answer.trim().parse::<String>().unwrap();

    if answer.eq("Yes") || answer.eq("yes") || answer.eq("Y") || answer.eq("y"){
        report = true;
        //println!("You answered {} which == {}", answer, repo);
    }
    else if answer.eq("no") || answer.eq("No")|| answer.eq("N") || answer.eq("n"){
        report = false;
        //println!("You answered {} which == {}", answer, repo);
    }
    
    report
}

fn getnumber() -> u32{
        let mut guesser = String::new();
        println!("Give me a number");
        io::stdin().read_line(&mut guesser).expect("Type in a number NIMROD!");
        let guesser = guesser.trim().parse::<u32>().unwrap();
        guesser
}
