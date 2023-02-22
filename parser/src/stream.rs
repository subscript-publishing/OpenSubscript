use std::{fmt::{Debug, Display}, ops::{FromResidual, Try, Not}, convert::Infallible};
use either::*;

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone, Copy)]
pub struct Range {
    pub index: usize,
    pub length: usize,
}

impl Range {
    pub fn len(self) -> usize {
        self.length
    }
    pub fn valid_delta(self, delta: usize) -> bool {
        delta <= self.len()
    }
    pub fn seek_forward(self, delta: usize) -> Option<Range> {
        if !self.valid_delta(delta) {
            return None
        }
        Some(Self { index: self.index + delta, length: self.length - delta })
    }
    pub fn slice_range(self) -> std::ops::Range<usize> {
        self.index..self.index+self.length
    }
    // pub fn difference_between(self, other: Delta)
}
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone, Copy)]
pub struct Cursor {
    pub index: usize,
    pub column: usize,
    pub line: usize,
}

impl Cursor {
    pub fn forward_sync_for<T: AsRef<char>>(self, span: &[T]) -> Cursor {
        let index = self.index + span.len();
        let mut column = self.column;
        let mut line = self.line;
        for x in span.into_iter().map(|x| *x.as_ref()) {
            if x == '\n' {
                line = line + 1;
                column = 0;
                continue;
            }
            column = column + 1;
        }
        Self { index, column, line }
    }
}


//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone, Copy)]
pub struct Span<T> {
    pub source: T,
    pub range: Range,
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Indexed<A> {
    pub index: usize,
    pub value: A,
}

impl<T> Indexed<T> {
    pub fn is_equal_to(&self, x: T) -> bool where T: PartialEq {
        self.value == x
    }
    pub fn matches(&self, f: impl FnOnce(&T) -> bool) -> bool {
        f(&self.value)
    }
    pub fn does_not_match(&self, f: impl FnOnce(&T) -> bool) -> bool {
        !f(&self.value)
    }
}


impl From<Indexed<char>> for char {
    fn from(value: Indexed<char>) -> Self { value.value }
}
impl AsRef<char> for Indexed<char> {
    fn as_ref(&self) -> &char {
        &self.value
    }
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone, Copy)]
pub struct InContext<V, Ctx> {
    pub context: Ctx,
    pub value: V,
}

impl<V, Ctx> InContext<V, Ctx> {
    fn map<T>(self, f: impl FnOnce(V) -> T) -> InContext<T, Ctx> {
        InContext { value: f(self.value), context: self.context }
    }
    fn filter_map(self, f: impl FnOnce(&V) -> bool) -> Option<InContext<V, Ctx>> {
        if f(&self.value) {
            return InContext { value: self.value, context: self.context }.into()
        }
        None
    }
    fn map_context<T>(self, f: impl FnOnce(Ctx) -> T) -> InContext<V, T> {
        InContext { value: self.value, context: f(self.context) }
    }
}

pub type OutContext<V, Ctx> = InContext<Option<V>, Ctx>;

impl<V, Ctx> OutContext<V, Ctx> {
    pub fn expand_option(self) -> Option<InContext<V, Ctx>> {
        let value = self.value?;
        Some(InContext { value, context: self.context })
    }
}



//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone, Copy)]
pub struct Stream<'a> {
    pub source: &'a [Indexed<char>],
    pub cursor: Cursor,
}

