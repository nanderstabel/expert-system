use anyhow::{Context, Result};
use expert_system::*;
use std::fs::File;
use std::iter::Peekable;

pub enum Token {
    Implicator,
    Operator(char),
    Parenthesis(char),
    Identifier(String), // Maybe just a char instead of String? --> A, B, C, D
}

pub struct Node<'a> {
    token: &'a Token,
    left: Box<Option<Node<'a>>>,
    right: Box<Option<Node<'a>>>,
}

pub struct Parser {
    tokens: Vec<Token>,
}

impl<'a> Parser {
    pub fn new() -> Self {
        Parser { tokens: Vec::new() }
    }

    fn tokenize(&mut self) -> Result<Vec<Token>> {
        let tokens = Vec::new();
        let mut tokens = tokens.iter().peekable();
        self.get_implicator(&mut tokens)
            .context(format!("Could not tokenize: {}", "Line 4"))?;

        // implementation goes here
        todo!();
    }

    fn get_implicator<I>(&mut self, tokens: &mut Peekable<I>) -> Result<Box<Option<Node<'a>>>>
    where
        I: Iterator<Item = &'a Token>,
    {
        // Dummy Error
        File::open(&"dummy").context(format!("Could find implicator: {}", "Line 4"))?;

        // implementation goes here
        todo!();
    }

    fn get_operator<I>(&mut self, tokens: &mut Peekable<I>) -> Result<Box<Option<Node<'a>>>>
    where
        I: Iterator<Item = &'a Token>,
    {
        // Dummy Error
        File::open(&"dummy").context(format!("Could find operator: {}", "Line 4"))?;

        // implementation goes here
        todo!();
    }

    fn get_parenthesis<I>(&mut self, tokens: &mut Peekable<I>) -> Result<Box<Option<Node<'a>>>>
    where
        I: Iterator<Item = &'a Token>,
    {
        // Dummy Error
        File::open(&"dummy").context(format!("Could find parenthesis: {}", "Line 4"))?;

        // implementation goes here
        todo!();
    }

    fn get_identifier<I>(&mut self, tokens: &mut Peekable<I>) -> Result<Box<Option<Node<'a>>>>
    where
        I: Iterator<Item = &'a Token>,
    {
        // Dummy Error
        File::open(&"dummy").context(format!("Could find identifier: {}", "Line 4"))?;

        // implementation goes here
        todo!();
    }

    pub fn parse(&mut self) -> Result<()> {
        let tokens = self
            .tokenize()
            .context(format!("Could not parse {}", "line 4"))?;

        // implementation goes here
        todo!();
    }
}
