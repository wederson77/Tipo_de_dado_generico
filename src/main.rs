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

    let array_a: Vec<i32> = vec![1, 2, 3, 4, 5];
    let array_b: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("O vetor maior é: {:?}", maior(array_a, array_b));

    let a_float: f32 = 55.55;
    let b_float: f32 = 77.77;
    println!("O float maior é: {}", maior(a_float, b_float));

    let a_float_i32: f32 = 55.55;
    let b_str: &str = "77.77";
    println!("O dado maior é: {}", maior(a_float_i32, b_str));
}