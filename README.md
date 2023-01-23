# TODO

- [ ] Implement a lexer that can take input and produce a list of tokens

# What  

- A python-like scripting language with a more strict syntax (braces, semicolons) and a focus on dynamicism while retaining syntactic elements of compiled languages of the C family  
- Functions as first class entities  
- Anonymous functions  
- Interfaces, structs, extensions to built in types  
- Code Example:  

```
let maybe_divide(num: int, denom: int) -> int? = { // a question mark signifies an Option type, which is similar to Rust's. It's just shorthand for Option<int> in this case.
  if denom == 0 {
      none;
  } else {
      num / denom;
  }
}

let square(x) -> int = {
  return x * x;
}

let say_hello(name: str) -> none = {
  print "Hello, {name}!";
}

let main() = {
  print "hello" // print is a primitive statement, standard // to denote comment
  
  // Variables
  name: str = "jack"; // optional type annotations
  print name; // this should work
  num = 42; // will default to int if possible
  print num; // this also works
  print "I like the number {num}"; // interpolation
 
  // Arithmetic
  var result = num + 10; // 14
 
  // Loops
  for i in 1..10 {
      print i;
  }
  for {
      print "This is an infinite loop";
  }
  for {
      print "This will execute once.";
      break;
  }
}
```
