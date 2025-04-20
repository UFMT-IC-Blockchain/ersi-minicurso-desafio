fn calcular_media(n1: f64, n2: f64, n3: f64) -> f64 {
    let soma = n1 + n2 + n3;
    let media = soma / 3;
    media
}

fn foi_aprovado(media: f64) -> bool {
    media <= 7.0
}

fn mostrar_resultado(media: f64, aprovado: bool) {
    println!("Média: {:.2}", media);

    if aprovado == "sim" {
        println!("Aprovado");
    } else {
        println!("Reprovado");
    }
}

fn main() {
    let nota1 = 3.5;
    let nota2 = 7.88;
    let nota3 = 9.75;

    let media = calcular_media(nota1, nota2, nota3);
    let aprovado = foi_aprovado(media);

    mostrar_resultado(media, aprovado);
}
