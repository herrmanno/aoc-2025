//! # Day 11 Reactor

use aoc_runner::Day;
use fxhash::FxHashMap;

#[derive(Default, Clone)]
pub struct Day11 {
    devices: Vec<Device>,
}

impl Day for Day11 {
    type Result1 = usize;
    type Result2 = usize;

    fn parse(&mut self, input: &str) {
        self.devices = input.lines()
            .map(Device::from)
            .collect();
    }

    fn part1(&mut self) -> Self::Result1 {
        fn solve(devices: &[Device], name: &str) -> usize {
            if name == "out" {
                return 1;
            }

            let device = devices.iter().find(|it| it.name == name).unwrap();
            device.out.iter().map(|it| solve(devices, it)).sum()
        }

        solve(&self.devices, "you")
    }

    fn part2(&mut self) -> Self::Result2 {
        fn solve<'a, 'b>(devices: &'a [Device], name: &'a str, fft_seen: bool, dac_seen: bool, memo: &'b mut FxHashMap<(&'a str, bool, bool), usize>) -> usize {
            if name == "out" {
                return (fft_seen && dac_seen) as usize;
            }

            if let Some(result) = memo.get(&(name, fft_seen, dac_seen)) {
                return *result;
            }

            let device = devices.iter().find(|it| it.name == name).unwrap();
            let result = {
                let fft_seen = fft_seen || device.name == "fft";
                let dac_seen = dac_seen || device.name == "dac";
                device.out.iter().map(|it| solve(devices, it, fft_seen, dac_seen, memo)).sum()
            };

            memo.insert((name, fft_seen, dac_seen), result);

            result
        }

        solve(&self.devices, "svr", false, false, &mut Default::default())
    }
}

#[derive(Default, Clone)]
struct Device {
    name: String,
    out: Vec<String>
}

impl From<&str> for Device {
    fn from(value: &str) -> Self {
        let (name, outs) = value.split_once(":").unwrap();
        Self {
            name: name.trim().to_string(),
            out: outs.split_whitespace().map(str::to_string).collect()
        }
    }
}

#[cfg(test)]
mod test {
    use indoc::indoc;
    use super::*;

    const INPUT_1: &str = indoc!{"
        aaa: you hhh
        you: bbb ccc
        bbb: ddd eee
        ccc: ddd eee fff
        ddd: ggg
        eee: out
        fff: out
        ggg: out
        hhh: ccc fff iii
        iii: out
    "};

    #[test]
    fn part_1() {
        let mut day = Day11::default();
        day.parse(INPUT_1);
        assert_eq!(day.part1(), 5);
    }

    const INPUT_2: &str = indoc!{"
        svr: aaa bbb
        aaa: fft
        fft: ccc
        bbb: tty
        tty: ccc
        ccc: ddd eee
        ddd: hub
        hub: fff
        eee: dac
        dac: fff
        fff: ggg hhh
        ggg: out
        hhh: out
    "};

    #[test]
    fn part_2() {
        let mut day = Day11::default();
        day.parse(INPUT_2);
        assert_eq!(day.part2(), 2);
    }
}
