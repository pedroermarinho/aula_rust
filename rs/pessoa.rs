
#[derive(Debug)]
enum Sexo{
    Homem,
    Mulher
}

struct Pessoa{
    nome: &'static str,
    sexo: Sexo
}

fn main() {

    let pessoa1=Pessoa{
        nome:"fulana",
        sexo:Sexo::Mulher
    };

    let pessoa2 = Pessoa{
        nome:"Ciclano",
        sexo:Sexo::Homem
    };

    println!("nome: {}, sexo: {:?}",pessoa1.nome,pessoa1.sexo);
    println!("nome: {}, sexo: {:?}",pessoa2.nome,pessoa2.sexo);

}