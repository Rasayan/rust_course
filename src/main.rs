fn main() {
    let mut x = 5;
    println!("The value of x is {}",x);

    x = 6;
    println!("The value of x is {}",x);

    // const FOLLOWERS: u32 = 1_000_000;

    let tup = ("Rsaayan", 100_000);
    let (channel, sub) = tup;

    println!("The channel {} has count {}",channel,sub);

    // const ARR: [i32; 3] = [10,19,20];
    // println!("The value of x is {}", arr[2]);

    println!("The value of the fn is {}",my_function(2,5));

    let _a = [1,2,3,4,5,5,6];

    //for i in a.iter{
        //....
    //}

    //for num in 1..9 {
        //....
    //}
}


fn my_function(x: i32, y: i32) -> i32 {
    let sum = x+y;
    if sum < 5 {
        sum+1
    } else if sum > 5 {
        sum-1
    } else {
        sum
    }

    // 'condition' variable
    // if condition { ... } else if { ... } else { ... }

    /*
        let counter = 0;
        let result = loop {
            counter += 1;

            if counter =  10 {
                break counter;
            }
        };
     */
} 