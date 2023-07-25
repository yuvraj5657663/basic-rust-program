fn print_distance(point: (f32, f32)) -> f32{
    let origin: (f32, f32) = (0.0, 0.0);
    ((point.0 - origin.0)as f32).abs() + ((point.1 - origin.1)as f32).abs()
}

fn main() {
    /* In this question, you are tasked with writing a function that takes a tuple called point representing the x-axis and y-axis coordinates of a point (similar to Question 1, Assignment 2). The function should calculate the distance between the given point and the origin (0, 0) and return the computed distance.

    The template of the function is given below

    fn print_distance(point: (f32, f32)) -> f32{
        // your code here
    }

    // Use the skeleton of the main below to call the function.

    fn main() {

        println!("The distance of the point from the origin is {}", print_distance((5.0,4.0)));

    }

    Notes on solving:

    1. Begin by destructuring the tuple parameter inside the function. This means assigning the values of the tuple to separate variables. This improves code readability as it allows us to use meaningful variable names instead of point.0 and point.1.

    2. Next, calculate the distance between the given point and the origin (0, 0). You can refer back to Question 3, Assignment 2 for a recap on the distance formula and the relevant functions to compute the distance.

    3. Keep in mind that when calling the function, it is important to use double parentheses to indicate the input as a tuple. Using a single parenthesis may cause the compiler to assume multiple inputs for the function, which is not the intended behavior in this exercise.

    By following these steps, you can write a clear and effective function to compute the distance of a point from the origin based on its x and y coordinates. */

    let point = (5.0,4.0);

    println!("The disatance of the point from the origin is {}", print_distance((point.0 , point.1)));
}
