


fn main() {

    let x = 5;

    println!("IF\n");

    if x == 5{
        println!("x é cinco!");
    } else {
        println!("x não é cinco!");
    }

    
    let mut y = 0;

    println!("\nIF\n");

    if x == 5 {
        y = 10;
    }else{
        y =20;
    }

    println!("y é igual a {}",y);

    println!("\nIF\n");

    y = if x == 5 { 10 } else { 20 };

    println!("y é igual a {}",y);


    let mut num = 0;

    println!("\nWHILE\n");

    while num != 10 {
        num += 1;
        println!("{}", num);
    }

    num = 0;

    println!("\nFOR\n");

    for num in 0..10{
        println!("{}",num);
    }

    println!("\nFOR\n");

    for (index, value) in (5..10).enumerate(){
        println!("index {} e valor {}",index,value);
    }

    println!("\nFOR\n");

    let line = "um ficheiro\nMuito Interessant".lines();

    for (index, value) in line.enumerate(){
        println!("{}:{}",index,value);
    }

    println!("\nTERMINAR CICLO\n");

    num = 5;

    let mut done = false;

    while !done{
        num += -3;
        println!("{}",num);

        if num % 5 == 0{
            done = true;
        }
    }

    println!("\nTERMINAR CICLO\n");

    num =5;

    loop{
        num += -3;
        println!("{}", num);

        if num % 5 == 0 { break; }
    }

    println!("\nTERMINAR CICLO\n");

    for num in 0..11{
        if num % 2 != 0 { continue; }

        println!("{}",num);

    }

    println!("\nCICLOS COM ETIQUETA\n");

    'ciclo1: for x in 0..11 {
        'ciclo2: for y in 0..11{
            if x % 2 == 0 { continue 'ciclo1; }

            if y % 2 == 0 { continue 'ciclo2; }

            println!("> ({}{})",x,y);
        }
    }
    

    num = 0;
    
    println!("\nLOOP\n");

    loop{
        println!("{}",num);
        if num == 5{
            break;
        }
        num+=1;
    }
}
