use std::io;
use std::io::Write;
use std::process;

fn main()
{
    loop
    {
        let mut stdin: std::io::Stdin = io::stdin();
    
        let operator: char = choose_operator(&mut stdin);
    
    
    
        let num1 = choose_num_f(&stdin, "first num: ");
        let num2 = choose_num_f(&stdin, "second num: ");
    
        eval_print(&num1, &num2, operator);

    }
}

fn flush_print(s: &str)
{
    print!("{}", s);
    io::stdout().flush().unwrap();
}


fn choose_operator(stdin: &mut std::io::Stdin) -> char
{
    let mut s_operator = String::new();
    let mut operator: char;
    loop
    {
        loop
        {
            flush_print("q to Quit, or choose operator [+-*/]: ");
            s_operator.clear();


            let line = stdin.read_line(&mut s_operator);
            if(!line.is_err())
            {
                break;
            }
            else
            {
                panic!("failed to read value from stdin");
            }
        }
        operator = s_operator.chars().nth(0).unwrap();
        match operator
        {
            '+' => break,
            '-' => break,
            '*' => break,
            '/' => break,
            'q' => std::process::exit(0),
            _   => println!("incorrect operator"),
        };

    }

    return operator;
}


fn choose_num_f(stdin: &std::io::Stdin, prompt_string: &str) -> f32
{
    let mut s = String::new();
    let num;

    flush_print(prompt_string);
    loop
    {
        match stdin.read_line(&mut s)
        {
            Ok(_) => (),
            Err(_) => panic!("failed to read value from cli"),
        }
        

        if(!s.trim().parse::<f32>().is_err())
        {
            num = s.trim().parse::<f32>().unwrap();
            break;
        }
        else
        {
            println!("failed to parse as float");
        }
    }

    return num;

}

fn eval_print(num1: &f32, num2: &f32, operator: char)
{
    let result: f32;

    match operator
    {
        '+' => result = num1 + num2,
        '-' => result = num1 - num2,
        '*' => result = num1 * num2,
        '/' => result = num1 / num2,
        _   => panic!("failed evalutation of numbers"),

    }

    println!("{} {} {} = {}", num1, operator, num2, result);
}
