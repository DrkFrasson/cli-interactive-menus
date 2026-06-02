use std::time;
use std::thread;
use std::{io, io::Write, env};

struct Arguments{
    style: String,
    color: String,
    alignment: String,
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
    let mut alignment: String = Default::default();

    match args.len() {
        0 => panic!(), // Should be unreachable;
        1 => { // program\n
            println!("You missed some arguments:");

            print!("Style > ");

            let _ = io::stdout().flush();
            io::stdin().read_line(&mut style).expect("Failed to read style!");

            while style == "\n" {
                println!("Not a valid answer, try again!");
                io::stdin().read_line(&mut style).expect("Failed to read style!");
            }

            print!("Color > ");

            let _ = io::stdout().flush();
            io::stdin().read_line(&mut color).expect("Failed to read color!");

            while style == "\n" {
                println!("Not a valid answer, try again!");
                io::stdin().read_line(&mut color).expect("Failed to read style!");
            }

            print!("Alignment > ");

            let _ = io::stdout().flush();
            io::stdin().read_line(&mut alignment).expect("Failed to read color!");
            if alignment == "\n" {alignment = "n".to_string()} else {};

            style = style.trim().parse().expect("Failed to clean input!");
            color = color.trim().parse().expect("Failed to clean input!");
            alignment = alignment.trim().parse().expect("Failed to clean input!");
        },
        2 => { // program [style]\n
            println!("You didn't defined the color!");

            print!("Color > ");

            let _ = io::stdout().flush();
            io::stdin().read_line(&mut color).expect("Failed to read color!");

            while style == "\n" {
                println!("Not a valid answer, try again!");
                io::stdin().read_line(&mut color).expect("Failed to read style!");
            }

            style = args[1 as usize].trim().parse().expect("Failed to clean first argument!");
            color = color.trim().parse().expect("Failed to clean input!");
            alignment = "n".to_string();
        },
        3 => { // program [style] [color]\n
            style = args[1 as usize].trim().parse().expect("Failed to clean first argument!");
            color = args[2 as usize].trim().parse().expect("Failed to clean second argument!");
            alignment = "n".to_string();
        },
        4 => {
            style = args[1 as usize].trim().parse().expect("Failed to clean first argument!");
            color = args[2 as usize].trim().parse().expect("Failed to clean second argument!");
            alignment = args[3 as usize].trim().parse().expect("Failed to clean second argument!");
        }
        5.. => println!("Too much arguments!"),
    }

    let mut color_n: u8 =
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

    let mut color: String = Default::default();

    match &style as &str {
        "background" => {
            style = "  ".to_string();
            if color_n == 31
            || color_n == 32
            || color_n == 33    // --> Bright colors.
            || color_n == 37
            || color_n == 91
            || color_n == 92
            || color_n == 93
            || color_n == 97
            || color_n == 39 {style += "\x1b[30m"} else {}
            if color_n == 39 {color_n = 47} else {color_n += 10;}
            color = color_n.to_string();
        },
        "foreground" => style = "  ".to_string(),
        "item"       => style = " *\x1b[22m\x1b[39m".to_string(),
        "bullet"     => style = "=>\x1b[22m\x1b[39m".to_string(),
        "simple"     => style = " >\x1b[22m\x1b[39m".to_string(),
        _other       => panic!(),
    };

    if style != "background" {color = color_n.to_string();} else {}
    return Arguments{style, color, alignment};
}

fn compose_menu(options: [&str; 10], arguments: Arguments) -> String
{
    let mut line: u8 = 0;

    let spacing: String = if arguments.alignment == "l" {"".to_string()} else if arguments.alignment == "r" {"  ".to_string()} else {" ".to_string()};
loop{
        for i in 0..10 {
            if i == line {
                println!("\x1b[{1}m{2}{3}{}\x1b[0m", options[i as usize], arguments.color, arguments.style, spacing);
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
