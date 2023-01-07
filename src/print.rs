pub fn run() {
  println!("Hello {}, you are from {}", "Brad", "Lisbon");
  println!("Hello {0}, you are from {1}, right {0}?", "Brad", "Lisbon");
  println!("{name} likes to play {activity}", name="John", activity="Football");
  println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);
  println!("{:?}", (12, true, "hello"));
  println!("10 + 10 = {}", 10+10);
}