impl<'a> Stream<'a> {
    pub fn is_empty(&'a self) -> bool { self.source.is_empty() }
    pub fn uncons(self) -> Option<InContext<Indexed<char>, Stream<'a>>> {
        if self.source.is_empty() {
            return None;
        }
        let item = self.source.first().unwrap().clone();
        let result = InContext{
            context: Stream{
                source: &self.source[1..],
                cursor: self.cursor.forward_sync_for(&[item]),
            },
            value: item,
        };
        Some(result)
    }
    pub fn take_char(self, char: char) -> Option<InContext<Indexed<char>, Stream<'a>>> {
        self.uncons()?.filter_map(|x| x.value == char).into()
    }
    pub fn take_while_true(
        self,
        f: impl Fn(char) -> bool
    ) -> Option<InContext<Stream<'a>, Stream<'a>>> {
        for (ix, char) in self.source.into_iter().enumerate() {
            if !f(char.value) {
                let value = &self.source[0..ix];
                if value.is_empty() {
                    return None
                }
                let result = InContext{
                    context: Stream{
                        source: &self.source[ix..],
                        cursor: self.cursor.forward_sync_for(value)
                    },
                    value: Stream{source: value, cursor: self.cursor},
                };
                return Some(result)
            }
        }
        let value = &self.source[0..];
        let result = InContext{
            context: Stream{
                source: &[],
                cursor: self.cursor.forward_sync_for(value),
            },
            value: Stream{source: value, cursor: self.cursor},
        };
        Some(result)
    }
    pub fn take_prefix(self, prefix: &str) -> Option<InContext<Stream<'a>, Stream<'a>>> {
        // if prefix.len() > self.source.len()
        let iter1 = self.source.into_iter();
        let iter2 = prefix.chars().into_iter().collect::<Vec<_>>();
        let iter2_len = iter2.len();
        if iter2.len() > iter1.len() {
            return None
        }
        for (l, r) in iter1.zip(iter2.into_iter()) {
            if l.value != r {
                return None
            }
        }
        let value = &self.source[0..iter2_len];
        let result = InContext{
            context: Stream{
                source: &self.source[iter2_len..],
                cursor: self.cursor.forward_sync_for(value),
            },
            value: Stream{
                source: value,
                cursor: self.cursor,
            },
        };
        Some(result)
    }
    pub fn as_token_view(self) -> TokenView<'a> {
        let range = Range{index: self.cursor.index, length: self.source.len()};
        TokenView { source: self.source, range }
    }
}


#[derive(Debug, Clone, Copy)]
pub struct CharView {
    pub span_index: usize,
    pub cursor: Cursor,
    pub char: Indexed<char>,
}

pub struct CharPredicate {
    pub predicate: fn(CharView) -> bool,
}

impl CharPredicate {
    pub fn is_valid(&self, view: CharView) -> bool { (self.predicate)(view) }
}

impl CharPredicate {
    const IS_WHITESPACE: CharPredicate = {
        fn predicate(view: CharView) -> bool { view.char.value == ' ' }
        CharPredicate { predicate }
    };
    const IS_NEWLINE: CharPredicate = {
        fn predicate(view: CharView) -> bool { view.char.value == ' ' }
        CharPredicate { predicate }
    };
    const IS_LETTER: CharPredicate = {
        fn predicate(view: CharView) -> bool { view.char.value.is_alphabetic() }
        CharPredicate { predicate }
    };
    const IS_NUMBER: CharPredicate = {
        fn predicate(view: CharView) -> bool { view.char.value.is_numeric() }
        CharPredicate { predicate }
    };
    const IS_LETTER_OR_NUMBER: CharPredicate = {
        fn predicate(view: CharView) -> bool { view.char.value.is_alphanumeric() }
        CharPredicate { predicate }
    };
    const IS_IDENTIFIER: CharPredicate = {
        fn predicate(view: CharView) -> bool {
            if view.span_index == 0 {
                CharPredicate::IS_NUMBER.is_valid(view)
            } else {
                CharPredicate::IS_LETTER_OR_NUMBER.is_valid(view)
            }
        }
        CharPredicate { predicate }
    };
    const IS_CONTENT_TEXT: CharPredicate = {
        fn predicate(view: CharView) -> bool {
            CharSymbol::is_symbol(&view.char.value).not()
        }
        CharPredicate { predicate }
    };
}

pub struct TakeWhileSpec {
    pub while_true: CharPredicate,
    pub ignoring: Option<CharPredicate>,
}

impl TakeWhileSpec {
    pub fn is_valid(&self, view: CharView) -> bool {
        if let Some(ignoring) = self.ignoring.as_ref() {
            self.while_true.is_valid(view) || ignoring.is_valid(view)
        } else {
            self.while_true.is_valid(view)
        }
    }
    const TAG_IDENTIFIER: TakeWhileSpec = {
        TakeWhileSpec {
            while_true: CharPredicate::IS_IDENTIFIER,
            ignoring: Some(CharPredicate::IS_WHITESPACE),
        }
    };
    const ARBITRARY_TEXT_CONTENT: TakeWhileSpec = {
        TakeWhileSpec { while_true: CharPredicate::IS_CONTENT_TEXT, ignoring: None}
    };
}

impl<'a> IO<'a> {
    pub fn apply_take_while_spec(self, spec: TakeWhileSpec) -> Option<InContext<Stream<'a>, Stream<'a>>> {
        for (ix, char) in self.source.into_iter().enumerate() {
            let view = CharView {
                span_index: ix,
                cursor: self.cursor,
                char: *char,
            };
            if !spec.is_valid(view) {
                let value = &self.source[0..ix];
                if value.is_empty() {
                    return None
                }
                let result = InContext{
                    context: Stream{
                        source: &self.source[ix..],
                        cursor: self.cursor.forward_sync_for(value)
                    },
                    value: Stream{source: value, cursor: self.cursor},
                };
                return Some(result)
            }
        }
        let value = &self.source[0..];
        let result = InContext{
            context: Stream{
                source: &[],
                cursor: self.cursor.forward_sync_for(value),
            },
            value: Stream{source: value, cursor: self.cursor},
        };
        Some(result)
    }
}

