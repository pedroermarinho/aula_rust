
#[derive(Debug)]
struct Contact{
    name:&'static str,
    phone_number: &'static str
}


fn main() {
    {
        let vector = vec![1,2,3];

        println!("{:?}",vector);
    }
    println!("\n");
    {
        let vector:Vec<i32> = (1..4).collect();

        println!("{:?}",vector)
    }
    println!("\n");
    {
        let contact_1 =Contact{
            name:"teste1",
            phone_number:"+55 (97) 91234234"
        };
        let contact_2 =Contact{
            name:"teste2",
            phone_number:"+55 (97) 9146756785"
        };

        let mut agenda = vec![contact_1,contact_2];

        println!("{:?}",agenda);

        let contact_3 =Contact{
            name:"teste3",
            phone_number:"+55 (97) 91467567885"
        };

        agenda.push(contact_3);
        println!("{:?}",agenda);

        println!("\n");

        let _ = agenda.pop();
        println!("{:?}",agenda);

        println!("\n");

        println!("Contact 1 ->{:?}",agenda[0]);
        println!("Contact 2 ->{:?}",agenda[1]);
        
        println!("\n");
        
        for contact in &agenda{
            println!("{:?}",contact);
        }

        println!("\n");

        let mut agenda_iter=agenda.iter();
        println!("{:?}",agenda_iter.next().unwrap());
        println!("{:?}",agenda_iter.next().unwrap());

    }
    println!("\n");
}