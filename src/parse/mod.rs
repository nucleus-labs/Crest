pub(crate) mod boo;
pub mod expectors;
pub(crate) mod pest_css;
pub(crate) mod pest_selector;

use bitmask_enum::bitmask;
use once_cell::sync::OnceCell;
use pest::{error::Error, Parser as _, Token};

use std::sync::Arc;

pub(crate) use pest_css::{CssParser, Rule as CssRule};
pub(crate) use pest_selector::{Rule as SelectorRule, SelectorParser};

pub use expectors::CssTokenTracker;

const NEWLINE_DEFINITIONS: [&str; 4] = ["\n", "\r\n", "\r", "\x0C"];
const WHITESPACE_DEFINITIONS: [&str; 6] = [" ", "\t", "\n", "\r\n", "\r", "\x0C"];

#[derive(Debug, Clone)]
pub struct SourceInfo {
    source: Arc<str>,
    newline_indices: Box<[usize]>,
    handle: OnceCell<Arc<Self>>,
}

#[derive(Debug, Clone)]
pub struct SourceLocation {
    source_info: Arc<SourceInfo>,
    pub idx: usize,
    /// starts at 1
    pub line: usize,
    /// starts at 1
    pub column: usize,
}

#[derive(Debug, Clone)]
pub struct SourceSlice {
    source_info: Arc<SourceInfo>,
    start: SourceLocation,
    end: SourceLocation,
}

#[derive(Debug, Clone)]
pub(crate) struct StackInfo<T> {
    pub source_info: Arc<SourceInfo>,
    pub rule: T,
    pub positions: (SourceLocation, Option<SourceLocation>),
    pub children: Vec<StackInfo<T>>,
}

pub(crate) type CssStackInfo = StackInfo<CssRule>;
pub(crate) type SelectorStackInfo = StackInfo<SelectorRule>;

#[bitmask]
pub enum TokenExpected {
    /// A [`<ident-token>`](https://drafts.csswg.org/css-syntax/#ident-token-diagram)
    Ident,

    /// A [`<hash-token>`](https://drafts.csswg.org/css-syntax/#hash-token-diagram) with the type flag set to "unrestricted"
    Hash,

    /// A [`<string-token>`](https://drafts.csswg.org/css-syntax/#string-token-diagram)
    QuotedString,

    /// A [`<url-token>`](https://drafts.csswg.org/css-syntax/#url-token-diagram)
    UnquotedUrl,

    /// A [`<number-token>`](https://drafts.csswg.org/css-syntax/#number-token-diagram)
    Number,

    /// A [`<percentage-token>`](https://drafts.csswg.org/css-syntax/#percentage-token-diagram)
    Percentage,

    /// A [`<dimension-token>`](https://drafts.csswg.org/css-syntax/#dimension-token-diagram)
    Dimension,

    /// A [`<function-token>`](https://drafts.csswg.org/css-syntax/#function-token-diagram)
    Function,
}

#[derive(Debug, Clone)]
pub enum ParserTokenValue<T> {
    Leaf,
    Internal(Vec<T>),
}

#[derive(Debug, Clone)]
pub struct CssToken {
    value: ParserTokenValue<Self>,
    rule: CssRule,
    start: SourceLocation,
    end: SourceLocation,
}

#[derive(Debug, Clone)]
pub struct SelectorToken {
    value: ParserTokenValue<Self>,
    rule: SelectorRule,
    start: SourceLocation,
    end: SourceLocation,
}

// ============== IMPL ==============

