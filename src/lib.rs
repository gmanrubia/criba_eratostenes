/// Devuelve un vector con todos los números primos menores o iguales a `limite`.
pub fn criba_eratostenes(limite: usize) -> Vec<usize> {
    if limite < 2 {
        return Vec::new();
    }

    // Creamos un vector donde el índice representa al número
    let mut es_primo = vec![true; limite + 1];
    es_primo[0] = false;
    es_primo[1] = false;

    let tope = (limite as f64).sqrt() as usize;

    for numero in 2..=tope {
        if es_primo[numero] {
            // Tachamos los múltiplos empezando desde numero * numero
            for multiplo in (numero * numero..=limite).step_by(numero) {
                es_primo[multiplo] = false;
            }
        }
    }

    // Recopilamos los índices que quedaron marcados como verdaderos
    es_primo
        .into_iter()
        .enumerate()
        .filter_map(|(num, primo)| if primo { Some(num) } else { None })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prueba_casos_base() {
        assert_eq!(criba_eratostenes(0), Vec::<usize>::new());
        assert_eq!(criba_eratostenes(1), Vec::<usize>::new());
        assert_eq!(criba_eratostenes(2), vec![2]);
    }

    #[test]
    fn prueba_hasta_treinta() {
        let esperados = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29];
        assert_eq!(criba_eratostenes(30), esperados);
    }
}
