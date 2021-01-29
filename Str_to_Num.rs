fn main() {
//let hoge = '\u{200B}';

let str1= "A";
let hoge = str1.as_bytes(); //文字列をバイト列 [65]に変換する
println!("{:0b}",hoge[0]);


let str2 = "A".as_bytes()[0];
println!("{}",format!("{:b}", str2)); //数値をN進数のStringに変換する format!()
let huga:String = format!("{:b}", str2);
let num:i32=huga.parse().unwrap(); //数値に変換

let str3 = "ABC".as_bytes(); 
println!("{}",String::from_utf8(str3.to_vec()).unwrap());

}