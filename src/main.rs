// fn main() {
//     let a = [10, 20, 30, 40, 50];
//     let mut i = 0;

//     while i < 5 {
//         println!("the value is: {}", a[i]);

//         i += 1;
//     }
// }

// This approach is **error-prone**; the code would **panic** if we change the definition of the `a` array for having **four elements** instead of **five** but forgetting to update the condition to `while index < 4`.

// A better alternative is using a `for` loop and execute some code for each item in a collection (*array*):

fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}