// Account adında bir trait oluşturuyorum.
trait Account {  
    // trait altındaki fonksiyon isimleri küçük harfle başlamalı. Örn:fn Deposit olmamalı.                    
    fn deposit(&mut self, amount: f64); 
    fn withdraw(&mut self, amount: f64); 
    fn balance(&self) -> f64; 
}

// BankAccount struct'ını oluşturuyoruz.
struct BankAccount {
    account_number: i32,      /*Hesap numarası yalnızca rakamlardan oluşuyorsa i32, 
                               eğer harf ve rakam içeriyorsa String olarak tanımlanmalıdır.*/
    holder_name: String,
    balance: f64,
}

// Account trait'ini BankAccount yapısına implement ediyoruz.
impl Account for BankAccount {
    fn deposit(&mut self, amount: f64) {
        self.balance += amount; 
    }

    fn withdraw(&mut self, amount: f64) {
        if amount <= self.balance {
            self.balance -= amount;           // Yeterli bakiye varsa, para çekme işlemi yapılıyor.
        } else {
            println!("insufficient funds."); 
        }
    }

    fn balance(&self) -> f64 {
        self.balance                // Hesabın bakiyesi döndürülüyor
    }
}

fn main() {
    // BankAccount tipinde iki hesap oluşturuluyor.
    let mut account1 = BankAccount {
        account_number: 111,
        holder_name: "Micheal".to_string(),        // String::from("Micheal") olarakta kullanılabilir.
        balance: 1000.0,
    };

    let mut account2 = BankAccount {
        account_number: 222,
        holder_name: String::from("Selena"),       // "Selena".to_string() olarakta kullanılabilir. 
        balance: 1500.0,
    };

    // account1'e deposit (para yatırma) işlemi için call (çağırma) işlemi yapıyoruz.
    account1.deposit(900.0);

    // account2'den withdraw (para çekme) işlemi için call ( çağırma) işlemi yapıyoruz.
    account2.withdraw(1250.0);

    // Her iki hesabın bakiyeleri yazdırılıyor
    println!("{}'s balance: {}", account1.holder_name, account1.balance());
    println!("{}'s balance: {}", account2.holder_name, account2.balance());
}
