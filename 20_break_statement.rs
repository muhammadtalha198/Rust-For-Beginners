Break Statement

fn main() {
  // define a for loop
  for i in 0..10 {
    println!("i:{}", i);
    if i == 5 {
      break;
    }
  }
}

Output
1.17s

i:0
i:1
i:2
i:3
i:4
i:5


Using With a while Loop#


fn main() {
  let mut i = 1;
  let found = false;
  // define a while loop
  while !found {
    println!("i:{}", i);
    if i == 5 {
      break;
    }
    i = i + 1;    
  }
}

Output
7.27s

i:1
i:2
i:3
i:4
i:5
