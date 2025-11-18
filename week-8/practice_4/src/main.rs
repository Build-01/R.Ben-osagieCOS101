fn main() {
  // Name vector 
  let name = vec!["Ruth", "Sam", "Chizaram", "Chiemelie", "Mosope", "Kelita", "Mark", "Igie", "Sharon", "Olaitan"];

  // Age vector
  let age = vec![16,17,18,19,20,22,23,14,32,44];

  print!("\nAge allocation:\n");
  for i in 0..age.len()
  {
    //itenating through i on the vector
    print!("{} is {} years old\n",name[i], age[i]);
  }
}
