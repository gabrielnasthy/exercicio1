// Disciplina : Linguagem e Lógica de Programação
// Professor : Alan Furukawa
// Descrição : Aqui você descreve o que o programa faz! (função)
// Autor(a) : Gabriel Aguiar Rocha
// Data atual : 04/10/2021
use std::io;
fn ler() -> f64 {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("falha ao ler a linha");
    input.trim().parse::<f64>().unwrap()
}
fn ler_int() -> i32 {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("falha ao ler a linha");
    input.trim().parse::<i32>().unwrap()
}
fn main() {
    const PI: f64 = 3.14;;
    println!("--------| calculadora de area |---------");
    println!("digite 1 para calcular a area do circulo");
    println!("digite 2 para calcular a area do quadrao");
    let mut opcao = ler_int();

    if opcao == 1 {
        println!("qual o raio do circulo que deseja calcular:");
        let num = ler();
        println!("a area eh igual {:?}", area_circulo(PI, num));
    } else if opcao == 2 {
        println!("digite o tamanho lado 1:");
        let mut lado1 = ler_int();
        println!("digite o tamanho lado 2:");
        let mut lado2 = ler_int();
        println!(
            "a area do quadrado é igual a {:?}",
            area_quadrado(lado1, lado2)
        )
    } else {
        println!("não tem essa opcao");
    }
}
fn area_circulo(pi: f64, r: f64) -> f64 {
    let mut area;
    area = (r * pi);
    area
}

fn area_quadrado(lado1: i32, lado2: i32) -> i32 {
    let mut area;
    area = (lado1) * (lado2);
    area
}
