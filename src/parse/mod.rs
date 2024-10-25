
use pest::{Position, Token};
use pest_derive::Parser;

use std::ops::{Deref, DerefMut};
use std::collections::HashMap;

use crate::css::CssToken;

#[derive(Parser)]
#[grammar = "css.pest"]
pub struct CssParser;

#[derive(Clone)]
pub struct StackInfo<'a> {
    pub rule: Rule, 
    pub positions: (Position<'a>, Option<Position<'a>>),
    pub children: Vec<StackInfo<'a>>
}

pub fn gen_token(pairs: pest::iterators::Pairs<Rule>) -> Option<CssToken> {
    let mut stack: Vec<StackInfo> = Vec::new().into();
    let mut root: Option<StackInfo> = None;

    for token in pairs.tokens() {
        match token {
            Token::Start { rule, pos } => stack.push(StackInfo::new(rule.clone(), pos.clone())),
            Token::End { rule, pos } => {
                if let Some(mut css_token) = stack.pop() {
                    css_token.positions.1 = Some(pos);
                    if let Some(parent) = stack.last_mut() {
                        parent.children.push(css_token);
                    }
                    else {
                        root = Some(css_token);
                    }
                }
            }
        }
    }

    match root {
        Some(token) => Some(token.into()),
        None => None,
    }
}

impl<'a> StackInfo<'a> {
    fn new(rule: Rule, pos: Position<'a>) -> Self {
        Self{
            rule,
            positions: (pos, None),
            children: Vec::new()
        }
    }
}

impl std::fmt::Display for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
