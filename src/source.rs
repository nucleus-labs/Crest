use pest::{Parser, RuleType, Token};

use std::cell::{Cell, OnceCell};
use std::sync::{Arc, Weak};

use crate::boo::Boo;

const NEWLINE_DEFINITIONS: [&str; 4] = ["\n", "\r\n", "\r", "\x0C"];
const WHITESPACE_DEFINITIONS: [&str; 6] = [" ", "\t", "\n", "\r\n", "\r", "\x0C"];

#[derive(Clone)]
pub struct SourceInfo {
    source: Arc<str>,
    newline_indices: Box<[usize]>,
    handle: OnceCell<Weak<Self>>,
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
    pub(crate) source_info: Arc<SourceInfo>,
    pub(crate) start: SourceLocation,
    pub(crate) end: SourceLocation,
}

#[derive(Debug, Clone)]
pub(crate) struct StackInfo<T> {
    pub source_info: Arc<SourceInfo>,
    pub rule: T,
    pub positions: (SourceLocation, Option<SourceLocation>),
    pub children: Vec<StackInfo<T>>,
}

#[derive(Debug, Clone)]
pub struct ParserToken<R: RuleType> {
    value: ParserTokenValue<Self>,
    rule: R,
    start: SourceLocation,
    end: SourceLocation,
}

#[derive(Debug, Clone)]
pub enum ParserTokenValue<T> {
    Leaf,
    Internal(Vec<T>),
}

#[derive(Debug, Clone, derive_more::From)]
pub enum ExpectError {
    #[from]
    SyntaxError(crate::syntax::CssExpectError),
    #[from]
    SelectorError(crate::selector::SelectorExpectError),

    #[from]
    Generic(String),
}

/// `TokenTracker`s are utilities for managing and parsing a sequence of `ParserToken`s.
///
/// This struct provides two primary features:
/// 1. A mechanism to "fake" token pops without mutating the underlying token vector, enabling
///    references to popped tokens to outlive the scope of the popping function.
/// 2. Convenient parsing functions (`expect_*`) to validate and extract specific token types,
///    returning descriptive errors (`ExpectError`) on failure.
///
/// Token popping is achieved by maintaining an internal offset index. Each pop operation
/// increments this index, effectively "consuming" tokens while preserving the underlying
/// vector's immutability. Once a token is "consumed," it cannot be revisited.
///
/// # Behavior
/// - Tokens are "consumed" in order, starting from the current offset.
/// - If an expectation fails, the offset remains unchanged.
/// - Tokens retain their ownership within `CssTokenTracker`, ensuring their lifetime matches
///   that of the tracker.
///
/// # Example Usage
/// For examples on usage, see `crate::selector::SelectorTokenTracker` or
/// `crate::syntax::CssTokenTracker`.
#[derive(Debug, Clone)]
pub struct TokenTracker<'a, R: RuleType> {
    /// A `Boo` wrapper around a `Vec<ParserToken<R>>`. Neither `Boo` nor `TokenTracker` guarantee
    /// that the contained vec is owned by a `TokenTracker`, but the design of `TokenTracker`
    /// assumes that any `boo` field adheres to this contract.
    ///
    /// The `boo` field enables the use of either owned or borrowed tokens, reducing
    /// redundancy in parsing implementations. It is essential that this field is only
    /// initialized within `TokenTracker` or similar contexts that maintain this contract.
    ///
    /// ## Notes:
    /// - Any modifications to the `boo` field should only occur within the
    ///   `TokenTracker` implementation to ensure the contract remains valid.
    /// - This abstraction avoids the need for separate types (e.g., `TokenTrackerBorrowed`)
    ///   by enabling flexible token ownership semantics.
    ///
    /// Misuse of the `boo` field outside its intended context can lead to undefined behavior
    /// in parsing operations. Use with caution and respect the ownership semantics.
    pub(crate) boo: crate::boo::Boo<'a, Vec<ParserToken<R>>>,

    pub(crate) idx: Cell<usize>,
}

// =============== IMPL ===============

