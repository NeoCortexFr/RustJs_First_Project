fn main() {
    /*
    Les variables, comme en JS, commence
    par une lettre ou un _ mais pas par un chiffre.
    Les chiffres sont autorisés dans le nom de la variable.
    */

    // let age = 40;

    // Par defaut, les variables sont imutables !
    // Pour lui donner la mutabilité, il faut ajouter mut
    // -> let mut age = 40;
    let mut age = 20;
    age = age + 1;
    if age > 30 {
        println! ("Hello les vieux, vous avez {} ans", age);
    } else {
        println! ("Hello les jeunes, vous avez {} ans", age);
    }
    // shadowing :
    // let age = age + 1;
    // (permet de changer le type de la variable)
    let age = "20";
    println! ("Vous avez maintenant {} ans", age);
}
