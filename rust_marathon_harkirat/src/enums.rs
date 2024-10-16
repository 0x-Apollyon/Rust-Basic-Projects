enum Shape {
    Circle(f64) , //radius
    Rectangle(f64 , f64) //height , width
}

fn main() {
    let mut my_shape = Shape::Circle(1.0); //makes radius of circle as 1.0
    println!("The radius of the circle is {}" , calc_area(my_shape));

    my_shape = Shape::Rectangle(10.0 , 20.0);
    println!("The area of the rectangle is {}" , calc_area(my_shape));
}

fn calc_area(shape : Shape) -> f64 {
    let area = match shape{
        Shape::Rectangle(a,b) => a*b ,
        Shape::Circle(r) => 3.141 * r * r
    };
    return area;

}