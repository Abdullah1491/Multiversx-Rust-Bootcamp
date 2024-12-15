// Concanetare adında bir fonksiyon oluşturuyorum. İçinde 2 string parametre ifadesi tanımladım.
fn concatenate_strings(string1: &str, string2: &str) -> String {
    // result adında yeni bir string değişkeni tanımlıyorum.
    let mut result=String::new();

    result.push_str(string1);     // her iki stringi resulta ekliyoruz.
    result.push_str(string2);
    // Birleştirilmiş halini geri döndürüyoruz.
    return result;                 
}

fn main(){
    // şimdi iki string değişkenini tanımlıyoruz. Onlara değer atamış olduk.
    let string1=String::from("Hello, ");
    let string2=String::from("Rust!");
    
    // concatenated_string adında bir değişken oluşturuyoruz ve concatenate_strings fonksiyonunun sonucunu bu değişkene atıyoruz.
    let concatenated_string=concatenate_strings(&string1,&string2);

    println!("{}",concatenated_string);// Çıktımızı concatenated olarak çağırıyoruz ve yazdırıyoruz.

    // çıktı: "Hello, Rust!""
}