// age checker with using expression
fn age_classer(age: u32) ->$str {
   
 let res = if age <= 18 {
    "you are not able to passe";
  } else if age >= 60 {
    "you age is exced , you're not able to passe the exam";
  } else {
     "Pass";
  }

  let userAge = 23;

  println!("Hi => {age_classer(userAge)}");

}
