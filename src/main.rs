use floating_point_conversor::operations::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 3 {
        let string: String = args[1].to_string();
        if string == "integer_to_binary" {
            let integer: i64 = args[2].parse().unwrap();
            if integer < -127 || integer > 127 {
                println!("{}", "Número não pode ser representado.");
            } else {
                println!("{}", integer_to_binary(integer));
            }
        }
        if string == "binary_to_integer" {
            let binary: String = args[2].to_string();
            if binary.chars().count() != 8 {
                println!("{}", "Número não pode ser representado.");
            } else {
                println!("{}", binary_to_integer(binary.as_str()));
            }
        }
        if string == "float_to_binary" {
            let float: f32 = args[2].parse().unwrap();
            if (float.trunc() as i32) <= -32 || (float.trunc() as i32) >= 32 {
                println!("{}", "Número não pode ser representado.");
            } else {
                println!("{}", float_to_binary(float));
            }
        }
        if string == "binary_to_float" {
            let binary: String = args[2].to_string();
            if binary.chars().count() != 8 {
                println!("{}", "Número não pode ser representado.");
            } else {
                println!("{}", binary_to_float(binary.as_str()));
            }
        }
    } else if args.len() == 1 {
        let mut operation: String = "".to_string();
        println!("Qual o tipo de conversão deseja fazer?\n1 - Integer para Binary\n2 - Binary para Integer\n3 - Float para Binary\n4 - Binary para Float\n");
        std::io::stdin()
            .read_line(&mut operation)
            .expect("Failed to read line");

        let operation: u8 = match operation.trim().parse() {
            Ok(num) => num,
            Err(_) => 0,
        };

        if operation == 1 {
            let mut integer: String = "".to_string();

            println!("Digite o número inteiro: ");

            std::io::stdin()
                .read_line(&mut integer)
                .expect("Failed to read line");
            let integer: i64 = match integer.trim().parse() {
                Ok(num) => num,
                Err(_) => 0,
            };
            if integer < -127 || integer > 127 {
                println!("{}", "Número não pode ser representado.");
            } else {
                println!("{}", integer_to_binary(integer));
            }
        } else if operation == 2 {
            println!("Digite o número binário: ");
            let mut binary: String = "".to_string();
            std::io::stdin()
                .read_line(&mut binary)
                .expect("Failed to read line");

            let binary: String = match binary.trim().parse() {
                Ok(num) => num,
                Err(_) => "".to_string(),
            };

            if binary.chars().count() != 8 {
                println!("{}", "Número não pode ser representado.");
            } else {
                println!("{}", binary_to_integer(binary.as_str()));
            }
        } else if operation == 3 {
            println!("Digite o número float: ");
            let mut float: String = "".to_string();
            std::io::stdin()
                .read_line(&mut float)
                .expect("Failed to read line");
            let float: f32 = match float.trim().parse() {
                Ok(num) => num,
                Err(_) => 0.0,
            };
            if (float.trunc() as i32) <= -32 || (float.trunc() as i32) >= 32 {
                println!("{}", "Número não pode ser representado.");
            } else {
                println!("{}", float_to_binary(float));
            }
        } else if operation == 4 {
            println!("Digite o número binário: ");
            let mut binary: String = "".to_string();
            std::io::stdin()
                .read_line(&mut binary)
                .expect("Failed to read line");
            let binary: String = match binary.trim().parse() {
                Ok(num) => num,
                Err(_) => "".to_string(),
            };
            if binary.chars().count() != 8 {
                println!("{}", "Número não pode ser representado.");
            } else {
                println!("{}", binary_to_float(binary.as_str()));
            }
        } else {
            println!("{}", "Opção inválida.");
        }
    }
}
