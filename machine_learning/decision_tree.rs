use rustrees::{DecisionTree, Dataset, r2};
 
fn main() {
    let dataset = Dataset::read_csv("data/titanic_train.csv", ",");
 
    let dt = DecisionTree::train_reg(
        &dataset, 
        Some(5),        // max_depth
        Some(1),  // min_samples_leaf        
        None,     // max_features (None = all features)
        Some(42), // random_state
    );
    
    let pred = dt.predict(&dataset);
    
    println!("r2 score: {}", r2(&dataset.target_vector, &pred));
}