use std::{time::{Instant, Duration}, collections::BTreeMap, fmt::Display};

/// Scaffold metr an AoC runner
pub trait Analyzer {
    /// Called before running all puzzles
    fn before_all(&mut self) {}

    /// Called after running all puzzles
    fn after_all(&mut self) {}

    /// Called before running a day's puzzle
    fn before_day(&mut self, _day: usize) {}

    /// Called after running a day's puzzle
    fn after_day(&mut self, _day: usize) {}

    /// Called before parsing a puzzle's input
    fn before_parse(&mut self, _day: usize) {}

    /// Called after parsing a puzzle's input
    fn after_parse(&mut self, _day: usize) {}

    /// Called before running a puzle's part
    fn before_part(&mut self, _day: usize, _part: usize) {}

    /// Called after running a puzle's part
    fn after_part(&mut self, _day: usize, _part: usize) {}
}

#[derive(Default)]
pub struct TimeAnalyzer {
    start_all: Option<Instant>,
    time_all: Option<Duration>,
    start_day: Option<Instant>,
    time_days: BTreeMap<usize, Duration>,
    start_parse: Option<Instant>,
    time_parse: BTreeMap<usize, Duration>,
    start_part: Option<Instant>,
    time_part: BTreeMap<(usize,usize), Duration>,
}

/// A simple analyzer that measures and prints run times
impl TimeAnalyzer {
    pub fn new() -> Self {
        TimeAnalyzer::default()
    }

    fn days(&self) -> Vec<usize> {
        self.time_days.keys().cloned().collect()
    }

    fn total_parse(&self) -> Duration {
        self.days().iter().map(|day| self.time_parse.get(day).unwrap()).cloned().reduce(|a,b| a.saturating_add(b)).unwrap_or_default()
    }

    fn total_part1(&self) -> Duration {
        self.days().iter().filter_map(|day| self.time_part.get(&(*day, 1)))
            .cloned().reduce(|a,b| a.saturating_add(b)).unwrap_or_default()
    }

    fn total_part2(&self) -> Duration {
        self.days().iter().filter_map(|day| self.time_part.get(&(*day, 2)))
            .cloned().reduce(|a,b| a.saturating_add(b)).unwrap_or_default()
    }

    fn total(&self) -> Duration {
        self.time_all.unwrap_or(self.time_days.values().sum())
    }

    fn report(&mut self) {
        fn print_line(day: impl Display, parse: Duration, part1: Duration, part2: Duration, total: Duration) {
            print!("| {:>6} |", day);
            print_col(parse);
            print_col(part1);
            print_col(part2);
            print_col(total);
            print!("\n");
        }

        fn print_col(duration: Duration) {
            if duration.as_secs() > 0 {
                print!(" {:>7}s |", duration.as_secs());
            } else if duration.as_millis() > 0 {
                print!(" {:>6}ms |", duration.as_millis());
            } else if duration.as_micros() > 0 {
                print!(" {:>6}μs |", duration.as_micros());
            } else {
                print!(" {:>6}ns |", duration.as_nanos());
            }
        }

        println!();
        println!("+--------|----------|----------|----------|----------+");
        println!("| Day    | Parse    | Part 1   | Part 2   | Total    |");
        println!("+--------|----------|----------|----------|----------+");
        print_line("Total", self.total_parse(), self.total_part1(), self.total_part2(), self.total());
        println!("+----------------------------------------------------+");
        for ref day in self.days() {
            print_line(
                day,
                self.time_parse.get(day).cloned().unwrap_or_default(),
                self.time_part.get(&(*day, 1)).cloned().unwrap_or_default(),
                self.time_part.get(&(*day, 2)).cloned().unwrap_or_default(),
                self.time_days.get(day).cloned().unwrap_or_default(),
            );
        }
        println!("+--------|----------|----------|----------|----------+");
    }
}

impl Analyzer for TimeAnalyzer {
    fn before_all(&mut self) {
        self.start_all = Some(Instant::now());
    }

    fn after_all(&mut self) {
        self.time_all = Some(self.start_all.unwrap().elapsed());
        self.report();
    }

    fn before_day(&mut self, _day: usize) {
        self.start_day = Some(Instant::now());
    }

    fn after_day(&mut self, day: usize) {
        self.time_days.insert(day, self.start_day.unwrap().elapsed());

        if self.start_all.is_none() {
            self.report();
        }
    }

    fn before_parse(&mut self, _day: usize) {
        self.start_parse = Some(Instant::now());
    }

    fn after_parse(&mut self, day: usize) {
        self.time_parse.insert(day, self.start_parse.unwrap().elapsed());
    }

    fn before_part(&mut self, _day: usize, _part: usize) {
        self.start_part = Some(Instant::now());
    }

    fn after_part(&mut self, day: usize, part: usize) {
        self.time_part.insert((day, part), self.start_part.unwrap().elapsed());
    }
}