pub type DoOpt<'a, T> = Option<InContext<T, IO<'a>>>;

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Clone)]
pub struct TokenView<'a> {
    pub source: &'a [Indexed<char>],
    pub range: Range,
}

impl<'a> TokenView<'a> {
    pub fn to_string(&'a self) -> String {
        self.source.into_iter().map(|x| x.value).collect::<String>()
    }
}

impl<'a> Debug for TokenView<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = self.to_string();
        f.write_str(&string)
    }
}
impl<'a> Display for TokenView<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = self.to_string();
        f.write_str(&string)
    }
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone, Copy)]
pub struct CharSymbol {
    pub value: Indexed<char>
}

impl CharSymbol {
    pub fn is_symbol(value: &char) -> bool {
        match value {
            '\\' => true,
            '@' => true,
            '#' => true,
            '{' => true,
            '}' => true,
            '[' => true,
            ']' => true,
            '(' => true,
            ')' => true,
            '<' => true,
            '>' => true,
            '\n' => true,
            _ => false
        }
    }
    pub fn parse_from(value: Indexed<char>) -> Option<Self> {
        value.matches(CharSymbol::is_symbol).then(|| Self { value })
    }
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone)]
pub enum InEnclosure<'a> {
    /// `{` and '}'
    CurlyBrackets(InCurlyBrackets<'a>),
    /// (`[` and `]`
    SquareBrackets(InSquareBrackets<'a>),
    /// (`(` and `)`
    RoundBrackets(InRoundBrackets<'a>),
    /// (`<` and `>`
    AngleBrackets(InAngleBrackets<'a>),
}

#[derive(Debug, Clone)]
pub struct InCurlyBrackets<'a> {
    pub open: Indexed<char>,
    pub content: ParsedAst<'a>,
    pub close: Indexed<char>,
}
#[derive(Debug, Clone)]
pub struct InSquareBrackets<'a> {
    pub open: Indexed<char>,
    pub content: ParsedAst<'a>,
    pub close: Indexed<char>,
}
#[derive(Debug, Clone)]
pub struct InRoundBrackets<'a> {
    pub open: Indexed<char>,
    pub content: ParsedAst<'a>,
    pub close: Indexed<char>,
}
#[derive(Debug, Clone)]
pub struct InAngleBrackets<'a> {
    pub open: Indexed<char>,
    pub content: ParsedAst<'a>,
    pub close: Indexed<char>,
}

impl<'a> IO<'a> {
    pub fn parse_in_curly_brackets(self) -> DoOpt<'a, InCurlyBrackets<'a>> {
        let context = self;
        let InContext { value: open, context } = context.take_char('{')?;
        let InContext { value: content, context } = context.parse_ast()?;
        let InContext { value: close, context } = context.take_char('}')?;
        let value = InCurlyBrackets{open, content, close};
        Some(InContext{value, context})
    }
    pub fn parse_in_square_brackets(self) -> DoOpt<'a, InSquareBrackets<'a>> {
        let context = self;
        let InContext { value: open, context } = context.take_char('[')?;
        let InContext { value: content, context } = context.parse_ast()?;
        let InContext { value: close, context } = context.take_char(']')?;
        let value = InSquareBrackets{open, content, close};
        Some(InContext{value, context})
    }
    pub fn parse_in_round_brackets(self) -> DoOpt<'a, InRoundBrackets<'a>> {
        let context = self;
        let InContext { value: open, context } = context.take_char('(')?;
        let InContext { value: content, context } = context.parse_ast()?;
        let InContext { value: close, context } = context.take_char(')')?;
        let value = InRoundBrackets{open, content, close};
        Some(InContext{value, context})
    }
    pub fn parse_in_angle_brackets(self) -> DoOpt<'a, InAngleBrackets<'a>> {
        let context = self;
        let InContext { value: open, context } = context.take_char('<')?;
        let InContext { value: content, context } = context.parse_ast()?;
        let InContext { value: close, context } = context.take_char('>')?;
        let value = InAngleBrackets{open, content, close};
        Some(InContext{value, context})
    }
    pub fn parse_in_enclosure(self) -> DoOpt<'a, InEnclosure<'a>> {
        Option::<InContext<InEnclosure, IO<'a>>>::None
            .or_else(|| self.parse_in_curly_brackets().map(|x| x.map(|x| InEnclosure::CurlyBrackets(x))))
            .or_else(|| self.parse_in_square_brackets().map(|x| x.map(|x| InEnclosure::SquareBrackets(x))))
            .or_else(|| self.parse_in_round_brackets().map(|x| x.map(|x| InEnclosure::RoundBrackets(x))))
            .or_else(|| self.parse_in_angle_brackets().map(|x| x.map(|x| InEnclosure::AngleBrackets(x))))
    }
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone)]
pub struct Content<'a> {
    pub content: TokenView<'a>
}

