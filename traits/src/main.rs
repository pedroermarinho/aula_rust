trait User {
    // construtor onde vamos receber um nome de usuário (login)
    fn new(username:&'static str) -> Self;

    //retorna o login definido em new
    fn username(&self) -> &'static str;

    // logar-se no sistema 
    fn login(&self) -> &'static str;

    // deslogar-se do sistema
    fn logout(&self) -> &'static str;

    fn is_logged_in(&self) -> bool{
        false
    }
}

#[derive(Debug)]
struct Admin{username:&'static str}
struct Operador{username:&'static str}
struct BascUser{username:&'static str}

impl User for Admin{
    
    fn new(username:&'static str)-> Admin{
        Admin{ username:username}
    } 

    fn username(&self) -> &'static str{
        self.username
    }

    fn login(&self) -> &'static str{
        "Usuário do tipo ADMIN entrou no sistema"
    }

    fn logout(&self) -> &'static str{
        "Usuário do tipo ADMIN saiu do sistema"
    }
}

impl User for Operador{
    
    fn new(username:&'static str)-> Operador{
        Operador{ username:username}
    } 

    fn username(&self) -> &'static str{
        self.username
    }

    fn login(&self) -> &'static str{
        "Usuário do tipo OPERADOR entrou no sistema"
    }

    fn logout(&self) -> &'static str{
        "Usuário do tipo OPERADOR saiu do sistema"
    }
}

impl User for BascUser{
    
    fn new(username:&'static str)-> BascUser{
        BascUser{ username:username}
    } 

    fn username(&self) -> &'static str{
        self.username
    }

    fn login(&self) -> &'static str{
        "Usuário do tipo BASCUSER entrou no sistema"
    }

    fn logout(&self) -> &'static str{
        "Usuário do tipo BASCUSER saiu do sistema"
    }
}

#[derive(PartialEq)]
struct MyStruct{
    number:i32
}

fn main() {
    let admin:Admin = User::new("Pedro Marinho");
    
    println!("\n");
    println!("Bem-vindo usuário {}",admin.username());
    println!("{}",admin.login());
    println!("{}",admin.logout());

    let operador:Operador = User::new("Pedro Eduardo");

    println!("\n");
    println!("Bem-vindo usuário {}",operador.username());
    println!("{}",operador.login());
    println!("{}",operador.logout());

    let basc_user:BascUser = User::new("Pedro Rodrigues");

    println!("\n");
    println!("Bem-vindo usuário {}",basc_user.username());
    println!("{}",basc_user.login());
    println!("{}",basc_user.logout());

    println!("\n");
    print!("{:?}",admin);

    println!("\n");
    let bol1:bool=true;
    let bol2:bool=false;
    
    println!("bol1 == bol2 -> {}", bol1 == bol2);
    println!("bol1.eq(&bol2) -> {}", bol1.eq(&bol2));
    println!("bol1 != bol2 -> {}", bol1 != bol2);
    println!("bol1.ne(&bol2) -> {}", bol1.ne(&bol2));

    println!("\n");
    let char1:char='a';
    let char2:char='b';
    
    println!("char1 == char2 -> {}", char1 == char2);
    println!("char1.eq(&char2) -> {}", char1.eq(&char2));
    println!("char1 != char2 -> {}", char1 != char2);
    println!("char1.ne(&char2) -> {}", char1.ne(&char2));

    println!("\n");
    let int1:i32=1;
    let int2:i32=2;
    
    println!("int1 == int2 -> {}", int1 == int2);
    println!("int1.eq(&int2) -> {}", int1.eq(&int2));
    println!("int1 != int2 -> {}", int1 != int2);
    println!("int1.ne(&int2) -> {}", int1.ne(&int2));

    println!("\n");
    let float1:f32=1.3;
    let float2:f32=1.2;
    
    println!("float1 == float2 -> {}", float1 == float2);
    println!("float1.eq(&float2) -> {}", float1.eq(&float2));
    println!("float1 != float2 -> {}", float1 != float2);
    println!("float1.ne(&float2) -> {}", float1.ne(&float2));

    println!("\n");
    let str1:&'static str="Pedro";
    let str2:&'static str="Marinho";
    
    println!("str1 == str2 -> {}", str1 == str2);
    println!("str1.eq(&str2) -> {}", str1.eq(str2));
    println!("str1 != str2 -> {}", str1 != str2);
    println!("str1.ne(&str2) -> {}", str1.ne(str2));

    println!("\n");

    let a = MyStruct{number:1};
    let b = MyStruct{number:1};

    print!("{}",a == b);


    println!("\n");

    let x1 = 0.0f64/0.0f64;
    let x2 = 0.0f64/0.0f64;
   
    let z1 = 0.0f32/0.0f32;
    let z2 = 0.0f32/0.0f32;

    println!("x1==x2 -> {}",x1==x2);
    println!("z1==z2 -> {}",z1==z2);

    println!("x1 -> {}",x1);
    println!("x2 -> {}",x2);
   
    println!("z1 -> {}",z1);
    println!("z2 -> {}",z2);
   

}
