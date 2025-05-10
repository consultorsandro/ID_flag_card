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

    // Identifica a bandeira do cartão com base no primeiro ou nos primeiros dígitos
    let card_flag = match card_number {
        _ if card_number.starts_with('4') => "VISA", // Bandeira VISA começa com 4
        _ if card_number.len() >= 2 && (51..=55).contains(&card_number[..2].parse::<u32>().unwrap_or(0)) => "Mastercard", // Mastercard entre 51 e 55
        _ if card_number.len() >= 4 && (2221..=2720).contains(&card_number[..4].parse::<u32>().unwrap_or(0)) => "Mastercard", // Mastercard entre 2221 e 2720
        _ if card_number.starts_with("4011") || card_number.starts_with("4312") || card_number.starts_with("4389") => "ELO", // ELO
        _ if card_number.len() >= 2 && (card_number.starts_with("34") || card_number.starts_with("37")) => "American Express", // American Express
        _ if card_number.starts_with("6011") || card_number.starts_with("65") || (644..=649).contains(&card_number[..3].parse::<u32>().unwrap_or(0)) => "Discover", // Discover
        _ if card_number.starts_with("6062") => "Hipercard", // Hpercard
        _ if card_number.len() >= 2 && (card_number.starts_with("30") || card_number.starts_with("36")) => "Diners Club", // Diners Club
        _ if card_number.len() >= 4 && (card_number.starts_with("2014") || card_number.starts_with("2149")) => "EnRoute", // EnRoute
        _ if card_number.starts_with("35") => "JCB", // JCB
        _ if card_number.starts_with("86") => "VOYAGER", // VOYAGER
        _ if card_number.starts_with("50") => "Aura", // Aura
        _ => "Desconhecida", // Para outros casos, bandeira desconhecida
    };

    // Exibe a bandeira do cartão
    println!("[A bandeira do número do cartão é: {} .]", card_flag);
}
