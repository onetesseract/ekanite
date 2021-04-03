# Ekanite  
no i don't know why this name  
  
This is a notoriously bad programming language currently in development, am writing an interpreter because i can't be asked to write a compiler and want a proof of concept. Eventually the `eval` function will be repurposed and it's tree-walking capacities will either hook into LLVM or output GNU-AS syntax (hard, but nice).  

Todo: 
- add path parsing (namespaces and ::)  
- optimise for tailrec  

## Running
put yer code in file.ek and `cargo run` (needs rust and cargo installed, any recent toolchain).

## Examples

see file.ek for what i am testing it with, but here are some examples that should maybe work.

##### Basic hello world
```
print("Hello, world!");
```  
  
##### If statements
```
if 1 == 2 {
    print("One equals two");
} else {
    print("One does not equal two");
};
```  
else if also works  
  
##### Functions and returning
```
some_fn(arg1: f64, arg2: str): bool {
    print(str);
    if arg1 == 3 {
        return true;
    } else {
        return false;
    };
};
```  
  
### I am considering
- removing semicolons
- not implementing loops and using instead tail recursion



### Random gibberish from previous commits
pp-pupu branch: master  
todo: use name snek  
note to self: recompiling after changing nothing will _not_ scare away the error