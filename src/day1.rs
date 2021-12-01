#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<u32> {
    input.lines().map(|x| { x.trim().parse().unwrap() }).collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[u32]) -> u32 {
    input.iter().fold( (0u32, u32::MAX), |(mut inc, last), &val| {
        if last < val {
            inc += 1;
        }
        (inc,val)
    }).0
}

#[derive(Default)]
struct Data {
    increments: u32,
    comp: u32,
    A: u32,
    B: u32,
    C: u32
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[u32]) -> u32 {
    let setup : Vec<&mut u32> = input[0..2].iter().scan(0, |sum, val| {*sum += val; Some(sum)}).collect::<Vec<_>>();
    0
    // let bookkeeping = Data { A: set }
    // input.iter().fold(Data{}, | mut data, &val| {
    //     match data.comp {
    //         0 => Data {
    //                 A: val,
    //                 comp: 1,
    //                 ..data
    //         },
    //         1 => Data {
    //                 A: data.A + val,
    //                 B: val,
    //                 comp: 2,
    //                 ..data
    //         },
    //         2 => Data {
    //             A: data.A + val,
    //             B: data.B + val,
    //             C: val,
    //             comp: 3
    //             ..data
    //         }
    //         3 => Data {
    //             A: val,
    //             B: data.B
    //             if data.A < data.B+val {

    //             }
    //         }

    //     }
    // (A, B)
    //     +A
    //     (+A, +B)
    //     (+A, +B, +C)
    //     compare (A, B) add to inc -> (D, +B, +C)
    //     (+D, +C, +E)
}
