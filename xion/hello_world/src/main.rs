fn main() {
    println!("Hello, world!");
    let mut a = 16;
    println!("Number is : {}",a);
    a = 50;
    println!("Number is : {}",a);

    let char = 'G';
    println!("Char : {}",char);
    
    let float_num = 3.14;
    println!("Num : {}",float_num);

    const CONST_VALUE : i128 = 12;
    println!("Constant value : {}",CONST_VALUE);

    let mut my_str: String = String::from("Created By Ganesh");
    println!("String : {}",my_str);

    let mut my_str2: String = "Hello World".to_string();
    println!("String : {}",my_str2);

    let mut my_vector: Vec<u64> = Vec::new();
    my_vector.push(1);
    my_vector.push(12);
    println!("Vector : {:?} & Length of vector : {}",my_vector,my_vector.len());

    struct Student {
        rollno: u128,
        marks: u128,
    };

    let my_student = Student{
        rollno: 101,
        marks: 89,
    };

    println!("Student Info\n Roll No : {} \n Marks : {}",my_student.rollno,my_student.marks);


    let mut c: u128 = sum(10,20);
    println!("Sum : {}",c);
    
    c = div(20,10);
    println!("Div : {}",c);

    c = mul(20,10);
    println!("Mul : {}",c);

    c = division(20,10);
    println!("Div : {}",c);

    c = modulus(30,11);
    println!("Mod : {}",c);

    let value = 2;
    if value == 2 {
        println!("True");
    }
    else {
        println!("False");
    }

}


fn sum(a: u128, b: u128) -> u128 {
    let c = a + b;
    return c;
}

fn div(a: u128, b: u128) -> u128 {
    a - b
}

fn mul(a: u128,b: u128) -> u128 {
    a * b
}

fn division(a: u128,b: u128) -> u128 {
    a / b
}

fn modulus(a: u128,b: u128) -> u128 {
    a % b
}