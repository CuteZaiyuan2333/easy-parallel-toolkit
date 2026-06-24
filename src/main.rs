use easy_parallel_toolkit::{
    Runnable,
    RuntimeMachine,
};
use start_from_struct::main;

struct Task2{
    id: i32,
    number1: i32,
    number2: i32,
}impl Task2{
    fn init(id: i32, number1: i32, number2: i32) -> Self{
        Self{
            id,
            number1,
            number2,
        }
    }
}impl Runnable<(i32, i32)> for Task2{
    fn enter(self: &mut Self) -> (i32, i32){
        let result = self.number1 + self.number2;
        return (self.id, result);
    }
}

#[main]
struct Main{
    
}impl Main{
    fn init() -> Self {
        Main{
            
        }
    }
    fn main(self: &Self) {
        let mut runtime: RuntimeMachine<(i32, i32)> = RuntimeMachine::init();
        let mut i:i32 = 0;
        for a in 0..2{
            for b in 0..2{
                runtime.static_push(Task2::init(i, a, b));
                i += 1;
            }
        }
        runtime.compute(3);
        i = 0;
        for a in 2..4{
            for b in 2..4{
                runtime.static_push(Task2::init(i, a, b));
                i += 1;
            }
        }
        runtime.obtain();
        for (id, recive) in runtime.result{
            println!("{}:{}", id, recive);
        }
    }
}