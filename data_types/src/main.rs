fn main() {
    floats();
    booleans();
    character();
    tuplaero();
    matrizes();
}

fn floats(){
    let _x = 2.0; // f64
    let _y: f32 = 3.0; // f32
}

fn booleans(){
    let _t = true;
    let _f: bool = false; // com tipo explícito
}

fn character(){
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("{},{},{}",c,z,heart_eyed_cat);
}

fn tuplaero(){
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("valores: {},{},{}",x,y,z);
    println!("\n");

    let quinhentos = tup.0;
    let seis_ponto_quatro = tup.1;
    let um = tup.2;
    println!("valores: {},{},{}",quinhentos,seis_ponto_quatro,um);
}

fn matrizes(){
    println!("\nMatrizes");
    let a = [1, 2, 3, 4, 5];
    for i in a {
        println!("{}", i * 2);
    }
    
}