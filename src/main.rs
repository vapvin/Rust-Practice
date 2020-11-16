use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
   println!("숫자를 맞추어 주세요");

   let secret_number = rand::thread_rng().gen_range(1, 101);

   println!("랜덤 숫자 {}", secret_number);

   loop {
    println!("생각하는 정답을 입력해 주세요");

    let mut guess = String::new();
 
    io::stdin().read_line(&mut guess)
        .expect("값을 읽어오지 못 했습니다.");
    let guess: u32 = guess.trim().parse()
        .expect("입력한 값이 숫자가 아닙니다.");
    println!("입력한 값: {}", guess);
    match guess.cmp(&secret_number){
        Ordering::Less => println!("입력한 숫자가 정답보다 작습니다."),
        Ordering::Greater => println!("입력한 숫자가 정다봅다 높습니다."),
        Ordering::Equal => {
            println!("정답입니다");
            break;
        },
    }
   }
}