impl<'a> IO<'a> {
    pub fn parse_content(self) -> Option<InContext<Content<'a>, IO<'a>>> {
        self
            .take_token_view(TakeWhileSpec::ARBITRARY_TEXT_CONTENT)?
            .map(|xs| Content{content: xs})
            .into()
    }
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone)]
pub struct IdentifierBlock<'a> {
    pub backslash: Indexed<char>,
    pub identifier: TokenView<'a>,
}

impl<'a> IO<'a> {
    pub fn parse_backslash(self) -> DoOpt<'a, Indexed<char>> {
        self.take_char('\\')
    }
    pub fn parse_identifier_block(self) -> DoOpt<'a, IdentifierBlock<'a>> {
        let context = self;
        let InContext { context, value: backslash } = context.parse_backslash()?;
        let InContext { context, value: identifier } = context.take_token_view(TakeWhileSpec::TAG_IDENTIFIER)?;
        let result = InContext{
            value: IdentifierBlock{backslash, identifier},
            context,
        };
        Some(result)
    }
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone)]
pub enum BeginTagHeader {
    Backslash {
        backslash_char: Indexed<char>,
    },
    Pipe {
        pipe_char: Indexed<char>,
    },
}

impl<'a> IO<'a> {
    pub fn parse_begin_tag_header(self) -> DoOpt<'a, BeginTagHeader> {
        let context = self;
        DoOpt::None
            .or_else(|| context.take_char('\\').map(|x| x.map(|x| BeginTagHeader::Backslash { backslash_char: x })))
            .or_else(|| context.take_char('|').map(|x| x.map(|x| BeginTagHeader::Pipe { pipe_char: x })))
    }
    pub fn parse_backslash_begin_tag_header(self) -> DoOpt<'a, BeginTagHeader> {
        let context = self;
        context
            .parse_backslash()
            .map(|x| x.map(|x| BeginTagHeader::Backslash { backslash_char: x }))
    }
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone)]
pub enum EndTagHeader<'a> {
    NoOP,
    Colon {
        colon_char: Indexed<char>,
        rest_of_line: ParsedAst<'a>,
    },
}

impl<'a> IO<'a> {
    pub fn parse_end_tag_header_colon(self) -> DoOpt<'a, EndTagHeader<'a>> {
        let context = self;
        let InContext { context, value: colon_char } = context.parse_colon()?;
        let InContext { context, value: rest_of_line } = context.parse_ast()?;
        let result = InContext{context, value: EndTagHeader::Colon {
            colon_char,
            rest_of_line,
        }};
        Some(result)
    }
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone)]
pub struct TagHeader<'a> {
    pub begin_type: BeginTagHeader,
    pub identifier: TokenView<'a>,
    pub attributes: Option<InSquareBrackets<'a>>,
    pub end_type: EndTagHeader<'a>,
}

impl<'a> IO<'a> {
    pub fn parse_tag_header(self) -> DoOpt<'a, TagHeader<'a>> {
        let context = self;
        let InContext { context, value: begin_type } = context.parse_backslash_begin_tag_header()?;
        let InContext { context, value: identifier } = context.take_token_view(TakeWhileSpec::TAG_IDENTIFIER)?;
        let (context, attributes) = {
            match context.parse_in_square_brackets() {
                Some(InContext { context, value }) => (context, Some(value)),
                None => (context, None),
            }
        };
        let (context, end_type) = {
            match context.parse_end_tag_header_colon() {
                Some(InContext { context, value }) => (context, value),
                None => (context, EndTagHeader::NoOP),
            }
        };
        let result = InContext{
            value: TagHeader {
                begin_type,
                identifier,
                attributes,
                end_type,
            },
            context,
        };
        Some(result)
    }
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone)]
pub struct IndentedList {
    
}

