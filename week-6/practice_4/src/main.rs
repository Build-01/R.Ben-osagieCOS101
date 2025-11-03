fn main() {
   let fullname = "Ben Osagie Ruth";
   let department = "Computer Science";
   let uni = "Pan-Atlantic University";



   let mut school = "School of Science".to_string();

   //Push string
   school.push_str(" and technology");



   println!("My name is: {}", fullname);


   //check lenght


   println!("The lenght of my fullname is : {}", fullname.len());
   println!("I am a student of {} Department", department);
   println!("{}", school);
   println!("{}", uni);
}
