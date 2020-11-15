use std::io;

fn main() {
   println!("숫자를 맞추어 주세요");

   println!("생각하는 정답을 입력해 주세요");

   let mut guess = String::new();

   io::stdin().read_line(&mut guess)
       .expect("값을 읽어오지 못 했습니다.");
   println!("입력한 값: {}", guess);
}
