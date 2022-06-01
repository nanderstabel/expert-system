use super::SequentialPermutationIter;

use crossbeam::channel::{bounded, Receiver};
use std::collections::HashMap;

const PARALLEL_THREAD_BUFF_SIZE: usize = 4000;

pub struct ParallelPermutationIter {
    pub variables: Vec<char>,
    pub thread_count: usize,
    pub receiver: Receiver<String>,
}

impl ParallelPermutationIter {
    pub fn new(
        formula: String,
        variables: Vec<char>,
        pos_map: HashMap<char, Vec<usize>>,
        thread_count: usize,
    ) -> ParallelPermutationIter {
        let total_end = 1 << variables.len();
        let mut chunked_iters = Vec::with_capacity(thread_count);
        match thread_count {
            0 => panic!("thread_count must be greater than 0"),
            1 => {
                chunked_iters.push(SequentialPermutationIter::new(
                    formula,
                    variables.clone(),
                    pos_map,
                    0,
                    total_end,
                ));
            }
            _ => {
                let step = total_end / thread_count;
                let mut start;
                let mut end;
                for i in 0..(thread_count) {
                    start = step * i;
                    end = start + step;
                    if i == (thread_count - 1) {
                        end = total_end;
                    }

                    chunked_iters.push(SequentialPermutationIter::new(
                        formula.clone(),
                        variables.clone(),
                        pos_map.clone(),
                        start,
                        end,
                    ));
                }
            }
        }

        let (sender, receiver) = bounded(PARALLEL_THREAD_BUFF_SIZE * thread_count);
        for iter in chunked_iters {
            let thread_sender = sender.clone();
            std::thread::spawn(move || {
                for permutation in iter {
                    thread_sender.send(permutation).unwrap();
                }
            });
        }
        ParallelPermutationIter {
            variables,
            thread_count,
            receiver,
        }
    }
}

impl Iterator for ParallelPermutationIter {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.receiver.recv().ok()
    }
}
