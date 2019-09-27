

fn main() -> i32 {

    let a: i32 = 1;


    print(foo(10));
}

fn foo(d: i32) -> i32{
    
    if(d <= 1){
        return d;
    }else{
        
        return  foo(d - 1) + foo(d - 2);
    }
}

