//! `Calc` — typed CSS `calc()` expression.
//!
//! Supports the basic mathematical operators (`+`, `-`, `*`, `/`),
//! parentheses, and the common CSS value types that can participate in
//! calculations.

use std::fmt;

use super::{Angle, Frequency, Length, Number, Percentage, Resolution, Time};

/// A CSS mathematical function value.
///
/// Covers `calc()`, `min()`, `max()`, and `clamp()`.
#[derive(Debug, Clone, PartialEq)]
pub struct Calc {
    kind: CalcKind,
    exprs: Vec<CalcExpr>,
}

/// The kind of mathematical function.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CalcKind {
    /// `calc(<expression>)`.
    Calc,
    /// `min(<expression>, ...)`.
    Min,
    /// `max(<expression>, ...)`.
    Max,
    /// `clamp(<min>, <preferred>, <max>)`.
    Clamp,
}

impl Calc {
    /// Construct `calc(expr)`.
    #[allow(clippy::self_named_constructors)]
    pub fn calc(expr: CalcExpr) -> Self {
        Calc {
            kind: CalcKind::Calc,
            exprs: vec![expr],
        }
    }

    /// Construct `min(exprs...)`.
    pub fn min(exprs: Vec<CalcExpr>) -> Self {
        Calc {
            kind: CalcKind::Min,
            exprs,
        }
    }

    /// Construct `max(exprs...)`.
    pub fn max(exprs: Vec<CalcExpr>) -> Self {
        Calc {
            kind: CalcKind::Max,
            exprs,
        }
    }

    /// Construct `clamp(low, preferred, high)`.
    pub fn clamp(low: CalcExpr, preferred: CalcExpr, high: CalcExpr) -> Self {
        Calc {
            kind: CalcKind::Clamp,
            exprs: vec![low, preferred, high],
        }
    }

    /// Parse a single `calc()` expression argument.
    pub fn parse(s: &str) -> Option<Self> {
        Calc::parse_expr(s).map(Calc::calc)
    }

    /// Parse a single expression (used internally for `calc`, `min`, etc.).
    pub fn parse_expr(s: &str) -> Option<CalcExpr> {
        let mut lexer = Lexer::new(s);
        let mut parser = Parser::new(&mut lexer);
        let expr = parser.parse_expr()?;
        if !parser.at(Token::Eof) {
            return None;
        }
        Some(expr)
    }

    /// Parse a math function by name and raw argument string.
    pub fn parse_function(name: &str, args_str: &str) -> Option<Self> {
        let name = name.to_ascii_lowercase();
        let args_str = args_str.trim();
        if args_str.is_empty() {
            return None;
        }
        let parts: Vec<&str> = args_str.split(',').map(str::trim).collect();
        match name.as_str() {
            "calc" if parts.len() == 1 => Calc::parse(parts[0]),
            "min" if !parts.is_empty() => {
                let exprs: Vec<CalcExpr> = parts
                    .iter()
                    .map(|p| Calc::parse_expr(p))
                    .collect::<Option<Vec<_>>>()?;
                Some(Calc::min(exprs))
            }
            "max" if !parts.is_empty() => {
                let exprs: Vec<CalcExpr> = parts
                    .iter()
                    .map(|p| Calc::parse_expr(p))
                    .collect::<Option<Vec<_>>>()?;
                Some(Calc::max(exprs))
            }
            "clamp" if parts.len() == 3 => {
                let low = Calc::parse_expr(parts[0])?;
                let preferred = Calc::parse_expr(parts[1])?;
                let high = Calc::parse_expr(parts[2])?;
                Some(Calc::clamp(low, preferred, high))
            }
            _ => None,
        }
    }
}

impl fmt::Display for Calc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            CalcKind::Calc => write!(f, "calc({})", self.exprs[0]),
            CalcKind::Min => {
                write!(f, "min(")?;
                for (i, expr) in self.exprs.iter().enumerate() {
                    if i > 0 {
                        f.write_str(", ")?;
                    }
                    fmt::Display::fmt(expr, f)?;
                }
                f.write_str(")")
            }
            CalcKind::Max => {
                write!(f, "max(")?;
                for (i, expr) in self.exprs.iter().enumerate() {
                    if i > 0 {
                        f.write_str(", ")?;
                    }
                    fmt::Display::fmt(expr, f)?;
                }
                f.write_str(")")
            }
            CalcKind::Clamp => write!(
                f,
                "clamp({}, {}, {})",
                self.exprs[0], self.exprs[1], self.exprs[2]
            ),
        }
    }
}