pub(crate) fn parse_css(source: &str) -> Result<CssToken, crate::error::Error> {
    let pairs_result = CssParser::parse(CssRule::CSS, source);
    if pairs_result.is_err() {
        return Err(pairs_result.unwrap_err().into());
    }

    let source_info: Arc<SourceInfo> = SourceInfo::new(source.into());
    let pairs = pairs_result.unwrap();
    let mut tokens = pairs.tokens();
    
    let mut stack: Vec<CssStackInfo> = Vec::new();
    let mut root: CssStackInfo = {
        let root_token = tokens.next().unwrap();
        match &root_token {
            Token::Start { rule, pos } => {
                let mut info = CssStackInfo::new(
                    source_info.clone(),
                    rule.clone(),
                    source_info.location_from_idx(pos.pos()),
                );
                info.positions.1 = Some(source_info.location_from_idx(source_info.source.len()));
                info
            },
            _ => panic!("Started with an end token?"),
        }
    };

    for token in tokens {
        match token {
            Token::Start { rule, pos } => {
                stack.push(CssStackInfo::new(
                    source_info.clone(),
                    rule.clone(),
                    source_info.location_from_idx(pos.pos()),
                ));
            }
            Token::End { rule, pos } => {
                if let Some(mut css_token) = stack.pop() {
                    css_token.positions.1 = Some(source_info.location_from_idx(pos.pos()));

                    if let Some(parent) = stack.last_mut() {
                        parent.children.push(css_token);
                    }
                }
            }
        }
    }

    Ok(CssToken::new(root))
}

pub fn parse_selector(source: &str) -> Result<SelectorToken, crate::error::Error> {
    let pairs_result = SelectorParser::parse(SelectorRule::SELECTOR, source);
    if pairs_result.is_err() {
        return Err(pairs_result.unwrap_err().into());
    }

    let source_info: Arc<SourceInfo> = SourceInfo::new(source.into());
    let pairs = pairs_result.unwrap();
    let mut stack: Vec<SelectorStackInfo> = Vec::new();
    let mut root: Option<SelectorStackInfo> = None;
    let mut has_mutated = false;

    for token in pairs.tokens() {
        match token {
            Token::Start { rule, pos } => {
                println!("{}{rule:?}", "    ".repeat(stack.len()));
                stack.push(SelectorStackInfo::new(
                    source_info.clone(),
                    rule.clone(),
                    source_info.location_from_idx(pos.pos()),
                ))
            }
            Token::End { rule, pos } => {
                if let Some(mut css_token) = stack.pop() {
                    css_token.positions.1 = Some(source_info.location_from_idx(pos.pos()));
                    let stack_size = stack.len();
                    if let Some(parent) = stack.last_mut() {
                        has_mutated = true;

                        let start = &css_token.positions.0;
                        let end = css_token.positions.1.as_ref().unwrap();
                        if start.line == end.line {
                            let token_slice = start.slice(&end);
                            println!("{}{rule:?}: '{}',", "    ".repeat(stack_size), token_slice);
                        } else {
                            println!("{}{rule:?},", "    ".repeat(stack_size));
                        }

                        parent.children.push(css_token);
                    } else if has_mutated {
                        root = Some(css_token);
                    }
                } else {
                    panic!();
                }
            }
        }
    }

    root.map(|x| SelectorToken::new(x))
        .ok_or(crate::error::Error::generic(
            "Unknown error occurred while processing token tree",
        ))
}

