// This shows a very simple and obvious in hindisght workaround for this error:
// --> src/main.rs:17:15
// |
// 17 |         (a,b) = add_seven(a,b); // I need to get this from the inner scope
// |         ----- ^
// |         |
// |         cannot assign to this expression
// |
// = note: see issue #71126 <https://github.com/rust-lang/rust/issues/71126> for more information
// and following that, this fix is also shown here:
// issue #78748 <https://github.com/rust-lang/rust/pull/78748>
// (a,b) = (1,2)
// ... becomes ...

// {
//   let (lhs0,lhs1) = (1,2);
//   a = lhs0;
//   b = lhs1;
// }


fn add_seven(x:u32, y:u32) -> (u32, u32) {
    
    (x+7, y+7)
}

fn main() {
    
    // method 1 does not work, this raises issue #71126
    
    let mut a:u32 = 21;
    let mut b:u32 = 66;

// This commented out set of lines show the problem
// Uncomment to see:

    // { // these parentheses represent a number of nested loops
    //     (a,b) = add_seven(a,b); // I need to get this from the inner scope
    // }
    println!("Method 1, won't compile: a,b = {},{}",a,b);


    // method 2, suggested work around: 
    // adding let does not work in all cases!
    // Because the inner scope changes are ignored
    
    let mut a:u32 = 21;
    let mut b:u32 = 66;

    { // these parentheses represent a number of nested loops
        let (a,b) = add_seven(a,b); // I need to get this from the inner scope
    }
    println!("Method 2, incorrect result: a,b = {},{}",a,b);

    
    // method 3 is the correct workaround

    let mut a:u32 = 21;
    let mut b:u32 = 66;
    
    { // this parentheses represent a number of nested loops
        let (x,y) = add_seven(a,b); // I need to get this from the inner scope
        a=x;
        b=y;
    }
    println!("Method 3, correct workaround: a,b = {},{}",a,b);
    

}