/// An expression inside a `calc()`.
#[derive(Debug, Clone, PartialEq)]
pub enum CalcExpr {
    /// A literal value.
    Value(CalcValue),
    /// Addition.
    Add(Box<CalcExpr>, Box<CalcExpr>),
    /// Subtraction.
    Sub(Box<CalcExpr>, Box<CalcExpr>),
    /// Multiplication.
    Mul(Box<CalcExpr>, Box<CalcExpr>),
    /// Division.
    Div(Box<CalcExpr>, Box<CalcExpr>),
    /// Unary negation.
    Neg(Box<CalcExpr>),
}

impl CalcExpr {
    /// Construct `left + right`.
    pub fn add_expr(left: CalcExpr, right: CalcExpr) -> Self {
        CalcExpr::Add(Box::new(left), Box::new(right))
    }

    /// Construct `left - right`.
    pub fn sub_expr(left: CalcExpr, right: CalcExpr) -> Self {
        CalcExpr::Sub(Box::new(left), Box::new(right))
    }

    /// Construct `left * right`.
    pub fn mul_expr(left: CalcExpr, right: CalcExpr) -> Self {
        CalcExpr::Mul(Box::new(left), Box::new(right))
    }

    /// Construct `left / right`.
    pub fn div_expr(left: CalcExpr, right: CalcExpr) -> Self {
        CalcExpr::Div(Box::new(left), Box::new(right))
    }

    /// Construct `-expr`.
    pub fn neg_expr(expr: CalcExpr) -> Self {
        CalcExpr::Neg(Box::new(expr))
    }

    fn precedence(&self) -> u8 {
        match self {
            CalcExpr::Add(..) | CalcExpr::Sub(..) => 1,
            CalcExpr::Mul(..) | CalcExpr::Div(..) => 2,
            CalcExpr::Neg(..) => 3,
            CalcExpr::Value(..) => 4,
        }
    }
}

impl fmt::Display for CalcExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_with_prec(0, f)
    }
}

impl CalcExpr {
    fn fmt_with_prec(&self, parent_prec: u8, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let prec = self.precedence();
        let needs_parens = prec < parent_prec;
        if needs_parens {
            f.write_str("(")?;
        }
        match self {
            CalcExpr::Value(v) => fmt::Display::fmt(v, f)?,
            CalcExpr::Add(a, b) => {
                a.fmt_with_prec(prec, f)?;
                f.write_str(" + ")?;
                b.fmt_with_prec(prec, f)?;
            }
            CalcExpr::Sub(a, b) => {
                a.fmt_with_prec(prec, f)?;
                f.write_str(" - ")?;
                b.fmt_with_prec(prec, f)?;
            }
            CalcExpr::Mul(a, b) => {
                a.fmt_with_prec(prec, f)?;
                f.write_str(" * ")?;
                b.fmt_with_prec(prec, f)?;
            }
            CalcExpr::Div(a, b) => {
                a.fmt_with_prec(prec, f)?;
                f.write_str(" / ")?;
                b.fmt_with_prec(prec, f)?;
            }
            CalcExpr::Neg(a) => {
                f.write_str("-")?;
                a.fmt_with_prec(prec, f)?;
            }
        }
        if needs_parens {
            f.write_str(")")?;
        }
        Ok(())
    }
}

/// A literal value that can appear inside a `calc()`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CalcValue {
    /// A number.
    Number(Number),
    /// A percentage (unclamped so expressions like `150%` are preserved).
    Percentage(Percentage),
    /// A length.
    Length(Length),
    /// A time.
    Time(Time),
    /// An angle.
    Angle(Angle),
    /// A frequency.
    Frequency(Frequency),
    /// A resolution.
    Resolution(Resolution),
}

