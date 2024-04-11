struct Point {
    x: f32, 
    y: f32,
    z: f32,
}

struct PointTuple(f32, f32, f32); // Tuples have to be declared without variable names

fn main() {
    // Here is a basic struct definition
    let point = Point {
        x: 32f32,
        y: 64f32,
        z: 96f32,
    };

    // here is the definition of a Struct Tuple we see a lot in Bevy
    let alt_point = PointTuple(1.0f32, 2.0f32, 3.0f32);

    // update the remaining fields not explicitly set
    let point_shorthand = Point {
        x: 64f32,
        ..point
    };
}