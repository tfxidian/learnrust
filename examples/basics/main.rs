
fn add2(x: i32, y: i32) -> i32{
    x+y
}

fn main(){
    let x: i32 =1;
    let y: i32 =13i32;
    let f: f64 =1.3f64;
    println!("{}\t{}\t{}", x, y, f);
    //x = 2; cannot assign twice to immutable variable
    let implicit_x = 1;
    let implicit_y = 1.3;
    println!("{}\t{}", implicit_x, implicit_y);
    let sum = x+y+13;
    println!("{}", sum);
    let sum = x+y+13;
    let mut mutable = 1;
    mutable = 4;
    mutable +=2;
    println!("{}", mutable);

    println!("hello!");
    let x: &str = "hello world!";
    println!("{}", x);
    let s: String = "Hello world".to_string();

    let s_slice :&str = &s;

    println!("{}{}", s, s_slice);
    // a fixed sized array
    let four_ints :[i32;4] = [1,2,3,4];
    println!("{:?}", four_ints);
    // a dynamic array 
    let mut vector: Vec<i32> = vec![1,2,3,4];
    vector.push(5);

    let slice: &[i32] =&vector;
    println!("{:?}{:?}", vector,slice); 
    let x: (i32, &str, f64) = (1, "hello", 3.4);
    let (a, b, c) = x;
    println!("{}{}{}",a,b,c);
    println!("{}", x.1);

}



/*
fn main(){
  let sf = f();  // f()返回值是一个无效引用
}

fn f() -> &String {  //报错 提示expected named lifetime parameter
  let s = String::from("hello");
  &s  // 返回s的引用
}   // s跳出作用域，堆中String字符串被释放

*/
