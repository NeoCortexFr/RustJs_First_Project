fn main() {
    let a : i32; // i32: integer
    /*Type entier: (en bits)
    i8, i16, i32, i64, i128 Entier signé (donc possible négatif)
    u8, u16, u32, u64, u128 Entier non signé (Impossible nombre négatif, démarre à 0)
    isize, usize -> vaut 32 ou 64 suivant l'architecture du pc (32 bits ou 64 bits)
    */
    let a = 5u32; // 5 en mode u32
    let b = 300_000; // possible de mettre un _ pour les grands nombres
    /*
    let mut a = 0u8;
    a = 255;
    a = a + 1;
    => ça fait 256, qui dépasse le u32 (qui vaut max 255), donc
    le run affiche une erreur overflow
    */

    /*Type float:
    f32, f64
    let a = 4.2; ou aussi let a = 5_000.3_001;
    */

    /*Type bool
    Stocké sur un octet (8bits = 1 byte) true ou false
    */

    /*Type Char:
    Stocké sur 4 octets (32 bits) unicode utf-8
    S'utilise avec les guillemets simples ' '
    */

    /*Type &str (string)
    String litteral
    let a = "hello world";

    "\" -> echappement de caractère
    
    */

    /*Type tuples
    let a = (5, 'b', 5.5, "hello");
    Ne peut être agrandi ou rétréci.
    let b = a.0;
    b = 5
    let b = a.3;
    b = "hello"

    destructuration:
    let (b,mut c,d,e) = a; (c sera mutable)
    => b : 5, c: b, d: 5.5, e: hello
    */

    /*Type array
    let a = [5,5,5,5];
    println!("Hello,{}", a[2]); pour acceder au 3ème élément du tableau
    let a = [5,5,5,5]; peut s'écrire 
    let a = [5;4];

    Tableau à 2 dimensions:
    let a = [[i32;4];3]; (Un tableau de 4 i32, 3x dans le tableau principal)

    Il est possible aussi:
    let a : i32 = 3;
    let b = [3,5,8,4,7];

    println!("{}", b[a as usize]);
    on se sert de la variable a comme index de recherche dans le
    tableau via "as"
    */

    /*Type shadowing et Result:
    let number = "42";
    On transforme ce "42" en type int
    let number = number.parse::<u32>();
    ou
    let number : u32 = number.parse();
    Renvoi le type result (qui renvoi ok ou Err) permet de vérifier s'il y a 
    une erreur (par exemple à la saisie utilisateur)
    ->
    let mut number = number.parse::<u32>.expect("Erreur lors du parse\n");
    // unwrap existe aussi
    */

    println!("Hello");
}
