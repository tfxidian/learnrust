
fn main(){
    
    let array  = [1,2,3];
    for i in array.iter(){
        println!("{}", i);
    }

    for i in 0u32..10{
        println!("{}", i);
    }
    println!("");
    
    if  1==1 {
        println!("Maths is working!");
    }else{
        println!("oh no ...");
    }

    while 1 == 1{
        println!("The universe is operating normally.");
        break
    }
    loop{
        println!("hello!");
        break
    }


    // memory safety
    //// Owned pointer – only one thing can ‘own’ this pointer at a time
    // This means that when the `Box` leaves its scope, it can be automatically deallocated safely.
    let mut mine: Box<i32> = Box::new(3);
    *mine = 5;
    println!("{}", mine);
    let mut now_its_mine = mine;
    *now_its_mine +=2;
    println!("{}", now_its_mine);
    //
    //println!("{}", mine);
    //borrow of moved value: `mine`

    let mut var = 4;
    println!("{}", var);
    var = 3;
    let ref_var: &i32 = &var;
    
    println!("{}", var);
    println!("{}", *ref_var);



}
