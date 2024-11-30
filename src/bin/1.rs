
#[aoc::main(1)]
fn main(part: i32, input: &str) -> usize {
    if part == 0 {
        input.lines().map(find_begin_end_digit).sum::<u32>() as usize
    }

    return 3;
        /*
    else {
        input.lines().map(find_begin_end_digit_or_word).sum::<u32>() as usize
    }*/
}
