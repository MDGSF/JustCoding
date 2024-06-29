use anyhow::Result;
use log::info;
use std::fs::File;
use std::io::Write;
use std::io::{BufRead, BufReader};

pub struct SortBigFile {
    input: String,
    output: String,
    small_file_num: usize, // split how many small files to process
}

impl SortBigFile {
    pub fn new(input: &str, output: &str) -> Self {
        SortBigFile {
            input: input.to_string(),
            output: output.to_string(),
            small_file_num: 10,
        }
    }

    pub fn process(&self) -> Result<()> {
        info!(
            "sort big file, input: {}, output: {}",
            self.input, self.output
        );

        let count = self.count_input_file_lines()?;
        let small_count = if count % self.small_file_num == 0 {
            count / self.small_file_num
        } else {
            count / self.small_file_num + 1
        };
        info!(
            "count: {}, small_file_num: {}, small_count: {}",
            count, self.small_file_num, small_count
        );

        let small_files = self.sort_small_file(small_count)?;

        self.merge_small_sorted_file(small_files)?;

        Ok(())
    }

    fn count_input_file_lines(&self) -> Result<usize> {
        let file = File::open(self.input.clone())?;
        let reader = BufReader::new(file);
        let lines = reader.lines();

        let mut line_count = 0;

        for line_result in lines {
            let line = line_result?;
            if !line.trim().is_empty() {
                line_count += 1;
            }
        }

        Ok(line_count)
    }

    fn sort_small_file(&self, small_count: usize) -> Result<Vec<String>> {
        let mut small_files = Vec::new();
        let mut current_file_index = 0;
        let mut current_count: usize = 0;
        let mut current_nums = Vec::new();

        let file = File::open(self.input.clone())?;
        let reader = BufReader::new(file);
        let lines = reader.lines();
        for line_result in lines {
            let line = line_result?;
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            current_count += 1;
            current_nums.push(line.parse::<i32>()?);
            if current_count < small_count {
                continue;
            }

            current_nums.sort();

            let current_file_name = format!("part_{}.txt", current_file_index);
            info!("create small file: {}", current_file_name);
            small_files.push(current_file_name.clone());
            let mut current_file = File::create(current_file_name)?;

            for num in current_nums.iter() {
                writeln!(current_file, "{}", num)?;
            }

            current_file_index += 1;
            current_count = 0;
            current_nums.clear();
        }

        Ok(small_files)
    }

    fn merge_small_sorted_file(&self, small_files: Vec<String>) -> Result<()> {
        info!("merge small sorted file: {:?}", small_files);

        let file_num = small_files.len();
        let mut readers = Vec::with_capacity(file_num);
        for file_name in small_files.iter() {
            let file = File::open(file_name)?;
            let reader = BufReader::new(file);
            let lines = reader.lines();
            readers.push(lines);
        }

        let mut output_file = File::create(self.output.clone())?;
        let mut finished_file = 0;
        let mut finished: Vec<bool> = vec![false; file_num];
        let mut nums: Vec<Option<i32>> = vec![None; file_num];

        while finished_file < file_num {
            let mut small_num = i32::MAX;
            let mut small_num_index = 0;
            for i in 0..file_num {
                if finished[i] {
                    continue;
                }

                if nums[i].is_none() {
                    match readers[i].next() {
                        Some(line_result) => {
                            let line = line_result?;
                            let line = line.trim();
                            nums[i] = Some(line.parse::<i32>()?);
                        }
                        None => {
                            finished[i] = true;
                            finished_file += 1;
                            continue;
                        }
                    }
                }

                if nums[i].unwrap() < small_num {
                    small_num = nums[i].unwrap();
                    small_num_index = i;
                }
            }

            nums[small_num_index] = None;
            writeln!(output_file, "{}", small_num)?;
        }

        Ok(())
    }
}