impl From<Number> for CalcValue {
    fn from(v: Number) -> Self {
        CalcValue::Number(v)
    }
}

impl From<Percentage> for CalcValue {
    fn from(v: Percentage) -> Self {
        CalcValue::Percentage(v)
    }
}

impl From<Length> for CalcValue {
    fn from(v: Length) -> Self {
        CalcValue::Length(v)
    }
}

impl From<Time> for CalcValue {
    fn from(v: Time) -> Self {
        CalcValue::Time(v)
    }
}

impl From<Angle> for CalcValue {
    fn from(v: Angle) -> Self {
        CalcValue::Angle(v)
    }
}

impl From<Frequency> for CalcValue {
    fn from(v: Frequency) -> Self {
        CalcValue::Frequency(v)
    }
}

impl From<Resolution> for CalcValue {
    fn from(v: Resolution) -> Self {
        CalcValue::Resolution(v)
    }
}

impl fmt::Display for CalcValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CalcValue::Number(v) => fmt::Display::fmt(v, f),
            CalcValue::Percentage(v) => fmt::Display::fmt(v, f),
            CalcValue::Length(v) => fmt::Display::fmt(v, f),
            CalcValue::Time(v) => fmt::Display::fmt(v, f),
            CalcValue::Angle(v) => fmt::Display::fmt(v, f),
            CalcValue::Frequency(v) => fmt::Display::fmt(v, f),
            CalcValue::Resolution(v) => fmt::Display::fmt(v, f),
        }
    }
}

// ── Lexer ───────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
enum Token {
    Value(CalcValue),
    Plus,
    Minus,
    Star,
    Slash,
    LParen,
    RParen,
    Eof,
    Invalid,
}

