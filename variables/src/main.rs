fn main() {
    one();
    two();
    three();
}

fn one(){
    println!("One");
    let mut x = 5;
    println!("O valor de x é: {}", x);
    x = 6;
    println!("O valor de x é: {}", x);
    println!("\n");
}

fn two(){
    println!("Two");
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("O valor de x é: {}", x);
    println!("\n");
}

fn three(){
    let espacos = "   ";
    let espacos = espacos.len();
    println!("expacos: {}", espacos);

}

