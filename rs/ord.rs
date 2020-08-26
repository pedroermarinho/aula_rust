use std::cmp::Ordering;

#[derive(PartialOrd,PartialEq,Eq,Clone, Copy)]
struct Pessoa{
    anos:i32,
    nome:&'static str,
}

impl Ord for Pessoa{
    
    fn cmp(&self, other: &Pessoa) -> Ordering {
        (self.anos).cmp(&(other.anos))
    }
}

fn older(p1:Pessoa,p2:Pessoa){
    if p1 > p2{
        println!("{} tem mais anos de vida do que {}",p1.nome,p2.nome);
    }else {
        println!("{} tem mais anos de vida do que {}",p2.nome,p1.nome);
    }
}


fn main() {
    let p1=Pessoa{
        nome:"Pedro",
        anos:20
    };
    let p2=Pessoa{
        nome:"Eduardo",
        anos:18
    };
    let p3=Pessoa{
        nome:"Rodrigues",
        anos:19
    };

    older(p1, p2);
    older(p2, p3);
    older(p3, p1);


}
