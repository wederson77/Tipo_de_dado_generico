// Define uma função genérica chamada `maior` que aceita dois parâmetros do mesmo tipo genérico `T`.
// A função retorna um valor do mesmo tipo `T`.
// O `where T: PartialOrd` especifica que o tipo `T` deve implementar o trait `PartialOrd`,
// que permite a comparação entre valores para determinar se um é maior, menor ou igual ao outro.
fn maior<T>(a: T, b: T) -> T 
where T: PartialOrd {
    // Se `a` for maior que `b`, retorna `a`.
    if a > b {
        a
    } else {
        // Caso contrário, retorna `b`.
        b
    }
}

fn main() {
    // Define duas variáveis inteiras `x` e `y` com valores 5 e 10.
    let x: i32 = 5;
    let y: i32 = 10;
    // Chama a função `maior` com `x` e `y` e imprime o maior número.
    println!("O maior número é: {}", maior(x, y));

    // Define duas variáveis de string slices `a` e `b` com os valores "hello" e "wenderson".
    let a: &str = "hello"; 
    let b: &str = "wenderson"; 
    // Chama a função `maior` com `a` e `b` e imprime a maior string em termos lexicográficos.
    println!("A maior String é: {}", maior(a, b));
}