/// A modified binary search algorithm that will find the exact or nearest
/// floored index of the given item.
///
/// Items outside the range of values in the array will return `None`.
/// Assumes the array is already sorted, otherwise the returned value is
/// meaningless.
///
/// ## Asserts
/// ```rust
/// fn floored_binary_index(arr: &[usize], item: usize) -> Option<usize> {
///     let length = arr.len();
/// 
///     if item < arr[0] || item > arr[length - 1] {
///         return None;
///     }
/// 
///     let mut left = 0;
///     let mut right = length - 1;
///     let mut middle = length / 2;
/// 
///     let mut current = arr[middle];
///     while left <= right {
///         match current.cmp(&item) {
///             std::cmp::Ordering::Less => left = middle + 1,
///             std::cmp::Ordering::Equal => return Some(middle),
///             std::cmp::Ordering::Greater => right = middle - 1,
///         }
///         middle = (right + left) / 2;
///         current = arr[middle];
///     }
/// 
///     Some(middle)
/// }
/// 
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
        arc.handle
            .set(Arc::downgrade(&arc))
            .expect("OnceCell should only be initialized once");
        arc
    }

    #[inline]
    pub fn get_handle(&self) -> Arc<Self> {
        self.handle.get().unwrap().upgrade().unwrap()
    }

    pub fn location_from_idx(&self, idx: usize) -> SourceLocation {
        let mut line = floored_binary_index(&self.newline_indices, idx);
        let line = line
            .map(|x| x + 1)
            .or_else(|| {
                if idx < self.newline_indices[0] {
                    Some(1)
                } else {
                    Some(self.newline_indices.len())
                }
            })
            .unwrap();

        if self.newline_indices[line - 1] > idx {
            panic!(
                "{} > {idx}\n{:?}",
                self.newline_indices[line - 1],
                self.newline_indices
            );
        }

        SourceLocation {
            source_info: self.get_handle(),
            idx,
            line,
            column: idx - self.newline_indices[line - 1] + 1,
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

impl std::fmt::Debug for SourceInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SourceInfo {{ ... }}")
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

impl<R: RuleType> StackInfo<R> {
    fn new(source_info: Arc<SourceInfo>, rule: R, pos: SourceLocation) -> Self {
        Self {
            source_info,
            rule,
            positions: (pos, None),
            children: Vec::new(),
        }
    }
}

impl<R: RuleType> ParserToken<R> {
    fn new(value: StackInfo<R>) -> Self {
        let token_val = if !value.children.is_empty() {
            let mut children: Vec<Self> = value
                .children
                .into_iter()
                .map(|x| Self::new(x))
                .collect::<Vec<_>>();
            ParserTokenValue::Internal(children)
        } else {
            ParserTokenValue::Leaf
        };

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

    pub fn get_rule(&self) -> R {
        self.rule
    }

    pub fn get_children(&self) -> Option<&Vec<ParserToken<R>>> {
        match &self.value {
            ParserTokenValue::Leaf => None,
            ParserTokenValue::Internal(vec) => Some(vec),
        }
    }
}

impl<'a, R: RuleType> TokenTracker<'a, R> {
    /// Creates a new `SelectorTokenTracker` from a `Boo` wrapping a vector of `SelectorToken`s.
    pub fn new(tokens_boo: Boo<'a, Vec<ParserToken<R>>>) -> Self {
        Self {
            boo: tokens_boo,
            idx: Cell::new(0),
        }
    }

    /// Retrieves a reference to the next token in the vector without consuming it.
    ///
    /// This method allows peeking at the current token for introspection, such as determining
    /// its type or length of child tokens, without advancing the offset index.
    #[inline]
    pub fn peek(&self) -> Option<&ParserToken<R>> {
        self.boo.get(self.idx.get())
    }

    #[inline]
    pub(crate) fn fail_because<O, E>(&self, error: E) -> Result<O, E> {
        self.idx.set(0.max(self.idx.get() - 1));
        Err(error)
    }

    /// Consumes the next token and returns a reference to it, or `None` if no tokens remain.
    ///
    /// This increments the internal offset index, marking the token as consumed.
    pub(crate) fn pop_front(&'a self) -> Option<&'a ParserToken<R>> {
        let result = self.boo.get(self.idx.get());
        if result.is_some() {
            self.idx.set(self.idx.get() + 1);
        }
        result
    }

    /// Consumes and returns references to the next `count` tokens, or `None` if fewer tokens
    /// remain than requested.
    ///
    /// This method advances the offset index by `count` if successful.
    pub(crate) fn pop_front_count(&'a self, count: usize) -> Option<Box<[&'a ParserToken<R>]>> {
        if self.boo.len() < self.idx.get() + count {
            return None;
        }

        let mut tokens = Vec::new();
        tokens.reserve(count);

        for i in 0..count {
            tokens.push(self.boo.get(self.idx.get() + i).unwrap());
        }

        self.idx.set(self.idx.get() + count);
        Some(tokens.into_boxed_slice())
    }

    /// Returns the number of unconsumed tokens remaining in the vector.
    #[inline]
    pub fn len(&self) -> usize {
        self.boo.len() - self.idx.get()
    }

    /// Returns true if there are still consumable tokens, else returns false
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.boo.is_empty() || self.len() == 0
    }
}

pub fn parse_source<R: RuleType, P: Parser<R>>(
    source: &str,
    rule: R,
) -> Result<ParserToken<R>, pest::error::Error<R>> {
    let source_info: Arc<SourceInfo> = SourceInfo::new(source.into());
    let pairs = P::parse(rule, source)?;

    let mut tokens = pairs.tokens();

    let mut stack: Vec<StackInfo<R>> = Vec::new();
    let mut root: StackInfo<R> = {
        match tokens.next().unwrap() {
            Token::Start { rule, pos } => {
                let pos = source_info.location_from_idx(pos.pos());
                StackInfo::new(source_info.clone(), rule.clone(), pos)
            }
            _ => unreachable!(),
        }
    };

    for token in tokens {
        match token {
            Token::Start { rule, pos } => {
                let pos = source_info.location_from_idx(pos.pos());
                stack.push(StackInfo::new(source_info.clone(), rule.clone(), pos));
            }
            Token::End { rule, pos } => {
                if let Some(mut css_token) = stack.pop() {
                    css_token.positions.1 = Some(source_info.location_from_idx(pos.pos()));

                    if let Some(parent) = stack.last_mut() {
                        parent.children.push(css_token);
                    } else {
                        root.children.push(css_token);
                    }
                } else {
                    root.positions.1 = Some(source_info.location_from_idx(pos.pos()));
                }
            }
        }
    }

    Ok(ParserToken::new(root))
}
