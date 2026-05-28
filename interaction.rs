use std::{io, io::Write/*, env*/};

fn main()
{
/*    let args: Vec<String> = env::args().collect();

    let mut argument: String = Default::default();

    match args.len()
    {
        0 => panic!("Actually impossible!"),
        1 => {
            print!("Especify the type of menu you want to try > ");
            let _ = io::stdout().flush();
            io::stdin()
                .read_line(&mut argument)
                .expect("Failed to read line!");
            while argument == Default::default()
            {
                print!("You should especify type of menu you want to try > ");
                io::stdin()
                    .read_line(&mut argument)
                    .expect("Failed to read line!");
            }
        },
        2 => argument = args[1];,// help {name}
        3.. => println!("Too many arguments, I only take one!"),
        other => panic!("Ink what it could be!"),
    }*/

    let options: [&str; 10] = [
    "Arch",
    "Debian",
    "Fedora",
    "Gentoo",
    "openSUSE",
    "Ubuntu",
    "freeBSD",
    "NixOS",
    "Kali",
    "Mint",
    ];

//    print!("\x1B[2J");

    let _ = io::stdout().flush();

    println!("There you could select multiple options:\n");


    let mut line: u8 = 0;

    let mut i: u8 = 0;
    while i < 10
    {
        if i == line {
            println!("\x1b[1;31m\x1b[33m=>\x1b[22m\x1b[39m {}", options[i as usize]);
        }else{
            println!("   {}", options[i as usize]);
        }
        i += 1;
    }

loop{
//        print!("\x1B[2J");

        let _ = io::stdout().flush();

        let mut input: String = Default::default();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input!");

        if input == "\x1B[A\n" {
            if line == 0 {} else {
                print!("\x1B[K\x1B[A\x1B[K");
                print!("\x1B s");
                print!("\x1b[{}A\x1B[2D", {10 - line});
                print!("\x1b[1;31m\x1b[33m=>\x1b[22m\x1b[39m");
                print!("\x1B[2D\x1b[B   ");
                print!("\x1B u");
                line -= 1;
            }
        }else if input == "\x1B[B\n" {
            if line == 9 {} else {
                print!("\x1B[K\x1B[A\x1B[K");
                print!("\x1B[5D");
                print!("\x1B s");
                print!("\x1b[{}A\x1B[2D", {10 - line});
                print!("\x1b[1;31m\x1b[33m=>\x1b[22m\x1b[39m");
                print!("\x1B[2D\x1b[A  ");
                print!("\x1B u");
                line += 1;
            }
        }else if input == "\n"{
            println!("You choosed this one: {}", options[line as usize]);
            break;
        }

        /*
        println!("{}", line);
        println!("{}", input);
        panic!();*/
    }

//    cursorItem(options);
    /*
    if argument == "recomendations" {
    }else if argument == "foreground" {
    }else if argument == "colored" {
    }else if argument == "item" {
    }else if argument == "cursorItem" {
    }else if argument == "simple" {
    }else if argument == "help" {
    }else{
    }*/
}

/*
fn recomendations()
{
    //
}

fn foreground()
{
    //
}

fn colored()
{
    //
}

fn item()
{
    //
}

fn cursorItem(options: String)
{
    print!("\x1B[2J");

    let _ = io::stdout().flush();

    let input: String = Default::default();

    let line: u8 = 0;
loop{
        print!("\x1B[2J");

        println!("There you could select multiple options:");

        let mut i: u8 = 0;
        while i < 10
        {
            if i == line {
                println!("\x1b[1;31m\x1b[33m=>\x1b[22m\x1b[39m {}", options[i]);
            }else{
                println!("   {}", options[i]);
            }
            i += 1;
        }

        let _ = io::stdout().flush();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input!");

        if input == "^[[A" {
            print!("\x1b[A");
            print!("\x1b[1;31m\x1b[33m=>\x1b[22m\x1b[39m");
        }else if input == "^[[B" {
            print!("\x1b[B");
            print!("\x1b[1;31m\x1b[33m=>\x1b[22m\x1b[39m");
        }else{}
    }
}

fn simple()
{
    //
}

fn ()
{
    //
}*/
