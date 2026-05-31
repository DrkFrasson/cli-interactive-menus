use std::time;
use std::thread;
use std::{io, io::Write, env};

struct Arguments{
    style: String,
    color: u8,
}

fn main()
{
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

    let args: Vec<String> = env::args().collect();
    let arguments = input_handling(args);

    println!("There you could select multiple options:");
    let choise: String = compose_menu(options, arguments);

    println!("Distro: {}", choise);

    print!("Downloading .iso --> [");
    for _ in 1..106 {print!("·");}
    print!("]1 TB/s\n");

    print!("Do you wanna install and reboot now? (y/n): ");

    let mut reboot: String = Default::default();
    let _ = io::stdout().flush();
    io::stdin()
        .read_line(&mut reboot)
        .expect("Failed to reboot");
    if reboot == "y\n" {
        println!("\nrebooting...");
    }else {
        println!("\nSomething was wrong, rebooting...");
    }

    print!("\x1B[H");

    print!("\x1B[2J");
    thread::sleep(time::Duration::from_secs(7));

    println!("you got scared, right?");
}

fn input_handling(args: Vec<String>) -> Arguments
{
    let mut style: String = Default::default();
    let mut color: String = Default::default();

    match args.len() {
        0 => panic!(), // Should be unreachable;
        1 => { // program\n
            println!("You missed some arguments:");

            print!("Style > ");

            let _ = io::stdout().flush();
            io::stdin().read_line(&mut style).expect("Failed to read style!");

            print!("Color > ");

            let _ = io::stdout().flush();
            io::stdin().read_line(&mut color).expect("Failed to read color!");

            style = style.trim().parse().expect("Failed to clean input!");
            color = color.trim().parse().expect("Failed to clean input!");
        },
        2 => { // program [style]\n
            println!("You didn't defined the color!");

            print!("Color > ");

            let _ = io::stdout().flush();
            io::stdin().read_line(&mut color).expect("Failed to read color!");

            style = args[1 as usize].trim().parse().expect("Failed to clean first argument!");
            color = color.trim().parse().expect("Failed to clean input!");
        },
        3 => { // program [style] [color]\n
            style = args[1 as usize].trim().parse().expect("Failed to clean first argument!");
            color = args[2 as usize].trim().parse().expect("Failed to clean second argument!");
        },
        4..1000 => println!("Too much arguments!"),
        _other => panic!("Incorrect format!"),
    }

/*
    arguments.style =
        match arguments.style as &str {
            "foreground" => "",
            "colored"    => "",
            "item"       => "",
            "cursorItem" => "",
            "simple"     => "",
            other        => "",
        }*/

    let color: u8 =
        match &color as &str {
            "red"      => 31, // Red
            "green"    => 32, // Green
            "yellow"   => 33, // Yellow
            "blue"     => 34, // Blue
            "white"    => 37, // White
            "b-red"    => 91, // Bright Red
            "b-green"  => 92, // Bright Green
            "b-yellow" => 93, // Bright Yellow
            "b-blue"   => 94, // Bright Blue
            "b-white"  => 97, // Bright White
            _other      => 39, // Default
        };
    return Arguments{style, color};
}

fn compose_menu(options: [&str; 10], arguments: Arguments) -> String
{
    let mut line: u8 = 0;

loop{
        for i in 0..10 {
            if i == line {
                println!("\x1b[{1}m=>\x1b[22m\x1b[39m {}", options[i as usize], arguments.color);
            }else{
                println!("   {}", options[i as usize]);
            }
        }

        let _ = io::stdout().flush();

        let mut input: String = Default::default();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input!");

        if input == "\x1B[A\n" {
            if line == 0 {} else {line -= 1;}
        }else if input == "\x1B[B\n" {
            if line == 9 {} else {line += 1};
        }else if input == "\n"{
            println!("You choosed this one: {}", options[line as usize]);
            return options[line as usize].to_string();
        }

        for _i in 1..12 {
            print!("");
            print!("\x1B[2K\x1B[A");
        }

        if line == 67 {println!("{}", arguments.style);} else {}
    }
}













/*
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

//    print!("\x1B[2J");


        for _i in 1..12 {
            print!("");
            print!("\x1B[2K\x1B[A");
        }

    }
    print!("Downloading .iso --> [");
    for _ in 1..106 {print!("·");}
    print!("]1 TB/s\n");
    print!("Do you wanna install and reboot now? (y/n): ");
    let mut reboot: String = Default::default();
    let _ = io::stdout().flush();
    io::stdin()
        .read_line(&mut reboot)
        .expect("Failed to reboot");
    if reboot == "y\n" {
        println!("\nrebooting...");
    }else {
        println!("\nSomething was wrong, rebooting...");
    }

    print!("\x1B[H");

    print!("\x1B[2J");
    thread::sleep(time::Duration::from_secs(7));

    println!("you got scared, right?");
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

fn cursorItem(options: [&str; 10])
{
}
*/
