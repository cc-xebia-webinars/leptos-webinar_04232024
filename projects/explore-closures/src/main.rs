fn main() {
    // let x = 2;
    let mut x = 2;

    // let add_two = |y| x + y;
    let add_two = move |y| x + y;

    let result = add_two(3);
    println!("result: {} + 3 => {}", x, result);

    x = 3;

    // when this code is commented, the assignment to x above works because the closure is destroyed
    // but if the command is uncommented, then the closure is still alive and the assignment to x
    // above will fail because x is still borrowed by the closure.
    // using move will fix this issue.
    let result = add_two(3);
    println!("result: {} + 3 != {}", x, result);
}
