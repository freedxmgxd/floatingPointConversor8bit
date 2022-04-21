fn integer_to_binary(integer: i64) -> String {
    let mut binary: String = "".to_string();
    let mut remainder: i64 = integer;
    if remainder < 0 {
        remainder = 255 + remainder;
    }
    for _i in 1..8 {
        let digit: i64 = remainder % 2;
        binary = digit.to_string() + &binary;
        remainder = remainder / 2;
    }
    if integer < 0 {
        binary = "1".to_string() + &binary.to_string();
    }
    return binary;
}

fn binary_to_integer(binary: String) -> i64 {
    let mut integer: i64 = 0;
    let mut aux: i64 = 0;
    for n in binary.chars() {
        let digit: i64 = n.to_digit(10).unwrap() as i64;
        let pot: i64 = (2 as i64).pow(aux as u32);
        integer = integer + digit * pot;
        aux = aux + 1;
    }
    if binary.chars().nth(0).unwrap() == '1' {
        integer = integer - 255;
    }

    return integer;
}

fn float_to_binary(float: f32) -> String {
    let mut binary: String = "".to_string();
    let mut aux: f32 = float;

    if float < 0.0 {
        binary = "1".to_string();
        aux = aux * -1.0;
    } else {
        binary = "0".to_string();
    }

    let mut int: i8 = aux.trunc() as i8;
    let mut mantissa: String = "".to_string();
    let mut i: i8 = 0;

    while int != 0 {
        let digit: i8 = int % 2;
        i = i + 1;
        int = int / 2;
        if int == 0 {
            break;
        };
        mantissa = digit.to_string() + &mantissa;
    }

    let mut j: i8 = i;
    let mut k: bool = false;
    let mut z: i8 = 0;

    while j < 5 {
        aux = aux.fract() * 2.0;
        let digit: i8 = aux.trunc() as i8;
        if (digit == 1 || j != 0) && k == false {
            k = true;
        } else if k == false {
            z = z + 1;
        }
        if k == true {
            if j != 0 {
                mantissa = mantissa + &digit.to_string();
            }
            j = j + 1;
        }
    }

    let mut exponent: String = "".to_string();

    if i > 0 {
        i = i + 2; // (i - 1) <- o expoente + 3 <- ajuste para equivalente em excesso de 4.
        for _j in 0..3 {
            let digit: i8 = i % 2;
            exponent = digit.to_string() + &exponent;
            i = i / 2;
        }
        binary = binary + &exponent + &mantissa;
    } else {
        z = -z + 2;
        for _j in 0..3 {
            let digit: i8 = z % 2;
            exponent = digit.to_string() + &exponent;
            z = z / 2;
        }
        binary = binary + &exponent + &mantissa;
    }

    return binary;
}

fn binary_to_float(binary: String) -> f32 {
    let mut float: f32 = 0.0;
    let mut signal: i8 = 1;
    let mut exponent: i8 = 0;

    let mut aux = binary.chars();
    let mut bit = aux.next().unwrap();

    if bit == '1' {
        signal = -1;
    }
    
    for j in 1..4 {
        bit = aux.next().unwrap();
        let digit: i8 = bit.to_digit(10).unwrap() as i8;
        let pot: i8 = (2 as i8).pow((3 - j) as u32);
        exponent = exponent + digit * pot;
    }
    exponent = exponent - 3;

    for j in 4..9 {
        if j == 4 {
            let digit: i8 = 1;
            let pot: f32 = (2 as f32).powi(exponent as i32);
            float = float + digit as f32 * pot;
        } else {
            bit = aux.next().unwrap();
            let digit: i8 = bit.to_digit(10).unwrap() as i8;
            let pot: f32 = (2 as f32).powi((exponent - (j - 4) as i8) as i32);
            float = float + digit as f32 * pot;
        }
    }

    float = signal as f32 * float;
    return float;
}

fn main() {
    // TODO: Caso o número não possa ser representado, o usuário deve ser informado.
    let binary: String = integer_to_binary(-126);
    let integer: i64 = binary_to_integer("10000001".to_string());
    println!("Binary: {} \nInteger: {}\n", binary, integer);

    let binary_f: String = float_to_binary(0.40625 as f32);
    let float: f32 = binary_to_float("11100111".to_string());
    println!("Float: {}\nDecimal: {}", binary_f, float);
}
