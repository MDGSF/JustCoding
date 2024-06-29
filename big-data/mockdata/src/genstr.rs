use super::*;
use anyhow::Result;
use log::info;
use rand::rngs::ThreadRng;
use rand::Rng;
use rand_distr::Alphanumeric;
use std::fs::{File, OpenOptions};
use std::io::Write;

pub struct GenStr {
    out: String,
    seperate: Seperate,
    count: usize,
    str_min_len: usize,
    str_max_len: usize,
}

impl GenStr {
    pub fn new(
        out: String,
        seperate: u8,
        count: usize,
        str_min_len: usize,
        str_max_len: usize,
    ) -> Self {
        GenStr {
            out,
            seperate: if seperate == 1 {
                Seperate::Line
            } else {
                Seperate::Space
            },
            count,
            str_min_len,
            str_max_len,
        }
    }

    pub fn generate(&self) -> Result<()> {
        info!(
            "genstr generate start, out: {:?}, seperate: {:?}, count: {:?}",
            self.out, self.seperate, self.count
        );

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(self.out.clone())?;

        let mut rng = rand::thread_rng();

        for _ in 0..self.count {
            self.write_str(&mut file, &mut rng)?;
            self.write_seperate(&mut file)?;
        }

        Ok(())
    }

    fn generate_rand_str(
        &self,
        rng: &mut ThreadRng,
        str_min_len: usize,
        str_max_len: usize,
    ) -> String {
        let len = rng.gen_range(str_min_len..=str_max_len);
        rng.sample_iter(&Alphanumeric)
            .take(len)
            .map(char::from)
            .collect()
    }

    fn write_str(&self, file: &mut File, rng: &mut ThreadRng) -> Result<()> {
        let s = self.generate_rand_str(rng, self.str_min_len, self.str_max_len);
        write!(file, "{}", s)?;

        Ok(())
    }

    fn write_seperate(&self, file: &mut File) -> Result<()> {
        match self.seperate {
            Seperate::Line => {
                writeln!(file, "")?;
            }
            Seperate::Space => {
                write!(file, " ")?;
            }
        }
        Ok(())
    }
}
