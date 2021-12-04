fn main() {
    /*
    Les variables, comme en JS, commence
    par une lettre ou un _ mais pas par un chiffre.
    Les chiffres sont autorisés dans le nom de la variable.
    */
    let age;
    age = 40;
    if age > 30 {
        println! ("Hello les vieux, vous avez {} ans", age);
    } else {
        println! ("Hello les jeunes, vous avez {} ans", age);
    }
}
