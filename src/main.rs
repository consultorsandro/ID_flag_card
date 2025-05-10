use std::io;

fn main() {
    // Solicita ao usuário que digite o número do cartão de crédito
    println!("Digite o número do cartão de crédito:");

    // Cria uma string para armazenar a entrada do usuário
    let mut input = String::new();

    // Lê a entrada do usuário
    io::stdin()
        .read_line(&mut input)
        .expect("Falha ao ler a linha");

    // Remove espaços em branco e verifica se a entrada contém apenas dígitos
    let card_number = input.trim();
    if !card_number.chars().all(char::is_numeric) {
        // Se a entrada não for numérica, exibe uma mensagem de erro
        println!("Por favor, digite apenas números.");
        return;
    }

    // Identifica a bandeira do cartão com base no primeiro dígito ou nos primeiros dígitos
    let card_flag = match card_number.chars().next() {
        Some('4') => "VISA", // Bandeira VISA começa com 4
        Some('5') => {
            // Bandeira MASTERCARD começa com 51 a 55
            if let Ok(prefix) = card_number[..2].parse::<u32>() {
                if (51..=55).contains(&prefix) {
                    "MASTERCARD"
                } else {
                    "Desconhecida"
                }
            } else {
                "Desconhecida"
            }
        }
        Some('3') => {
            // Bandeira AMEX começa com 34 ou 37
            if let Ok(prefix) = card_number[..2].parse::<u32>() {
                if prefix == 34 || prefix == 37 {
                    "AMEX"
                } else {
                    "Desconhecida"
                }
            } else {
                "Desconhecida"
            }
        }
        Some('6') => "DISCOVER", // Bandeira DISCOVER começa com 6
        _ => "Desconhecida", // Para outros casos, bandeira desconhecida
    };

    // Exibe a bandeira do cartão
    println!("[A bandeira do número do cartão é: {} .]", card_flag);
}

