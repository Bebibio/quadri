use std::io;

enum Quadrilatere {
    Carre,
    Rectangle,
    Losange,
    Parallelogramme,
    Quelquonque
}


struct Point {
    x: f32,
    y: f32
}
impl Point {
    fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y
        }
    }
}

struct Vector<'a> {
    a: &'a Point,
    b: &'a Point,
    x: f32,
    y: f32,
    len: f32,
}
impl<'a> Vector<'a> {
    fn new(a: &'a Point, b: &'a Point) -> Self {
        let x = b.x-a.x;
        let y = b.y-a.y;
        let len = (x.powi(2)+y.powi(2)).sqrt();
        Vector {
            a: &a,
            b: &b,
            x,
            y,
            len
        }
    }
}

fn what_is_this(a: Point, b: Point, c: Point, d: Point) -> Quadrilatere {
    //Vecteur de trois cotes
    let ab = Vector::new(&a, &b);
    let bc = Vector::new(&b, &c);
    let dc = Vector::new(&d, &c);

    //Vecteur des diagonales
    let ac = Vector::new(&a, &c);
    let bd = Vector::new(&b, &d);

    if ab.x == dc.x && ab.y == dc.y {
        if ac.len == bd.len {
            if ab.len == bc.len {
                return Quadrilatere::Carre
            } else {
                return Quadrilatere::Rectangle
            }
        } else if ab.len == bc.len {
            if ac.len == bd.len {
                return Quadrilatere::Carre
            } else {
                return Quadrilatere::Losange
            }
        } else {
            return Quadrilatere::Parallelogramme
        }
    } else {
        return Quadrilatere::Quelquonque
    }
}

fn ask_coords(nbr: i8) -> Point {
    loop {
        println!("Entrer le point {} selon le format suivant \"x,y\": ", nbr);
        let mut input = String::new();
        loop {
            match io::stdin().read_line(&mut input) {
                Err(_) => {},
                Ok(_) => {
                    let inputs_str: Vec<&str> = input.split(',').collect();
                    let mut inputs : Vec<f32> = Vec::new();
                    for i in inputs_str {
                        let result = i.trim().parse::<f32>();
                        inputs.push(match result {
                            Ok(n) => n,
                            Err(_) => {
                                println!("Quelque chose a raté lors de la saisie des coordonées.");
                                break
                            }
                        });
                    }
                    if inputs.len() == 2 {
                        let a = Point::new(inputs[0], inputs[1]);
                        return a
                    } else if inputs.len() < 2 {
                        println!("Le nombre de coordonnées est trop faible.");
                        break
                    } else {
                        println!("Le nombre de coordonnées est trop important.");
                        break
                    }
                }
            }
        }
    }
}

fn main() {
    let a = ask_coords(1);
    let b = ask_coords(2);
    let c = ask_coords(3);
    let d = ask_coords(4);

    let type_ = what_is_this(a, b, c, d);
    match type_ {
        Quadrilatere::Carre => println!("Ceci est un carré"),
        Quadrilatere::Rectangle => println!("Ceci est un rectangle"),
        Quadrilatere::Losange => println!("Ceci est un losange"),
        Quadrilatere::Parallelogramme => println!("Ceci est un parallélogramme"),
        Quadrilatere::Quelquonque => println!("Ceci est un quadrilatère quelquonque")
    }

    println!("Entrer pour fermer");
    io::stdin().read_line(&mut String::new()).unwrap();
}