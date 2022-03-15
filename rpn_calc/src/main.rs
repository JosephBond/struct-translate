#![feature(trace_macros)]
use text_io::read;

macro_rules! rpn {
    (@op [ $b:expr, $a:expr $(, $stack:expr)* ] $op:tt $($rest:tt)*) => {
        rpn!([ $a $op $b $(, $stack)* ] $($rest)*)
    };

    ($stack:tt + $($rest:tt)*) => {
        rpn!(@op $stack + $($rest)*)
    };

    ($stack:tt - $($rest:tt)*) => {
        rpn!(@op $stack - $($rest)*)
    };

    ($stack:tt * $($rest:tt)*) => {
        rpn!(@op $stack * $($rest)*)
    };

    ($stack:tt / $($rest:tt)*) => {
        rpn!(@op $stack / $($rest)*)
    };

    ([ $($stack:expr),* ] $num:tt $($rest:tt)*) => {
        rpn!([ $num $(, $stack)* ] $($rest)*)
    };
    ([ $result:expr ]) => {
        $result
    };

    ($($tokens:tt)*) => {
        rpn!([] $($tokens)*)
    };
}

fn main() {
    trace_macros!(true);
    let val = rpn!(4 1 - 3 + 2 *);
    trace_macros!(false);
    println!("{}", val);
    // println!("Expanding rpn!(4 1 - 3 + 2 *)");
    // println!("Matched rule @empty_stack, next call: rpn!([] 4 1 - 3 + 2 *)");
    // println!("mdb> query $stack");
    // println!("$stack = []");
    // println!("mdb> step 1");
    // println!("Expanding rpn!([] 4 1 - 3 + 2 *)");
    // println!("Matched rule @next, next call: rpn!([4] 1 - 3 + 2 *)");
    // println!("mdb> step 1");
    // println!("\n\t...\n");
    // println!("Expanding rpn!([4 - 1 + 3 * 2])");
    // println!("Matched rule @result, next call (4 - 1 + 3) * 2");
    // println!("mdb> step 1");
    // println!("No further rules to be matched, final expression: ");
    // println!("(4 - 1 + 3) * 2");
    // println!("mdb> eval $result");
    // println!("eval($result) = 12");

    

}
