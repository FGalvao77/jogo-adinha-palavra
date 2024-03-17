// PROJETO: Jogo da forca com Rust
// AUTOR: Fernando Galvão
// DATA: 17/03/2024

// Importando os módulos necessários
use std::io;
use std::io::Write;
use rand::Rng;

// Função principal
fn main() {

    println!("\n#### JOGO: Adivinhe a palavra ####");
    println!("====================================");
    // Definindo uma palavra de teste
    // let word = "pera";
  
    // Lista de palavras
    let words = vec!["banana", "abacaxi", "laranja", "morango", "uva", "melancia", "pera", "abacate", "manga", "kiwi", "graviola", "caju", "tangerina", "ameixa", "maracuja", "acerola"];

    // Realizando o sorteio aleatório da palavra da lista
    let word = words[rand::thread_rng().gen_range(0..words.len())];
    // Imprimindo a quantidade de letras da palavra sorteada
    println!("A palavra tem {} letras", word.chars().count());
    
    // let word = words[rand::random::<usize>() % words.len()];
  
    // Definindo a quantidade de tentativas: quantidade de letras da palavra sorteada + duas tentativas extras
    let max_try = word.len() + 2;

    let mut letters_tried = vec![];
       
    let mut try_remaining = max_try;

    loop {
      let current_word = get_current_word(word, &letters_tried);
      println!("Palavra: {}\n", current_word);
       
      if current_word == word {
        println!("\nPARABÉNS. Você ganhou!!! ;)\n\n");
        break;
      }

      if try_remaining == 0 {
        println!("Infelizemente! Você perdeu! :( ");
        println!("A palavra era: {}", word);
        break;
      }

      print!("Digite uma letra: ");
      
      io::stdout().flush().unwrap();
      let mut letter = String::new();
      
      io::stdin().read_line(&mut letter).unwrap();
      let letter = letter.trim().to_lowercase();

      if letters_tried.contains(&letter) {
        println!("Você já tentou essa letra. Tente outra, por favor!");
        continue;
      }

      letters_tried.push(letter.clone());
      try_remaining -= 1;

      if !word.contains(&letter) {
        println!("Letra não encontrada. Você tem {} tentativas restantes.", try_remaining);
        continue;
      }
      
      println!("Letra encontrada!");
    }
}

fn get_current_word(word: &str, letters_tried: &[String]) -> String {
    let mut current_word = String::new();

    for letter in word.chars() {
      if letters_tried.contains(&letter.to_string()) {
        current_word.push(letter);
      } else {
        current_word.push('_');
      }
    }
  
    current_word
}





