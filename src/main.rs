use std::io;


fn main() {

    println!("Devine le nombre !"); 
    println!("Veuillez rentrer une valeur :");

    let mut guess = String::new();
    
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);

}
