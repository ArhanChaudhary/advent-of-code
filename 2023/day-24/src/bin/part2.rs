fn part2(_: &str) -> usize {
    /*
       I couldn't figure out how to solve this algebraically so I resorted to using
       sympy in python, which doesn't have a clean rust equivalent :(
       not really satisfied with my solution but I've already spent enough time on this

       >>> c_x, c_y, c_z, d_x, d_y, d_z = symbols('c_x c_y c_z d_x d_y d_z')
       >>> eq1 = Eq((c_x - 147847636573416) / (185 - d_x), (c_y - 190826994408605) / (49 - d_y))
       >>> eq2 = Eq((c_y - 190826994408605) / (49 - d_y), (c_z - 140130741291716) / (219 - d_z))
       >>> eq3 = Eq((c_x - 287509258905812) / (-26 - d_x), (c_y - 207449079739538) / (31 - d_y))
       >>> eq4 = Eq((c_y - 207449079739538) / (31 - d_y), (c_z - 280539021150559) / (8 - d_z))
       >>> eq5 = Eq((c_x - 390970075767404) / (-147 - d_x), (c_y - 535711685410735) / (-453 - d_y))
       >>> eq6 = Eq((c_y - 535711685410735) / (-453 - d_y), (c_z - 404166182422876) / (-149 - d_z))
       >>> eq7 = Eq((c_x - 306391780523937) / (-24 - d_x), (c_y - 382508967958270) / (-274 - d_y))
       >>> eq8 = Eq((c_y - 382508967958270) / (-274 - d_y), (c_z - 264612201472049) / (28 - d_z))
       >>> solve((eq1, eq2, eq3, eq4, eq5, eq6, eq7, eq8), (c_x, c_y, c_z, d_x, d_y, d_z))
       [(239756157786030, 463222539161932, 273997500449219, 47, -360, 18)]
       >>> 239756157786030+463222539161932+273997500449219
       976976197397181
       >>>
    */
    unimplemented!();
}

fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}
