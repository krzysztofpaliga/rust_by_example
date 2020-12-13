fn main() {
    let logical: bool = true;
    let a_float: f64 = 1.0;
    let an_integer = 5i32;

    let default_float = 3.0;
    let default_integer = 7;

    let mut inferred_type = 12;
    inferred_type =4294967296i64;

    let mut mutable = 12;
    mutable = 21;

    // mutable = true;

    let mutable = true;

    println!("1 + 2 = {}", 1u32 + 2);
    println!("1 - 2 = {}", 1i32 - 2);

    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    println!("One million is written as {}", 1_000_000u32);

    fn reverse(pair: (i32, bool)) -> (bool, i32) {
        let (integer, boolean) = pair;
        (boolean, integer)
    }

    println!("reverse(3, true): {:?}", reverse((3, true)));

    #[derive(Debug)]
    struct Matrix(f32, f32, f32, f32);

    let long_tuple = (1u8, 2u16, 3u32, 4u64,
                                    -1i8, -2i16, -3i32, -4i64,
                                    0.1f32, 0.2f64,
                                    'a', true);
    println!("long tuple first vaule: {}", long_tuple.0);
    println!("long tuple second value: {}", long_tuple.1);

    let tuple_of_tuples = ((1u8, 2u16, 3u32), (4u64, -1i8), -2i16);

    println!("tuple of tuples: {:?}", tuple_of_tuples);

    let pair = (1, true);
    println!("pair is {:?}", pair);
    println!("the reversed pair is {:?}", reverse(pair));

    println!("one element tuple : {:?}", (5u32,));
    println!("just an integer: {:?}", (5u32));

    let tuple = (1, "hello", 4.5, true);
    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);
    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix);

    use std::fmt;
    impl fmt::Display for Matrix {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

            write!(f, "( {} {} )\n( {} {} )", &self.0, &self.1, &self.2, &self.3)
        }
    }

    println!("matrix:\n{}", matrix);

    fn transpose(m: Matrix) -> Matrix {
        // let Matrix(a1, a2, b1, b2) = m;
        // Matrix(a1, b1, a2, b2);
        Matrix(m.0, m.2, m.1, m.3)
    }

    println!("transposed matrix:\n{}", transpose(matrix));

    use std::mem;
    fn analyze_slice(slice: &[i32]) {
        println!("first element of the slice: {}", slice[0]);
        println!("the slice has {} elements", slice.len());
    }

    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    let ys: [i32; 500] = [0; 500];
    println!("first element of the array xs: {}", xs[0]);
    println!("second element of the array xs: {}", xs[1]);
    println!("first element of the array ys: {}", ys[0]);
    println!("second element of the array ys: {}", ys[1]);

    println!("number of elements in array xs: {}", xs.len());
    println!("array xs occupies {} bytes", mem::size_of_val(&xs));
    println!("borrow the whole array xs as as slice");
    analyze_slice(&xs);
    println!("borrow a section of the array ys as a slice");
    analyze_slice(&ys[1..4]);

    // println!("{}", xs[5]);
}