impl<'a> IO<'a> {
    pub fn next_line_info0(self) -> DoOpt<'a, ()> {
        if self.source.last().filter(|x| x.is_equal_to('\n')).is_some() {
            
        }
        let InContext { context, value } = self.take_while_true(|char| char != '\n')?;
        println!("context: {value:?}");
        Some(InContext { context, value: () })
    }
    pub fn next_line_info(self) -> DoOpt<'a, ()> {
        unimplemented!()
    }
    pub fn parse_indents(self) -> DoOpt<'a, Vec<ParsedAst<'a>>> {
        unimplemented!()
    }
}


//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone)]
pub enum TagBody<'a> {
    InCurlyBrackets(InCurlyBrackets<'a>),
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone)]
pub struct Tag<'a> {
    pub tag_header: TagHeader<'a>,
    // pub body: 
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone)]
pub struct TagBlock<'a> {
    pub identifier: IdentifierBlock<'a>,
    pub attributes: Option<InSquareBrackets<'a>>,
    pub body: Option<InCurlyBrackets<'a>>,
}

impl<'a> IO<'a> {
    pub fn parse_tag_block(self) -> DoOpt<'a, TagBlock<'a>> {
        let context = self;
        let InContext { context, value: identifier } = context.parse_identifier_block()?;
        let (context, attributes) = {
            match context.parse_in_square_brackets() {
                Some(InContext { context, value }) => (context, Some(value)),
                None => (context, None),
            }
        };
        let (context, body) = {
            match context.parse_in_curly_brackets() {
                Some(InContext { context, value }) => (context, Some(value)),
                None => (context, None),
            }
        };
        let result = InContext{
            value: TagBlock{
                identifier,
                attributes,
                body,
            },
            context,
        };
        Some(result)
    }
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone)]
pub struct TagLine<'a> {
    pub identifier: IdentifierBlock<'a>,
    pub attributes: Option<InSquareBrackets<'a>>,
    pub line: Option<InCurlyBrackets<'a>>,
}

impl<'a> IO<'a> {
    pub fn parse_pipe(self) -> DoOpt<'a, Indexed<char>> {
        self.take_char('|')
    }
    pub fn parse_colon(self) -> DoOpt<'a, Indexed<char>> {
        self.take_char(':')
    }
    pub fn parse_tag_line(self) -> DoOpt<'a, TagLine<'a>> {
        let context = self;
        let InContext { context, value: identifier } = context.parse_identifier_block()?;
        let (context, attributes) = {
            match context.parse_in_square_brackets() {
                Some(InContext { context, value }) => (context, Some(value)),
                None => (context, None),
            }
        };
        let InContext { context, value: _ } = context.parse_colon()?;
        let (context, body) = {
            match context.parse_in_curly_brackets() {
                Some(InContext { context, value }) => (context, Some(value)),
                None => (context, None),
            }
        };
        unimplemented!()
    }
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone)]
pub enum ParsedAst<'a> {
    Content(Content<'a>),
    Enclosure(Box<InEnclosure<'a>>),
    // TagBlock(Box<TagBlock<'a>>),
    TagHeader(Box<TagHeader<'a>>),
}

impl<'a> IO<'a> {
    pub fn parse_ast(self) -> DoOpt<'a, ParsedAst<'a>> {
        Option::<InContext<ParsedAst<'a>, IO<'a>>>::None
            .or_else(|| self.parse_tag_header().map(|x| x.map(|x| ParsedAst::TagHeader(Box::new(x)))))
            .or_else(|| self.parse_in_enclosure().map(|x| x.map(|x| ParsedAst::Enclosure(Box::new(x)))))
            .or_else(|| self.parse_content().map(|x| x.map(|x| ParsedAst::Content(x))))
    }
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
pub type IO<'a> = Stream<'a>;

impl<'a> IO<'a> {
    pub fn take_token_view(self, spec: TakeWhileSpec) -> DoOpt<'a, TokenView<'a>> {
        self
            .apply_take_while_spec(spec)?
            .map(|x| x.as_token_view())
            .map_context(|x| x)
            .into()
    }
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――

pub fn dev1() {
    let source_code_1 = include_str!("../samples/syntax2.txt");
    // let source_code_1 = "\\h1{X}";
    let source = source_code_1.chars().enumerate().map(|(ix, c)| Indexed{value: c, index: ix}).collect::<Vec<_>>();
    let source_stream = Stream{source: &source[..], cursor: Cursor { index: 0, column: 0, line: 0 }};
    let io = source_stream;
    let node = io.next_line_info().map(|InContext { context, value }| {
        println!("result: {value:#?}")
    });
    if node.is_none() {
        eprintln!("Error: Failed to parse input string");
    }

}