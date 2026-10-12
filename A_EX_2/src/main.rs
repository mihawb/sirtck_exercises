use std::collections::BTreeMap;
use std::io;

/// Zamienia wejściowy tekst na wektor znaków zawierający tylko litery alfabetu.
/// Zwraca `Err("Brak danych")`, gdy po przetworzeniu brak liter.
pub fn normalize_letters(input: &str) -> Result<Vec<char>, String> {
    let letters: Vec<char> = input
        .trim()
        .chars()
        .map(|c| c.to_ascii_lowercase())
        .filter(|c| c.is_ascii_alphabetic())
        .collect();

    (!letters.is_empty())
        .then_some(letters)
        .ok_or_else(|| "Brak danych".to_string())
}

/// Buduje histogram wystąpień liter na podstawie wektora znaków.
pub fn build_histogram(letters: &[char]) -> BTreeMap<char, usize> {
    let mut histogram = BTreeMap::new();
    for c in letters {
        *histogram.entry(*c).or_insert(0) += 1;
    }
    histogram
}

/// Formatuje histogram do wypisania na stdout jako linie `litera: liczba`.
pub fn format_histogram(counts: &BTreeMap<char, usize>) -> Vec<String> {
    let mut output = Vec::new();
    for (k, v) in counts {
        output.push(format!("{}: {}", k, v));
    }
    output
}

/// Główna logika: przygotowuje wynik do wypisania lub zwraca komunikat o błędzie.
pub fn run_from_str(input: &str) -> Result<Vec<String>, String> {
    let normalized = normalize_letters(input)?;
    let histogram = build_histogram(&normalized);
    Ok(format_histogram(&histogram))
}

pub fn main() {
    let mut buffer = String::new();

    if io::stdin().read_line(&mut buffer).is_err() {
        println!("Brak danych");
        return;
    }

    match run_from_str(&buffer) {
        Ok(lines) => {
            for line in lines {
                println!("{}", line);
            }
        }
        Err(message) => println!("{}", message),
    }
}
