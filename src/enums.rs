#[derive(Debug)]
enum GenderCategory {
   Male,
   Female
}

fn main() {
   let male = GenderCategory::Male;
   let female = GenderCategory::Female;

   println!("{:?}", male);
   println!("{:?}", female);
}