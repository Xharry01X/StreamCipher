






fn main() {
   
  let found_toy = Some("Teddy bear");

  match found_toy {
    Some(toy) => println!("Yay, I found a {}!", toy),
    None => println!("oh no, the box is empty"),
  }

}
