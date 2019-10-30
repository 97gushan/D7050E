fn foo() -> i32{
    let a: i32 = 1;

    let b: i32 = 1;

    while(b <= 10){
        b = b + 1;
    }
    return b;

}

fn main() -> i32 {
    let b: i32 = foo();

    return b;
}