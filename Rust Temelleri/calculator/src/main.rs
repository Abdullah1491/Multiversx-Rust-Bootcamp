use std::io;

enum Operation {            // bir enum yapısı oluşturduk varyantlarını f64 olarak tanımladık.
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

fn calculate(operation: Operation)->f64 {     // calculate diye bir fonksiyon oluşturduk. Argümanlarını Operation enum yapısından aldık.
    match operation {                         // match metodu ile bu enum yapısının içindeki farklı durumları kontrol ediyoruz inceliyoruz.
        Operation::Add(a,b)=> a+b,
        Operation::Subtract(a,b)=> a-b,
        Operation::Multiply(a,b)=> a*b,
        Operation::Divide(a,b)=>  {
            if b ==0.0 {
                println!("Warning: Zero cannot divide by zero.");
                return f64::NAN;
            }else{
                a/b
            }
        }
    }
}


fn main() {

    println!("What is the first number?: ");
    let mut input1 = String::new(); 
    io::stdin().read_line(&mut input1).expect("Failure to read number."); 
    let num1: f64 = input1.trim().parse().expect("The number is unvalid. ");

    println!("Which operation would you like to use? (Add,Subtract,Multiply,Divide)");

    let mut operation = String::new(); 
    io::stdin().read_line(&mut operation).expect("Failure to read."); 
    let operation = operation.trim().to_lowercase();

    println!("What is the second number?: ");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failure to read number.");
    let num2: f64 = input2.trim().parse().expect("The number is unvalid. ");



    // Operation enum oluşturuyoruz.
    let operation_enum = match operation.as_str() { 
        "add" => Operation::Add(num1, num2), 
        "subtract" => Operation::Subtract(num1, num2), 
        "multiply" => Operation::Multiply(num1, num2), 
        "divide" => Operation::Divide(num1, num2), _ => { 
            println!("Invalid operation"); 
            return; 
        }
    };
    
    let result =calculate(operation_enum);    //calculate fonksiyonunu çağırıyoruz.
    println!("Result is {result}");




   
}
