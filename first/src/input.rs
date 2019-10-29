fn foo() -> i32{

    let a: i32 = 1;

    while(a < 10){
        a = a + 1;
    }

    if(a >= 10 || 9 == 20 - 5 && false){
        return a;
    }

    return a + 5 * (2 - 5);
}

fn main() -> i32 {
    return foo();
}
