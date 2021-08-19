fn stack_only(b: i32) -> i32{
    let c = 3;
    return b + c + stack_and_heap();
}

fn stack_and_heap()->i32{
    let d = 5;
    let e = Box::new(7); // Box - smart pointer in rust.smart pointer allocate memory on stack and de-allocate/free memory when exit.
                        // let e = stores the addeess of the value "7" in stack and store value in heap with addess.
    return d + *e; // and "*e" will de-reference the value 
}   

pub fn run(){
    let a = 2;
    let result = stack_only(a);
    dbg!(result);
}