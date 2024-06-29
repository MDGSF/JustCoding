use super::*;
use anyhow::{bail, Result};
use log::info;
use rand::rngs::ThreadRng;
use rand::Rng;
use std::fs::{File, OpenOptions};
use std::io::Write;

pub struct GenNum {
    out: String,
    seperate: Seperate,
    count: usize,
    numtype: String,
}

impl GenNum {
    pub fn new(out: String, seperate: u8, count: usize, numtype: String) -> Self {
        GenNum {
            out,
            seperate: if seperate == 1 {
                Seperate::Line
            } else {
                Seperate::Space
            },
            count,
            numtype,
        }
    }

    pub fn generate(&self) -> Result<()> {
        info!(
            "gennum generate start, out: {:?}, seperate: {:?}, count: {:?}",
            self.out, self.seperate, self.count
        );

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(self.out.clone())?;

        let mut rng = rand::thread_rng();

        for _ in 0..self.count {
            self.write_num(&mut file, &mut rng)?;
            self.write_seperate(&mut file)?;
        }

        Ok(())
    }

    fn write_num(&self, file: &mut File, rng: &mut ThreadRng) -> Result<()> {
        if self.numtype == "u8" {
            let number: u8 = rng.gen();
            write!(file, "{}", number)?;
        } else if self.numtype == "u16" {
            let number: u16 = rng.gen();
            write!(file, "{}", number)?;
        } else if self.numtype == "u32" {
            let number: u32 = rng.gen();
            write!(file, "{}", number)?;
        } else if self.numtype == "u64" {
            let number: u64 = rng.gen();
            write!(file, "{}", number)?;
        } else if self.numtype == "i8" {
            let number: i8 = rng.gen();
            write!(file, "{}", number)?;
        } else if self.numtype == "i16" {
            let number: i16 = rng.gen();
            write!(file, "{}", number)?;
        } else if self.numtype == "i32" {
            let number: i32 = rng.gen();
            write!(file, "{}", number)?;
        } else if self.numtype == "i64" {
            let number: i64 = rng.gen();
            write!(file, "{}", number)?;
        } else {
            bail!("Invalid num type: {}", self.numtype);
        }

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
