use crate::{output, Output};

use super::r#type::OutputList;

impl<'a> OutputList<'a> {
    pub fn new() -> Self {
        Self {
            output_list: vec![],
        }
    }

    pub fn new_from(output_list: Vec<Output<'a>>) -> Self {
        Self { output_list }
    }

    pub fn add(&mut self, output: Output<'a>) -> &Self {
        self.output_list.push(output);
        self
    }

    pub fn remove(&mut self, idx: usize) -> &Self {
        if idx >= self.output_list.len() {
            return self;
        }
        self.output_list.remove(idx);
        self
    }

    pub fn clear(&mut self) {
        self.output_list.clear();
    }

    pub fn run(&mut self) -> &Self {
        let output_list: Vec<Output<'a>> = self.output_list.clone();
        self.clear();
        for output in output_list {
            output.output();
        }
        self
    }

    pub fn query_idx(&self, idx: usize) -> Output {
        if idx >= self.output_list.len() {
            return Output::default();
        }
        self.output_list[idx].clone()
    }

    pub fn run_idx(&mut self, idx: usize) -> &Self {
        if idx >= self.output_list.len() {
            return self;
        }
        let output: Output<'_> = self.query_idx(idx);
        output.output();
        self
    }
}
