
use std::io;


fn main() {
    loop {
        println!(" -----------------------------------");
        println!("|                                   |");
        println!("|     Калькулятор discriminant      |");
        println!("|                                   |");
        println!("|                                   |");
        println!("| B.Y. @sweert_emperor              |");
        println!("|      ^^^^^^^^^^^^^^^              |");
        println!(" -----------------------------------");

        
        let mut a_str = String::new();
        
        
        let mut b_str = String::new();

        
        let mut c_str = String::new();

        println!("Введите А: ");
        match io::stdin().read_line(&mut a_str) {
            Ok(_) => {},
            Err(e) => println!("Ошибка ввода {e:?}")
        }
        
        println!("Введите B: ");
        match io::stdin().read_line(&mut b_str) {
            Ok(_) => {},
            Err(e) => println!("Ошибка ввода {e:?}")
        }
        
        println!("Введите C: ");
        match io::stdin().read_line(&mut c_str) {
            Ok(_) => {},
            Err(e) => println!("Ошибка ввода {e:?}")
        }

        let a:f64 = a_str.trim().parse().unwrap();
        let b:f64 = b_str.trim().parse().unwrap();
        let c:f64 = c_str.trim().parse().unwrap();

        let d:f64 = (b*b) - 4.0 * (a * c);

        if d > 0.0 {
            let x1 = ((-b) + d.sqrt()) / (2.0 * a);
            let x2 = ((-b) - d.sqrt()) / (2.0 * a);

            println!("Ответ!!! - 2 корня\nD = {d},\tx1 = {x1},\tx2 = {x2}");
            
        }
        if d == 0.0 {
            let x = (-b) / (2.0 * a);
            println!("Ответ - 1 корень!!!\nD = {d},\tx = {x}");
            
        }
        if d < 0.0 {
            println!("Ответ - РЕШЕНИЙ НЕТ!!!\nD = {d}\t");
            
        }
    }
    


}



