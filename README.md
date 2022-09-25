- ## What  
- A python-like scripting language with a more strict syntax (braces, semicolons) and a focus on dynamicism while retaining syntactic elements of compiled languages of the C family  
- Functions as first class entities  
- Anonymous functions  
- Interfaces, structs, extensions to built in types  
- Code Example:  
```
def maybe_divide(num: int, denom: int) -> int? { // a question mark signifies an Option type, which is similar to Rust's. It's just shorthand for Option<int> in this case.
  if denom == 0 {
      none;
  } else {
      num / denom;
  }
}

def square(x) -> int {
  return x * x;
}

def say_hello(name: str) -> none {
  print "Hello, {name}!";
}

def main() {
  print "hello" // print is a primitive statement, standard // to denote comment
  
  // Variables
  name: str = "jack"; // optional type annotations
  print name; // this should work
  num = 42; // will default to int if possible
  print num; // this also works
  print "I like the number {num}"; // interpolation
 
  // Arithmetic
  result = num + 10; // 14
 
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
