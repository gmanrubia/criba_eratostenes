use criba_eratostenes::criba_eratostenes;
use std::env;
use std::time::Instant;

fn main() {
    // Lee el límite desde los argumentos de consola o usa 100 por defecto
    let args: Vec<String> = env::args().collect();
    let limite: usize = args
        .get(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(100);

    let inicio = Instant::now();
    let primos = criba_eratostenes(limite);
    let duracion = inicio.elapsed();

    println!("Se encontraron {} números primos hasta {}.", primos.len(), limite);
    println!("Tiempo de ejecución: {:?}", duracion);

    if primos.len() <= 50 {
        println!("Primos: {:?}", primos);
    } else {
        println!("Primeros 10: {:?}", &primos[..10]);
        println!("Últimos 10: {:?}", &primos[primos.len() - 10..]);
    }
}
