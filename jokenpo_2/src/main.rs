#[derive(Debug, PartialEq)]
enum Jogada{
    PEDRA,
    PAPEL,
    TESOURA
}

impl Jogada{
    fn ganha_de(&self)->Jogada{
        match self{
            Jogada::PEDRA => Jogada::TESOURA,
            Jogada::PAPEL => Jogada::PEDRA,
            Jogada::TESOURA => Jogada::PAPEL
        }
    }
}

fn jokenpo(a:Jogada, b:Jogada) -> Option<Jogada>{
    if a == b {
        None
    }else if a.ganha_de() == b{
        Some(a)
    }else{
        Some(b)
    }    
}

#[test]
fn teste_jokenpo(){

    assert_eq!(jokenpo(Jogada::PEDRA,Jogada::PAPEL),Some(Jogada::PAPEL));
    assert_eq!(jokenpo(Jogada::PEDRA,Jogada::TESOURA),Some(Jogada::PEDRA));
    assert_eq!(jokenpo(Jogada::TESOURA,Jogada::PAPEL),Some(Jogada::TESOURA));

    assert_eq!(jokenpo(Jogada::PAPEL,Jogada::PEDRA),Some(Jogada::PAPEL));
    assert_eq!(jokenpo(Jogada::TESOURA,Jogada::PEDRA),Some(Jogada::PEDRA));
    assert_eq!(jokenpo(Jogada::PAPEL,Jogada::TESOURA),Some(Jogada::TESOURA));

    assert_eq!(jokenpo(Jogada::TESOURA,Jogada::TESOURA),None);
    assert_eq!(jokenpo(Jogada::PAPEL,Jogada::PAPEL),None);
    assert_eq!(jokenpo(Jogada::PEDRA,Jogada::PEDRA),None);

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
