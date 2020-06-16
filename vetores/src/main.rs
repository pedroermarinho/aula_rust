

fn main() {

    println!("\nVetor\n");

    let vactor = vec![1, 2, 3, 4, 5];
    let vactor2 = vec![0; 10];

    println!("4 -> {}",vactor[4]);

    println!("\nVetor\n");

    let i:usize=1;

    println!("{}",vactor[i]);

    println!("\nVetor\n");

    match vactor2.get(5){
        Some(x) => println!("{}",x),
        None => println!("Invalido")
    }

    match vactor2.get(20){
        Some(x) => println!("{}",x),
        None => println!("Invalido")
    }

    println!("\nInteração\n");

    let mut v = vec![1,2,3,4,5];

    for i in &v {
        println!("Referencia para {}",i);
    }

    for i in &mut v {
        println!("Referencia mutavel para {}",i);
    }

    for i in v{
        println!("Toma posse do vetor e dos seus elementos {}",i);
    }

    println!("\nInteração\n");


}
