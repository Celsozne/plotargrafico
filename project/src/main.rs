mod functions; // Importa o arquivo functions.rs

use functions::{Divisao, Multiplicacao, Potencia, Raiz, Resultado, Soma, Subtracao};
use std::io::{self, Write};

fn main() {
    println!("--- Calculadora em Rust ---");

    loop {
        println!("\nEscolha a operação:");
        println!("1. Soma (+)");
        println!("2. Subtração (-)");
        println!("3. Multiplicação (*)");
        println!("4. Divisão (/)");
        println!("5. Potência (^)");
        println!("6. Raiz (x√y)");
        println!("0. Sair");
        print!("Opção: ");
        io::stdout().flush().unwrap();

        let mut escolha = String::new();
        io::stdin()
            .read_line(&mut escolha)
            .expect("Erro ao ler opção");
        let escolha = escolha.trim();

        if escolha == "0" {
            break;
        }

        // Pegar números do usuário
        let n1 = ler_numero("Digite o primeiro número: ");
        let n2 = ler_numero("Digite o segundo número: ");

        // Decidir qual struct instanciar
        // Box<dyn Resultado> permite armazenar qualquer struct que implemente Resultado
        let operacao: Box<dyn Resultado> = match escolha {
            "1" => Box::new(Soma {}),
            "2" => Box::new(Subtracao {}),
            "3" => Box::new(Multiplicacao {}),
            "4" => Box::new(Divisao {}),
            "5" => Box::new(Potencia {}),
            "6" => Box::new(Raiz {}),
            _ => {
                println!("Opção inválida!");
                continue;
            }
        };

        let res = operacao.calcular(n1, n2);

        if res.is_nan() {
            println!("Resultado: Erro matemático (Divisão por zero ou Raiz inválida)");
        } else {
            println!("Resultado: {}", res);
        }
    }
}

// Função auxiliar para ler números do teclado
fn ler_numero(prompt: &str) -> f64 {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Erro ao ler número");

        match input.trim().parse::<f64>() {
            Ok(num) => return num,
            Err(_) => println!("Por favor, digite um número válido."),
        }
    }
}
