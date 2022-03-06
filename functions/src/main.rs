fn main() {
    one(5,6);
    two();

    let three = three();
    println!("three retorna: {}", three);
}

fn one(x: i32,y:i64) {
    println!("O valor de x é: {}", x);
    println!("O valor de y é: {}", y);
}

fn two(){
    let x = 5;

    let y = {
        let x = 3;
        x + 1 //Não pode ter ponto e vírgula
    };

    println!("O valor de y é: {}", y); //4
}

fn three()->i32{

    let a = 1;
    let b = 2;
    a+b //Sem o ponto e vírgula o "return" fica implícito
}