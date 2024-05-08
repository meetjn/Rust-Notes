fn main() {
    let num = 7;
    let result = fibo(num);
    println!("{}", result);

}
fn fibo(num: i32) -> i32{
    if num < 2{
        return num;
    }
    return fibo(num-1) + fibo(num - 2);
}

// fn main() {
//     let mut counter = 0;

//     let result = loop {
//         counter += 1;

//         if counter == 10 {
//             break counter * 2;
//         }
//     };

//     println!("The result is {result}");
// }

// fn main() {
//     for number in (1..4).rev() {
//         println!("{number}");
//     }
//     println!("fir bakchodi!");
// }