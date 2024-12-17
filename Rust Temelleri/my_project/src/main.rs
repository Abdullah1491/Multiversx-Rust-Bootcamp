fn main() {
    let collection = vec![3, 5, 7, 10];  
    let filter = FilterCondition { value: 5 }; 
    let filtered_collection = custom_filter(&collection, &filter);

    println!("Filtered items: {:?}", filtered_collection);
}

struct FilterCondition {
    value: i32,
}

impl FilterCondition {
    fn is_match(&self, item: &i32) -> bool {
        *item == self.value           
    }
}

fn custom_filter(collection: &[i32], condition: &FilterCondition) -> Vec<i32> {
    collection.iter()
        .filter(|&item| condition.is_match(&item)) 
        .copied() 
        .collect() 
}        
