#[allow(dead_code)]
enum Sexo{
    Masculino,
    Feminino,
    Outro
}

fn check_grade(grade: f32)-> (){
    match grade{
        0.0...4.8 =>println!("Disapproved"),
        4.9...5.9 =>println!("Exam"),
        6.0...10.0 =>println!("Approved"),
        _ => println!("Invalid grade!!!"),
    }
}

fn check_number(number: i16)-> (){
    match number{
        0 => println!("zero"),
        2 | 3 | 5 | 7 | 11 => println!("prime"),
        _ => println!("Any number"),
    }
}

fn check_tuple(t:(i32,i32))-> (){
    match t{
        (0,_x) => println!("o primeiro é zero"),
        (_x,0) => println!("o segundo é zero"),
        _ => println!("nenhum é zero"),
    }
}

fn is_vogal_ou_consoante(c:char) -> char{
    match c{
        'a' | 'e' | 'i' | 'o' | 'u' => 'v',
        'A' | 'E' | 'I' | 'O' | 'U' => 'v',
        _ => 'c'
    }
}

fn main() {

    let grade_a = 0.0;
    let grade_b = 3.2;
    let grade_c = 5.1;
    let grade_d = 8.3;

    check_grade(grade_a);
    check_grade(grade_b);
    check_grade(grade_c);
    check_grade(grade_d);

    println!("\n");

    let number_a = 0;
    let number_b = 3;
    let number_c = 8;

    check_number(number_a);
    check_number(number_b);
    check_number(number_c);

    println!("\n");

    let sexo = Sexo::Feminino;

    match sexo{
        Sexo::Feminino=> println!("Feminino"),
        Sexo::Masculino=> println!("Masculino"),
        Sexo::Outro=> println!("outro"),
    }

    println!("\n");

    check_tuple((0,1));
    check_tuple((1,0));
    check_tuple((1,1));

    println!("\n");

    let name: &'static str = "pedro";
    let mut vogal_cont =0;
    let mut consoante_cont =0;

    for i in name.chars(){
        match is_vogal_ou_consoante(i){
            'v'=> vogal_cont+=1,
            'c'=> consoante_cont+=1,
            _ =>()
        }
    }

    println!("{} tem {} vogais(v) e {} consoantes(c)",name,vogal_cont,consoante_cont);

    println!("\n");

    for i in name.chars(){
        match is_vogal_ou_consoante(i){
            r @ 'v'=> println!("{}",r),
            r @ 'c'=> println!("{}",r),
            _ =>()
        }
    }
}