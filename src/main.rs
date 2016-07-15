mod rop;

use rop::utils::RopResult;
use rop::utils::succeed;

fn main() {
    let t = RopResult::Success(5, Option::None::<Vec<String>>);
    match t {
        RopResult::Success(x, _) => println!("This is the success {}", x),
        RopResult::Failure(_) => println!("Not what we expected"), 
    }
    let u = succeed::<_, ()>(5);
    match u {
        RopResult::Success(x, _) => println!("Success {}", x),
        RopResult::Failure(_) => println!("Not what we expected"), 
    }
}

