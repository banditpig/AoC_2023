//n * (T -n) > D
fn find_valid_n_range(t: f64, d: f64) -> (i32, i32) {
    //-n^2  + nt -d > 0
    //ax^2 + bx +c
    //x = (-b +/- sqrt( b*b - 4*a*c))2*a
    let a = -1.0;
    let b = t;
    let c = -d;
    let n1 = ((-b + (b * b - 4.0 * a * c).sqrt()) / 2.0 * a).ceil();
    let n2 = ((-b - (b * b - 4.0 * a * c).sqrt()) / 2.0 * a).floor();

    (n1 as i32, n2 as i32)
}
pub fn part1() {
    let n1 = find_valid_n_range(53., 275.);
    let n2 = find_valid_n_range(71., 1181.);
    let n3 = find_valid_n_range(78., 1215.);
    let n4 = find_valid_n_range(80., 1524.);

    let x = (n1.1 - n1.0 + 1) * (n2.1 - n2.0 + 1) * (n3.1 - n3.0 + 1) * (n4.1 - n4.0 + 1);
    println!("Day 6 Part 1:{}", x);
}
pub fn part2() {
    // Time:        53     71     78     80
    // Distance:   275   1181   1215   1524
    let vs = find_valid_n_range(53717880., 275118112151524.);
    println!("Day 6 Part 2:{}", vs.1 - vs.0 + 1);
}
