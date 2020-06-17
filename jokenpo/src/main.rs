#[derive(Debug, PartialEq)]
enum Jogada{
    PEDRA,
    PAPEL,
    TESOURA
}

#[derive(Debug, PartialEq)]
enum Resultado{
    GANHOU(Jogada),
    EMPATE
}

fn jokenpo(a:Jogada, b:Jogada) -> Resultado{
    match (a, b){
        (Jogada::PEDRA,Jogada::PAPEL) | (Jogada::PAPEL,Jogada::PEDRA) => Resultado::GANHOU(Jogada::PAPEL),
        (Jogada::PEDRA,Jogada::TESOURA) | (Jogada::TESOURA,Jogada::PEDRA) => Resultado::GANHOU(Jogada::PEDRA),
        (Jogada::TESOURA,Jogada::PAPEL) | (Jogada::PAPEL,Jogada::TESOURA) => Resultado::GANHOU(Jogada::TESOURA),
        (_,_) => Resultado::EMPATE
    }
}

#[test]
fn teste_jokenpo(){

    assert_eq!(jokenpo(Jogada::PEDRA,Jogada::PAPEL),Resultado::GANHOU(Jogada::PAPEL));
    assert_eq!(jokenpo(Jogada::PEDRA,Jogada::TESOURA),Resultado::GANHOU(Jogada::PEDRA));
    assert_eq!(jokenpo(Jogada::TESOURA,Jogada::PAPEL),Resultado::GANHOU(Jogada::TESOURA));

    assert_eq!(jokenpo(Jogada::PAPEL,Jogada::PEDRA),Resultado::GANHOU(Jogada::PAPEL));
    assert_eq!(jokenpo(Jogada::TESOURA,Jogada::PEDRA),Resultado::GANHOU(Jogada::PEDRA));
    assert_eq!(jokenpo(Jogada::PAPEL,Jogada::TESOURA),Resultado::GANHOU(Jogada::TESOURA));

    assert_eq!(jokenpo(Jogada::TESOURA,Jogada::TESOURA),Resultado::EMPATE);
    assert_eq!(jokenpo(Jogada::PAPEL,Jogada::PAPEL),Resultado::EMPATE);
    assert_eq!(jokenpo(Jogada::PEDRA,Jogada::PEDRA),Resultado::EMPATE);

}

fn main(){

    println!("{:?}",jokenpo(Jogada::PEDRA,Jogada::PAPEL));    
    println!("{:?}",jokenpo(Jogada::PEDRA,Jogada::TESOURA));    
    println!("{:?}",jokenpo(Jogada::TESOURA,Jogada::PAPEL));
    
    println!();

    println!("{:?}",jokenpo(Jogada::PAPEL,Jogada::PEDRA));    
    println!("{:?}",jokenpo(Jogada::TESOURA,Jogada::PEDRA));    
    println!("{:?}",jokenpo(Jogada::PAPEL,Jogada::TESOURA));
    
    println!();

    println!("{:?}",jokenpo(Jogada::PEDRA,Jogada::PEDRA));    
    println!("{:?}",jokenpo(Jogada::PAPEL,Jogada::PAPEL));    
    println!("{:?}",jokenpo(Jogada::TESOURA,Jogada::TESOURA));    

}
