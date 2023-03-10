fn main() {
    let s = "hello";
    // 不可変 const スタック
    let mut _char = String::from("hello");
    // 可変 var ヒープ
    _char.push_str(" world");
    // 可変だから追加可能
    println!("{}", _char);

    let s1 = String::from("hello");
    // let s2 = s1;
    let s2 = s1.clone();
    println!("{}", s1);
    // アドレスコピーするため、関数を抜けると解放が二重に行われる


    let s3 = String::from("hello");
    takes_ownership(s3);

    let x = 5;
    makes_copy(x);

    // s3はString型であるためtakes_ownershipで有効ではなくなる
    // xはi32であるためコピーされる 所有権は移らないためmakes_copy後でも使える

    let s4 = String::from("hello");
    let(s5,len) = calculate_length(s4);
    println!("the length of {} is {}", s5,len);

    let s6 = String::from("hello");
    let len = calculate_length_with_pointer(&s6);
    println!("the length of {} is {}", s6,len);
    let mut s7 = String::from("hello");
    change(&mut s7);
    println!("{}", s7);

    let mut s8 = String::from("hello");
    let r1 = &s8;
    let r2 = &s8;
    // let r3 = &mut s8;
    // r1 r2は不可変で参照しているためr3で可変参照することはできない

    let reference_to_nothing = dangle();

    let mut s9 = String::from("hello world");
    let word = first_word(&s9);

    s9.clear();
    println!("{}", s9);
}

fn takes_ownership(some_string:String){
    println!("{}", some_string);
}
fn makes_copy(some_integer:i32){
    println!("{}", some_integer);
}
fn gives_ownership()-> String{
    let some_string = String::from("hello");
    some_string
}
// 引数なしでsome_stringを返す

fn takes_and_gives_back(a_string:String) -> String{
    a_string
}
//  引数渡して返り値もある

fn calculate_length(s:String)->(String,usize){
    let length = s.len();
    (s,length)
}
fn calculate_length_with_pointer(s:&String)->usize{
    s.len()
}
fn change(some_string:&mut String){
    some_string.push_str("world");
}
fn dangle()->String{
    let s = String::from("hello");

    s
}
fn first_word(s:&String)->usize{
    let bytes = s.as_bytes();

    for (i,&item) in bytes.iter().enumerate(){
        if item == b' '{
            return i;
        }
    }
    s.len()
}