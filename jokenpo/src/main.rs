#[derive(Debug)]
enum Jogada{
    PEDRA,
    PAPEL,
    TESSOURA,
    EMPATE
}

fn jokenpo(a:Jogada, b:Jogada) -> Jogada{
    match (a, b){
        (Jogada::PEDRA,Jogada::PAPEL) => {return Jogada::PAPEL},
        (Jogada::PEDRA,Jogada::TESSOURA) => {return Jogada::PEDRA},
        (Jogada::TESSOURA,Jogada::PAPEL) => {return Jogada::TESSOURA},
        (_,_) => { return Jogada::EMPATE;}
    }
}

fn main(){
    println!("{:?}",jokenpo(Jogada::PEDRA,Jogada::PAPEL));    
    println!("{:?}",jokenpo(Jogada::PEDRA,Jogada::TESSOURA));    
    println!("{:?}",jokenpo(Jogada::TESSOURA,Jogada::PAPEL));    
    println!("{:?}",jokenpo(Jogada::PEDRA,Jogada::PEDRA));    
}