struct Lexer<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> Lexer<'a> {
    fn new(input: &'a str) -> Self {
        Lexer { input, pos: 0 }
    }

    fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        if self.pos >= self.input.len() {
            return Token::Eof;
        }
        let ch = self.peek_char().unwrap();
        match ch {
            '+' => {
                self.advance();
                Token::Plus
            }
            '-' => {
                self.advance();
                Token::Minus
            }
            '*' => {
                self.advance();
                Token::Star
            }
            '/' => {
                self.advance();
                Token::Slash
            }
            '(' => {
                self.advance();
                Token::LParen
            }
            ')' => {
                self.advance();
                Token::RParen
            }
            _ => {
                if ch.is_ascii_digit() || ch == '.' {
                    self.read_value()
                } else {
                    // Unknown token: mark the rest of the input invalid
                    // so the parser fails cleanly.
                    self.pos = self.input.len();
                    Token::Invalid
                }
            }
        }
    }

    fn read_value(&mut self) -> Token {
        let start = self.pos;
        // Number part: digits and at most one dot.
        let mut dot_seen = false;
        while self.pos < self.input.len() {
            let ch = self.peek_char().unwrap();
            if ch.is_ascii_digit() {
                self.advance();
            } else if ch == '.' && !dot_seen {
                dot_seen = true;
                self.advance();
            } else {
                break;
            }
        }
        let num_str = &self.input[start..self.pos];
        let value: f32 = match num_str.parse() {
            Ok(v) => v,
            Err(_) => return Token::Eof,
        };

        // Optional unit: `%` or a run of letters (including viewport/container
        // units with leading letters).
        let unit_start = self.pos;
        if self.peek_char() == Some('%') {
            self.advance();
        } else {
            while self.pos < self.input.len() {
                let ch = self.peek_char().unwrap();
                if ch.is_ascii_alphabetic() {
                    self.advance();
                } else {
                    break;
                }
            }
        }
        let unit = &self.input[unit_start..self.pos];
        match value_with_unit(value, unit) {
            Some(v) => Token::Value(v),
            None => Token::Eof,
        }
    }

    fn skip_whitespace(&mut self) {
        while self.pos < self.input.len() {
            let ch = self.peek_char().unwrap();
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn peek_char(&self) -> Option<char> {
        self.input[self.pos..].chars().next()
    }

    fn advance(&mut self) {
        if let Some(ch) = self.peek_char() {
            self.pos += ch.len_utf8();
        }
    }
}

fn value_with_unit(value: f32, unit: &str) -> Option<CalcValue> {
    if unit.is_empty() {
        return Some(CalcValue::Number(Number::from(value)));
    }
    let unit = unit.to_ascii_lowercase();
    match unit.as_str() {
        "%" => Some(CalcValue::Percentage(Percentage::from_raw(value))),
        "px" => Some(CalcValue::Length(Length::px(value))),
        "em" => Some(CalcValue::Length(Length::em(value))),
        "rem" => Some(CalcValue::Length(Length::rem(value))),
        "ex" => Some(CalcValue::Length(Length::ex(value))),
        "ch" => Some(CalcValue::Length(Length::ch(value))),
        "cap" => Some(CalcValue::Length(Length::cap(value))),
        "rcap" => Some(CalcValue::Length(Length::rcap(value))),
        "lh" => Some(CalcValue::Length(Length::lh(value))),
        "rlh" => Some(CalcValue::Length(Length::rlh(value))),
        "vw" => Some(CalcValue::Length(Length::vw(value))),
        "vh" => Some(CalcValue::Length(Length::vh(value))),
        "vmin" => Some(CalcValue::Length(Length::vmin(value))),
        "vmax" => Some(CalcValue::Length(Length::vmax(value))),
        "vi" => Some(CalcValue::Length(Length::vi(value))),
        "vb" => Some(CalcValue::Length(Length::vb(value))),
        "svw" => Some(CalcValue::Length(Length::svw(value))),
        "svh" => Some(CalcValue::Length(Length::svh(value))),
        "svmin" => Some(CalcValue::Length(Length::svmin(value))),
        "svmax" => Some(CalcValue::Length(Length::svmax(value))),
        "svi" => Some(CalcValue::Length(Length::svi(value))),
        "svb" => Some(CalcValue::Length(Length::svb(value))),
        "lvw" => Some(CalcValue::Length(Length::lvw(value))),
        "lvh" => Some(CalcValue::Length(Length::lvh(value))),
        "lvmin" => Some(CalcValue::Length(Length::lvmin(value))),
        "lvmax" => Some(CalcValue::Length(Length::lvmax(value))),
        "lvi" => Some(CalcValue::Length(Length::lvi(value))),
        "lvb" => Some(CalcValue::Length(Length::lvb(value))),
        "dvw" => Some(CalcValue::Length(Length::dvw(value))),
        "dvh" => Some(CalcValue::Length(Length::dvh(value))),
        "dvmin" => Some(CalcValue::Length(Length::dvmin(value))),
        "dvmax" => Some(CalcValue::Length(Length::dvmax(value))),
        "dvi" => Some(CalcValue::Length(Length::dvi(value))),
        "dvb" => Some(CalcValue::Length(Length::dvb(value))),
        "cqw" => Some(CalcValue::Length(Length::cqw(value))),
        "cqh" => Some(CalcValue::Length(Length::cqh(value))),
        "cqi" => Some(CalcValue::Length(Length::cqi(value))),
        "cqb" => Some(CalcValue::Length(Length::cqb(value))),
        "cqmin" => Some(CalcValue::Length(Length::cqmin(value))),
        "cqmax" => Some(CalcValue::Length(Length::cqmax(value))),
        "cm" => Some(CalcValue::Length(Length::cm(value))),
        "mm" => Some(CalcValue::Length(Length::mm(value))),
        "in" => Some(CalcValue::Length(Length::inches(value))),
        "pt" => Some(CalcValue::Length(Length::pt(value))),
        "pc" => Some(CalcValue::Length(Length::pc(value))),
        "q" => Some(CalcValue::Length(Length::q(value))),
        "fr" => Some(CalcValue::Length(Length::fr(value))),
        "s" => Some(CalcValue::Time(Time::s(value))),
        "ms" => Some(CalcValue::Time(Time::ms(value))),
        "deg" => Some(CalcValue::Angle(Angle::deg(value))),
        "rad" => Some(CalcValue::Angle(Angle::rad(value))),
        "grad" => Some(CalcValue::Angle(Angle::grad(value))),
        "turn" => Some(CalcValue::Angle(Angle::turn(value))),
        "hz" => Some(CalcValue::Frequency(Frequency::hz(value))),
        "khz" => Some(CalcValue::Frequency(Frequency::khz(value))),
        "dpi" => Some(CalcValue::Resolution(Resolution::dpi(value))),
        "dpcm" => Some(CalcValue::Resolution(Resolution::dpcm(value))),
        "dppx" => Some(CalcValue::Resolution(Resolution::dppx(value))),
        "x" => Some(CalcValue::Resolution(Resolution::x(value))),
        _ => None,
    }
}

// ── Parser ──────────────────────────────────────────────────────────

struct Parser<'a, 'b> {
    lexer: &'a mut Lexer<'b>,
    current: Token,
}

impl<'a, 'b> Parser<'a, 'b> {
    fn new(lexer: &'a mut Lexer<'b>) -> Self {
        let current = lexer.next_token();
        Parser { lexer, current }
    }

    fn parse_expr(&mut self) -> Option<CalcExpr> {
        self.parse_add_sub()
    }

    fn parse_add_sub(&mut self) -> Option<CalcExpr> {
        let mut left = self.parse_mul_div()?;
        loop {
            if self.eat(Token::Plus) {
                let right = self.parse_mul_div()?;
                left = CalcExpr::add_expr(left, right);
            } else if self.eat(Token::Minus) {
                let right = self.parse_mul_div()?;
                left = CalcExpr::sub_expr(left, right);
            } else {
                break;
            }
        }
        Some(left)
    }

    fn parse_mul_div(&mut self) -> Option<CalcExpr> {
        let mut left = self.parse_unary()?;
        loop {
            if self.eat(Token::Star) {
                let right = self.parse_unary()?;
                left = CalcExpr::mul_expr(left, right);
            } else if self.eat(Token::Slash) {
                let right = self.parse_unary()?;
                left = CalcExpr::div_expr(left, right);
            } else {
                break;
            }
        }
        Some(left)
    }

    fn parse_unary(&mut self) -> Option<CalcExpr> {
        if self.eat(Token::Plus) {
            return self.parse_unary();
        }
        if self.eat(Token::Minus) {
            return Some(CalcExpr::neg_expr(self.parse_unary()?));
        }
        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Option<CalcExpr> {
        match self.current {
            Token::Value(v) => {
                self.advance();
                Some(CalcExpr::Value(v))
            }
            Token::LParen => {
                self.advance();
                let expr = self.parse_expr()?;
                self.expect(Token::RParen)?;
                Some(expr)
            }
            _ => None,
        }
    }

    fn eat(&mut self, token: Token) -> bool {
        if self.current == token {
            self.advance();
            true
        } else {
            false
        }
    }

    fn expect(&mut self, token: Token) -> Option<()> {
        if self.current == token {
            self.advance();
            Some(())
        } else {
            None
        }
    }

    fn at(&self, token: Token) -> bool {
        self.current == token
    }

    fn advance(&mut self) {
        self.current = self.lexer.next_token();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_simple_addition() {
        let calc = Calc::calc(CalcExpr::add_expr(
            CalcExpr::Value(Length::px(10.0).into()),
            CalcExpr::Value(Length::px(20.0).into()),
        ));
        assert_eq!(calc.to_string(), "calc(10px + 20px)");
    }

    #[test]
    fn display_operator_precedence() {
        let calc = Calc::calc(CalcExpr::sub_expr(
            CalcExpr::Value(Percentage::from_raw(100.0).into()),
            CalcExpr::div_expr(
                CalcExpr::Value(Length::px(20.0).into()),
                CalcExpr::Value(Number::from(2.0).into()),
            ),
        ));
        assert_eq!(calc.to_string(), "calc(100% - 20px / 2)");
    }

    #[test]
    fn display_parentheses() {
        let calc = Calc::calc(CalcExpr::div_expr(
            CalcExpr::sub_expr(
                CalcExpr::Value(Percentage::from_raw(100.0).into()),
                CalcExpr::Value(Length::px(20.0).into()),
            ),
            CalcExpr::Value(Number::from(2.0).into()),
        ));
        assert_eq!(calc.to_string(), "calc((100% - 20px) / 2)");
    }

    #[test]
    fn display_unary_minus() {
        let calc = Calc::calc(CalcExpr::sub_expr(
            CalcExpr::Value(Percentage::from_raw(100.0).into()),
            CalcExpr::neg_expr(CalcExpr::Value(Length::px(20.0).into())),
        ));
        assert_eq!(calc.to_string(), "calc(100% - -20px)");
    }

    #[test]
    fn display_min_max_clamp() {
        assert_eq!(
            Calc::min(vec![
                CalcExpr::Value(Length::px(100.0).into()),
                CalcExpr::Value(Percentage::from_raw(50.0).into()),
            ])
            .to_string(),
            "min(100px, 50%)"
        );
        assert_eq!(
            Calc::max(vec![
                CalcExpr::Value(Length::px(100.0).into()),
                CalcExpr::Value(Percentage::from_raw(50.0).into()),
            ])
            .to_string(),
            "max(100px, 50%)"
        );
        assert_eq!(
            Calc::clamp(
                CalcExpr::Value(Length::px(10.0).into()),
                CalcExpr::Value(Percentage::from_raw(50.0).into()),
                CalcExpr::Value(Length::px(100.0).into()),
            )
            .to_string(),
            "clamp(10px, 50%, 100px)"
        );
    }

    #[test]
    fn parse_simple_values() {
        let cases: [(&str, &str); 5] = [
            ("100%", "calc(100%)"),
            ("20px", "calc(20px)"),
            ("0", "calc(0)"),
            ("1.5s", "calc(1.5s)"),
            ("90deg", "calc(90deg)"),
        ];
        for (input, expected) in cases {
            let calc = Calc::parse(input).expect(input);
            assert_eq!(calc.to_string(), expected, "{input}");
        }
    }

    #[test]
    fn parse_expressions() {
        let cases: [(&str, &str); 6] = [
            ("100% - 20px", "calc(100% - 20px)"),
            ("100% - 20px / 2", "calc(100% - 20px / 2)"),
            ("(100% - 20px) / 2", "calc((100% - 20px) / 2)"),
            ("2 * (100% - 20px)", "calc(2 * (100% - 20px))"),
            ("100% + -20px", "calc(100% + -20px)"),
            ("100%-20px", "calc(100% - 20px)"),
        ];
        for (input, expected) in cases {
            let calc = Calc::parse(input).expect(input);
            assert_eq!(calc.to_string(), expected, "{input}");
        }
    }

    #[test]
    fn parse_math_functions() {
        let cases: [(&str, &str); 6] = [
            ("calc(100% - 20px)", "calc(100% - 20px)"),
            ("min(100% - 20px, 50%)", "min(100% - 20px, 50%)"),
            ("max(100px, 50%)", "max(100px, 50%)"),
            ("clamp(10px, 50%, 100px)", "clamp(10px, 50%, 100px)"),
            (
                "min((100% - 20px) / 2, 100px)",
                "min((100% - 20px) / 2, 100px)",
            ),
            (
                "clamp(1rem, 2vw + 1rem, 3rem)",
                "clamp(1rem, 2vw + 1rem, 3rem)",
            ),
        ];
        for (input, expected) in cases {
            let open = input.find('(').expect(input);
            let close = input.rfind(')').expect(input);
            let name = &input[..open];
            let args = &input[open + 1..close];
            let calc = Calc::parse_function(name, args).expect(input);
            assert_eq!(calc.to_string(), expected, "{input}");
        }
    }

    #[test]
    fn parse_rejects_invalid_input() {
        assert!(Calc::parse("").is_none());
        assert!(Calc::parse("(").is_none());
        assert!(Calc::parse("100% -").is_none());
        assert!(Calc::parse("100% unknown").is_none());
        assert!(Calc::parse_function("min", "").is_none());
        assert!(Calc::parse_function("clamp", "10px, 50%").is_none());
        assert!(Calc::parse_function("unknown", "10px").is_none());
    }
}
