

// account traitini Result<(), String> şeklinde güncelledik.
trait Account {                   
    fn deposit(&mut self, amount: f64) -> Result<(),String>;    
    fn withdraw(&mut self, amount: f64) -> Result<(),String>; 
    fn balance(&self) -> f64; 
}


struct BankAccount {
    account_number: i32,      
    holder_name: String,
    balance: f64,
}


impl Account for BankAccount {
    fn deposit(&mut self, amount: f64) -> Result<(), String> {   // implementasyon kısmında da result string kısmını güncelledik.
        if amount <= 0.0 {
            Err("The deposit amount must be greater than zero.".to_string())   
            // Err("Hata mesajı".to_string()) varyantını ekledik.
        } else {
            self.balance += amount;
            Ok(())                      // Ok(()) varyantını ekledik.
        }
    }

    fn withdraw(&mut self, amount: f64) -> Result<(), String> {    // implementasyon kısmında da result string kısmını güncelledik.
        if amount <= 0.0 {
            Err("Çekilecek miktar sıfırdan büyük olmalıdır.".to_string())  
            // Err("Hata mesajı".to_string()) varyantını ekledik.
        } else if amount > self.balance {
            Err("Insufficient funds.".to_string())
        } else {
            self.balance -= amount;
            Ok(())                     // Ok(()) varyantını ekledik.
        }
    }

    fn balance(&self) -> f64 {
        self.balance                
    }
}

fn main() {
    
    let mut account1 = BankAccount {
        account_number: 111,
        holder_name: "Micheal".to_string(),        
        balance: 1000.0,
    };

    let mut account2 = BankAccount {
        account_number: 222,
        holder_name: String::from("Selena"),       
        balance: 1500.0,
    };
    // match ifadesiyle deposit varyantına handling error döndürülmüştür/eklenmiştir.
    match account1.deposit(900.0) {
        Ok(_) => println!("{}'s deposit successful.", account1.holder_name),
        Err(e) => println!("Error: {}", e),
    }
    

    // match ifadesiyle withdraw varyatona handling error döndürülmüştür/eklenmiştir.
    match account2.withdraw(1250.0) {
        Ok(_) => println!("{}'s withdrawal successful.", account2.holder_name),
        Err(e) => println!("Error: {}", e),
    }    
    
    println!("{}'s balance: {}", account1.holder_name, account1.balance());
    println!("{}'s balance: {}", account2.holder_name, account2.balance());
}