/// A modified binary search algorithm that will find the exact or nearest
/// floored index of the given item.
///
/// Items outside the range of values in
/// the array will return `None`. Assumes the array is already sorted,
/// otherwise the returned value is meaningless.
///
/// ## Asserts
/// ```rs
/// assert!(floored_binary_index(&[1, 3, 5, 9, 15], 0).is_none(), "Should fail because 0 is out of the range 0-15");
/// assert!(floored_binary_index(&[1, 3, 5, 9, 15], 16).is_none(), "Should fail because 16 is out of the range 0-15");
///
/// assert_eq!(floored_binary_index(&[1, 3, 5, 9, 15], 2).unwrap(), 0);
/// //                                  ^ would be here, so use previous index: 0
///
/// assert_eq!(floored_binary_index(&[1, 3, 5, 9, 15], 4).unwrap(), 1);
/// //                                     ^ would be here, so use previous index: 1
///
/// assert_eq!(floored_binary_index(&[1, 3, 5, 9, 15], 6).unwrap(), 2);
/// //                                        ^ would be here, so use previous index: 2
///
/// assert_eq!(floored_binary_index(&[1, 3, 5, 9, 15], 8).unwrap(), 2);
/// //                                        ^ would be here, so use previous index: 2
///
/// assert_eq!(floored_binary_index(&[1, 3, 5, 9, 15], 14).unwrap(), 3);
/// //   would be here, so use previous index: 3 ^
///
/// assert_eq!(floored_binary_index(&[1, 3, 5, 9, 15], 15).unwrap(), 4);
/// ```
fn floored_binary_index(arr: &[usize], item: usize) -> Option<usize> {
    let length = arr.len();

    if item < arr[0] || item > arr[length - 1] {
        return None;
    }

    let mut left = 0;
    let mut right = length - 1;
    let mut middle = length / 2;

    let mut current = arr[middle];
    while left <= right {
        match current.cmp(&item) {
            std::cmp::Ordering::Less => left = middle + 1,
            std::cmp::Ordering::Equal => return Some(middle),
            std::cmp::Ordering::Greater => right = middle - 1,
        }
        middle = (right + left) / 2;
        current = arr[middle];
    }

    Some(middle)
}

impl SourceInfo {
    pub fn new(source: Arc<str>) -> Arc<Self> {
        let mut newline_indices = vec![0];

        for i in 0..(source.len() - 1) {
            if NEWLINE_DEFINITIONS
                .iter()
                .any(|&x| source[i..i + 2].starts_with(x))
            {
                newline_indices.push(i);
            }
        }

        let new = Self {
            source: source.into(),
            newline_indices: newline_indices.into_boxed_slice(),
            handle: OnceCell::new(),
        };

        let arc: Arc<Self> = Arc::new(new);
        arc.handle.set(arc.clone()).unwrap();
        arc
    }

    pub fn get_handle(&self) -> Arc<Self> {
        self.handle.get().unwrap().clone()
    }

    pub fn location_from_idx(&self, idx: usize) -> SourceLocation {
        let mut line = floored_binary_index(&self.newline_indices, idx);
        line = line.or_else(|| {
            if idx < self.newline_indices[0] {
                Some(0)
            } else {
                Some(self.newline_indices.len() - 1)
            }
        });

        if self.newline_indices[line.unwrap()] > idx {
            panic!(
                "{} > {idx}\n{:?}",
                self.newline_indices[line.unwrap()],
                self.newline_indices
            );
        }

        SourceLocation {
            source_info: self.get_handle(),
            idx,
            line: line.unwrap() + 1,
            column: idx - self.newline_indices[line.unwrap()],
        }
    }
}

impl SourceLocation {
    pub fn slice(&self, other: &Self) -> SourceSlice {
        assert!(
            Arc::ptr_eq(&self.source_info, &other.source_info),
            "Cannot slice between 2 different sources!"
        );
        SourceSlice {
            source_info: self.source_info.clone(),
            start: self.clone(),
            end: other.clone(),
        }
    }
}

impl SourceSlice {
    #[inline]
    pub fn get(&self) -> &str {
        &self.source_info.source[self.start.idx..self.end.idx]
    }
}

impl std::fmt::Display for SourceSlice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get())
    }
}

impl std::ops::Deref for SourceSlice {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

impl std::cmp::PartialEq for SourceSlice {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.source_info, &other.source_info)
            && self.start.idx == other.start.idx
            && self.end.idx == other.end.idx
    }
}

impl std::cmp::Eq for SourceSlice {}

impl std::hash::Hash for SourceSlice {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        Arc::as_ptr(&self.source_info.source).hash(state);
        self.start.idx.hash(state);
        self.end.idx.hash(state);
    }
}

impl CssStackInfo {
    fn new(source_info: Arc<SourceInfo>, rule: CssRule, pos: SourceLocation) -> Self {
        Self {
            source_info,
            rule,
            positions: (pos, None),
            children: Vec::new(),
        }
    }
}

impl SelectorStackInfo {
    fn new(source_info: Arc<SourceInfo>, rule: SelectorRule, pos: SourceLocation) -> Self {
        Self {
            source_info,
            rule,
            positions: (pos, None),
            children: Vec::new(),
        }
    }
}

impl std::fmt::Display for CssRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl CssToken {
    fn new(value: CssStackInfo) -> Self {
        let token_val = if !value.children.is_empty() {
            let mut children: Vec<CssToken> = value
                .children
                .into_iter()
                .map(|x| CssToken::new(x))
                .collect::<Vec<_>>();
            ParserTokenValue::Internal(children)
        } else {
            ParserTokenValue::Leaf
        };

        assert!(
            matches!(value.rule, CssRule::CSS),
            "Root node of css token tree is '{:?}', not CSS",
            value.rule
        );

        Self {
            value: token_val,
            rule: value.rule,
            start: value.positions.0,
            end: value.positions.1.unwrap(),
        }
    }

    pub fn is_leaf(&self) -> bool {
        matches!(self.value, ParserTokenValue::Leaf)
    }

    pub fn get_source(&self) -> SourceSlice {
        self.start.slice(&self.end)
    }

    pub fn get_indices(&self) -> (SourceLocation, SourceLocation) {
        (self.start.clone(), self.end.clone())
    }

    pub fn get_rule(&self) -> CssRule {
        self.rule
    }

    pub fn get_children(&self) -> Option<&Vec<CssToken>> {
        match &self.value {
            ParserTokenValue::Leaf => None,
            ParserTokenValue::Internal(vec) => Some(vec),
        }
    }
}

impl SelectorToken {
    fn new(value: SelectorStackInfo) -> Self {
        let token_val = if value.children.len() > 0 {
            let mut children: Vec<SelectorToken> = Vec::new();
            for child in value.children.iter() {
                children.push(SelectorToken::new(child.clone()));
            }
            ParserTokenValue::Internal(children)
        } else {
            ParserTokenValue::Leaf
        };

        assert!(
            matches!(value.rule, SelectorRule::SELECTOR),
            "Root node of selector token tree is '{:?}', not SELECTOR",
            value.rule
        );

        Self {
            value: token_val,
            rule: value.rule,
            start: value.positions.0,
            end: value.positions.1.unwrap(),
        }
    }

    pub fn is_leaf(&self) -> bool {
        matches!(self.value, ParserTokenValue::Leaf)
    }

    pub fn get_source(&self) -> Option<SourceSlice> {
        match &self.value {
            ParserTokenValue::Leaf => Some(self.start.slice(&self.end)),
            ParserTokenValue::Internal(vec) => None,
        }
    }

    pub fn get_indices(&self) -> (SourceLocation, SourceLocation) {
        (self.start.clone(), self.end.clone())
    }

    pub fn get_rule(&self) -> SelectorRule {
        self.rule
    }

    pub fn get_children(&self) -> Option<&Vec<SelectorToken>> {
        match &self.value {
            ParserTokenValue::Leaf => None,
            ParserTokenValue::Internal(vec) => Some(vec),
        }
    }
}

impl std::fmt::Display for CssToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, " {:?}", self.get_rule());
        if self.is_leaf() {
            Ok(())
        } else {
            write!(f, " [");
            for item in self.get_children().unwrap().iter() {
                write!(f, " {item} ->");
            }
            write!(f, " ]");
            Ok(())
        }
    }
}

impl std::fmt::Display for SelectorToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.get_rule());
        if self.is_leaf() {
            Ok(())
        } else {
            for item in self.get_children().unwrap().iter() {
                write!(f, "{item}, ");
            }
            Ok(())
        }
    }
}
