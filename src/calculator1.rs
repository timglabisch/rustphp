#![allow(unused_imports)]
#![allow(unused_variables)]
use std::str::FromStr;
use ast::{If, Expression, Block, Variable};
extern crate lalrpop_util as __lalrpop_util;
use self::__lalrpop_util::ParseError as __ParseError;

mod __parse__Start {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use std::str::FromStr;
    use ast::{If, Expression, Block, Variable};
    extern crate lalrpop_util as __lalrpop_util;
    use self::__lalrpop_util::ParseError as __ParseError;
    pub fn parse_Start<
        'input,
    >(
        input: &'input str,
    ) -> Result<Box<If>, __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match try!(__state0(input, None, &mut __tokens, __lookahead)) {
            (_, Some(__lookahead), _) => {
                Err(__ParseError::ExtraToken { token: __lookahead })
            }
            (_, None, __Nonterminal::____Start(__nt)) => {
                Ok(__nt)
            }
            _ => unreachable!(),
        }
    }

    #[allow(dead_code)]
    pub enum __Nonterminal<'input> {
        Block(Box<Block>),
        CompareAble(Box<String>),
        Expression(Box<Expression>),
        ExpressionMath(Box<Expression>),
        If(Box<If>),
        Operator(&'input str),
        OperatorComparison(&'input str),
        OperatorMath(&'input str),
        Start(Box<If>),
        T__IF(&'input str),
        T__NUMBER(Box<String>),
        T__VARIABLE(Box<String>),
        ____Start(Box<If>),
        ____simple__variable(Box<Variable>),
        simple__variable(Box<Variable>),
    }

    // State 0
    //   If = (*) T_IF "(" Expression ")" Block [EOF]
    //   Start = (*) If [EOF]
    //   T_IF = (*) "if" ["("]
    //   __Start = (*) Start [EOF]
    //
    //   "if" -> Shift(S4)
    //
    //   If -> S1
    //   Start -> S2
    //   T_IF -> S3
    pub fn __state0<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (14, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym0 = &mut Some((__tok0));
                __result = try!(__state4(input, __lookbehind, __tokens, __sym0));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        loop {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::If(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state1(input, __lookbehind, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::Start(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state2(input, __lookbehind, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::T__IF(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state3(input, __lookbehind, __tokens, __lookahead, __sym0));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
    }

    // State 1
    //   Start = If (*) [EOF]
    //
    //   EOF -> Reduce(Start = If => Call(ActionFn(2));)
    //
    pub fn __state1<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Box<If>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action2(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Start(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 2
    //   __Start = Start (*) [EOF]
    //
    //   EOF -> Reduce(__Start = Start => Call(ActionFn(0));)
    //
    pub fn __state2<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Box<If>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action0(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::____Start(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 3
    //   If = T_IF (*) "(" Expression ")" Block [EOF]
    //
    //   "(" -> Shift(S5)
    //
    pub fn __state3<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (1, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state5(input, __lookbehind, __tokens, __sym0, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 4
    //   T_IF = "if" (*) ["("]
    //
    //   "(" -> Reduce(T_IF = "if" => Call(ActionFn(3));)
    //
    pub fn __state4<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (1, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action3(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::T__IF(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 5
    //   CompareAble = (*) T_NUMBER [")"]
    //   CompareAble = (*) T_NUMBER ["*"]
    //   CompareAble = (*) T_NUMBER ["+"]
    //   CompareAble = (*) T_NUMBER ["-"]
    //   CompareAble = (*) T_NUMBER ["<"]
    //   CompareAble = (*) T_NUMBER ["=<"]
    //   CompareAble = (*) T_NUMBER ["=="]
    //   CompareAble = (*) T_NUMBER ["==="]
    //   CompareAble = (*) T_NUMBER ["=>"]
    //   CompareAble = (*) T_NUMBER [">"]
    //   CompareAble = (*) T_NUMBER ["\\\\"]
    //   CompareAble = (*) T_VARIABLE [")"]
    //   CompareAble = (*) T_VARIABLE ["*"]
    //   CompareAble = (*) T_VARIABLE ["+"]
    //   CompareAble = (*) T_VARIABLE ["-"]
    //   CompareAble = (*) T_VARIABLE ["<"]
    //   CompareAble = (*) T_VARIABLE ["=<"]
    //   CompareAble = (*) T_VARIABLE ["=="]
    //   CompareAble = (*) T_VARIABLE ["==="]
    //   CompareAble = (*) T_VARIABLE ["=>"]
    //   CompareAble = (*) T_VARIABLE [">"]
    //   CompareAble = (*) T_VARIABLE ["\\\\"]
    //   Expression = (*) CompareAble [")"]
    //   Expression = (*) ExpressionMath [")"]
    //   ExpressionMath = (*) CompareAble Operator Expression [")"]
    //   If = T_IF "(" (*) Expression ")" Block [EOF]
    //   T_NUMBER = (*) r#"([0-9]+)"# [")"]
    //   T_NUMBER = (*) r#"([0-9]+)"# ["*"]
    //   T_NUMBER = (*) r#"([0-9]+)"# ["+"]
    //   T_NUMBER = (*) r#"([0-9]+)"# ["-"]
    //   T_NUMBER = (*) r#"([0-9]+)"# ["<"]
    //   T_NUMBER = (*) r#"([0-9]+)"# ["=<"]
    //   T_NUMBER = (*) r#"([0-9]+)"# ["=="]
    //   T_NUMBER = (*) r#"([0-9]+)"# ["==="]
    //   T_NUMBER = (*) r#"([0-9]+)"# ["=>"]
    //   T_NUMBER = (*) r#"([0-9]+)"# [">"]
    //   T_NUMBER = (*) r#"([0-9]+)"# ["\\\\"]
    //   T_NUMBER = (*) r#"([0-9]+)\\.([0-9]+)"# [")"]
    //   T_NUMBER = (*) r#"([0-9]+)\\.([0-9]+)"# ["*"]
    //   T_NUMBER = (*) r#"([0-9]+)\\.([0-9]+)"# ["+"]
    //   T_NUMBER = (*) r#"([0-9]+)\\.([0-9]+)"# ["-"]
    //   T_NUMBER = (*) r#"([0-9]+)\\.([0-9]+)"# ["<"]
    //   T_NUMBER = (*) r#"([0-9]+)\\.([0-9]+)"# ["=<"]
    //   T_NUMBER = (*) r#"([0-9]+)\\.([0-9]+)"# ["=="]
    //   T_NUMBER = (*) r#"([0-9]+)\\.([0-9]+)"# ["==="]
    //   T_NUMBER = (*) r#"([0-9]+)\\.([0-9]+)"# ["=>"]
    //   T_NUMBER = (*) r#"([0-9]+)\\.([0-9]+)"# [">"]
    //   T_NUMBER = (*) r#"([0-9]+)\\.([0-9]+)"# ["\\\\"]
    //   T_VARIABLE = (*) r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# [")"]
    //   T_VARIABLE = (*) r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# ["*"]
    //   T_VARIABLE = (*) r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# ["+"]
    //   T_VARIABLE = (*) r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# ["-"]
    //   T_VARIABLE = (*) r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# ["<"]
    //   T_VARIABLE = (*) r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# ["=<"]
    //   T_VARIABLE = (*) r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# ["=="]
    //   T_VARIABLE = (*) r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# ["==="]
    //   T_VARIABLE = (*) r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# ["=>"]
    //   T_VARIABLE = (*) r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# [">"]
    //   T_VARIABLE = (*) r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# ["\\\\"]
    //
    //   r#"([0-9]+)"# -> Shift(S11)
    //   r#"([0-9]+)\\.([0-9]+)"# -> Shift(S12)
    //   r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# -> Shift(S13)
    //
    //   CompareAble -> S6
    //   Expression -> S7
    //   ExpressionMath -> S8
    //   T_NUMBER -> S9
    //   T_VARIABLE -> S10
    pub fn __state5<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (18, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state11(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (19, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state12(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (20, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state13(input, __lookbehind, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::CompareAble(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state6(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Expression(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state7(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::ExpressionMath(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state8(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::T__NUMBER(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state9(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::T__VARIABLE(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state10(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 6
    //   Expression = CompareAble (*) [")"]
    //   ExpressionMath = CompareAble (*) Operator Expression [")"]
    //   Operator = (*) OperatorComparison [r#"([0-9]+)"#]
    //   Operator = (*) OperatorComparison [r#"([0-9]+)\\.([0-9]+)"#]
    //   Operator = (*) OperatorComparison [r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"#]
    //   Operator = (*) OperatorMath [r#"([0-9]+)"#]
    //   Operator = (*) OperatorMath [r#"([0-9]+)\\.([0-9]+)"#]
    //   Operator = (*) OperatorMath [r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"#]
    //   OperatorComparison = (*) "<" [r#"([0-9]+)"#]
    //   OperatorComparison = (*) "<" [r#"([0-9]+)\\.([0-9]+)"#]
    //   OperatorComparison = (*) "<" [r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"#]
    //   OperatorComparison = (*) "=<" [r#"([0-9]+)"#]
    //   OperatorComparison = (*) "=<" [r#"([0-9]+)\\.([0-9]+)"#]
    //   OperatorComparison = (*) "=<" [r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"#]
    //   OperatorComparison = (*) "==" [r#"([0-9]+)"#]
    //   OperatorComparison = (*) "==" [r#"([0-9]+)\\.([0-9]+)"#]
    //   OperatorComparison = (*) "==" [r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"#]
    //   OperatorComparison = (*) "===" [r#"([0-9]+)"#]
    //   OperatorComparison = (*) "===" [r#"([0-9]+)\\.([0-9]+)"#]
    //   OperatorComparison = (*) "===" [r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"#]
    //   OperatorComparison = (*) "=>" [r#"([0-9]+)"#]
    //   OperatorComparison = (*) "=>" [r#"([0-9]+)\\.([0-9]+)"#]
    //   OperatorComparison = (*) "=>" [r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"#]
    //   OperatorComparison = (*) ">" [r#"([0-9]+)"#]
    //   OperatorComparison = (*) ">" [r#"([0-9]+)\\.([0-9]+)"#]
    //   OperatorComparison = (*) ">" [r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"#]
    //   OperatorMath = (*) "*" [r#"([0-9]+)"#]
    //   OperatorMath = (*) "*" [r#"([0-9]+)\\.([0-9]+)"#]
    //   OperatorMath = (*) "*" [r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"#]
    //   OperatorMath = (*) "+" [r#"([0-9]+)"#]
    //   OperatorMath = (*) "+" [r#"([0-9]+)\\.([0-9]+)"#]
    //   OperatorMath = (*) "+" [r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"#]
    //   OperatorMath = (*) "-" [r#"([0-9]+)"#]
    //   OperatorMath = (*) "-" [r#"([0-9]+)\\.([0-9]+)"#]
    //   OperatorMath = (*) "-" [r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"#]
    //   OperatorMath = (*) "\\\\" [r#"([0-9]+)"#]
    //   OperatorMath = (*) "\\\\" [r#"([0-9]+)\\.([0-9]+)"#]
    //   OperatorMath = (*) "\\\\" [r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"#]
    //
    //   ")" -> Reduce(Expression = CompareAble => Call(ActionFn(16));)
    //   "*" -> Shift(S17)
    //   "+" -> Shift(S18)
    //   "-" -> Shift(S19)
    //   "<" -> Shift(S20)
    //   "=<" -> Shift(S21)
    //   "==" -> Shift(S22)
    //   "===" -> Shift(S23)
    //   "=>" -> Shift(S24)
    //   ">" -> Shift(S25)
    //   "\\\\" -> Shift(S26)
    //
    //   Operator -> S14
    //   OperatorComparison -> S15
    //   OperatorMath -> S16
    pub fn __state6<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Box<String>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (3, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state17(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (4, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state18(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (5, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state19(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (7, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state20(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (8, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state21(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (9, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state22(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (10, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state23(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (11, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state24(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (12, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state25(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (13, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state26(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (2, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action16(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Expression(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Operator(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state14(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                __Nonterminal::OperatorComparison(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state15(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::OperatorMath(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state16(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 7
    //   If = T_IF "(" Expression (*) ")" Block [EOF]
    //
    //   ")" -> Shift(S27)
    //
    pub fn __state7<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<Box<Expression>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (2, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state27(input, __lookbehind, __tokens, __sym0, __sym1, __sym2, __sym3));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 8
    //   Expression = ExpressionMath (*) [")"]
    //
    //   ")" -> Reduce(Expression = ExpressionMath => Call(ActionFn(15));)
    //
    pub fn __state8<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Box<Expression>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (2, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action15(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Expression(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 9
    //   CompareAble = T_NUMBER (*) [")"]
    //   CompareAble = T_NUMBER (*) ["*"]
    //   CompareAble = T_NUMBER (*) ["+"]
    //   CompareAble = T_NUMBER (*) ["-"]
    //   CompareAble = T_NUMBER (*) ["<"]
    //   CompareAble = T_NUMBER (*) ["=<"]
    //   CompareAble = T_NUMBER (*) ["=="]
    //   CompareAble = T_NUMBER (*) ["==="]
    //   CompareAble = T_NUMBER (*) ["=>"]
    //   CompareAble = T_NUMBER (*) [">"]
    //   CompareAble = T_NUMBER (*) ["\\\\"]
    //
    //   ")" -> Reduce(CompareAble = T_NUMBER => Call(ActionFn(12));)
    //   "*" -> Reduce(CompareAble = T_NUMBER => Call(ActionFn(12));)
    //   "+" -> Reduce(CompareAble = T_NUMBER => Call(ActionFn(12));)
    //   "-" -> Reduce(CompareAble = T_NUMBER => Call(ActionFn(12));)
    //   "<" -> Reduce(CompareAble = T_NUMBER => Call(ActionFn(12));)
    //   "=<" -> Reduce(CompareAble = T_NUMBER => Call(ActionFn(12));)
    //   "==" -> Reduce(CompareAble = T_NUMBER => Call(ActionFn(12));)
    //   "===" -> Reduce(CompareAble = T_NUMBER => Call(ActionFn(12));)
    //   "=>" -> Reduce(CompareAble = T_NUMBER => Call(ActionFn(12));)
    //   ">" -> Reduce(CompareAble = T_NUMBER => Call(ActionFn(12));)
    //   "\\\\" -> Reduce(CompareAble = T_NUMBER => Call(ActionFn(12));)
    //
    pub fn __state9<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Box<String>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (11, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action12(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::CompareAble(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 10
    //   CompareAble = T_VARIABLE (*) [")"]
    //   CompareAble = T_VARIABLE (*) ["*"]
    //   CompareAble = T_VARIABLE (*) ["+"]
    //   CompareAble = T_VARIABLE (*) ["-"]
    //   CompareAble = T_VARIABLE (*) ["<"]
    //   CompareAble = T_VARIABLE (*) ["=<"]
    //   CompareAble = T_VARIABLE (*) ["=="]
    //   CompareAble = T_VARIABLE (*) ["==="]
    //   CompareAble = T_VARIABLE (*) ["=>"]
    //   CompareAble = T_VARIABLE (*) [">"]
    //   CompareAble = T_VARIABLE (*) ["\\\\"]
    //
    //   ")" -> Reduce(CompareAble = T_VARIABLE => Call(ActionFn(11));)
    //   "*" -> Reduce(CompareAble = T_VARIABLE => Call(ActionFn(11));)
    //   "+" -> Reduce(CompareAble = T_VARIABLE => Call(ActionFn(11));)
    //   "-" -> Reduce(CompareAble = T_VARIABLE => Call(ActionFn(11));)
    //   "<" -> Reduce(CompareAble = T_VARIABLE => Call(ActionFn(11));)
    //   "=<" -> Reduce(CompareAble = T_VARIABLE => Call(ActionFn(11));)
    //   "==" -> Reduce(CompareAble = T_VARIABLE => Call(ActionFn(11));)
    //   "===" -> Reduce(CompareAble = T_VARIABLE => Call(ActionFn(11));)
    //   "=>" -> Reduce(CompareAble = T_VARIABLE => Call(ActionFn(11));)
    //   ">" -> Reduce(CompareAble = T_VARIABLE => Call(ActionFn(11));)
    //   "\\\\" -> Reduce(CompareAble = T_VARIABLE => Call(ActionFn(11));)
    //
    pub fn __state10<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Box<String>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (11, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action11(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::CompareAble(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 11
    //   T_NUMBER = r#"([0-9]+)"# (*) [")"]
    //   T_NUMBER = r#"([0-9]+)"# (*) ["*"]
    //   T_NUMBER = r#"([0-9]+)"# (*) ["+"]
    //   T_NUMBER = r#"([0-9]+)"# (*) ["-"]
    //   T_NUMBER = r#"([0-9]+)"# (*) ["<"]
    //   T_NUMBER = r#"([0-9]+)"# (*) ["=<"]
    //   T_NUMBER = r#"([0-9]+)"# (*) ["=="]
    //   T_NUMBER = r#"([0-9]+)"# (*) ["==="]
    //   T_NUMBER = r#"([0-9]+)"# (*) ["=>"]
    //   T_NUMBER = r#"([0-9]+)"# (*) [">"]
    //   T_NUMBER = r#"([0-9]+)"# (*) ["\\\\"]
    //
    //   ")" -> Reduce(T_NUMBER = r#"([0-9]+)"# => Call(ActionFn(4));)
    //   "*" -> Reduce(T_NUMBER = r#"([0-9]+)"# => Call(ActionFn(4));)
    //   "+" -> Reduce(T_NUMBER = r#"([0-9]+)"# => Call(ActionFn(4));)
    //   "-" -> Reduce(T_NUMBER = r#"([0-9]+)"# => Call(ActionFn(4));)
    //   "<" -> Reduce(T_NUMBER = r#"([0-9]+)"# => Call(ActionFn(4));)
    //   "=<" -> Reduce(T_NUMBER = r#"([0-9]+)"# => Call(ActionFn(4));)
    //   "==" -> Reduce(T_NUMBER = r#"([0-9]+)"# => Call(ActionFn(4));)
    //   "===" -> Reduce(T_NUMBER = r#"([0-9]+)"# => Call(ActionFn(4));)
    //   "=>" -> Reduce(T_NUMBER = r#"([0-9]+)"# => Call(ActionFn(4));)
    //   ">" -> Reduce(T_NUMBER = r#"([0-9]+)"# => Call(ActionFn(4));)
    //   "\\\\" -> Reduce(T_NUMBER = r#"([0-9]+)"# => Call(ActionFn(4));)
    //
    pub fn __state11<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (11, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action4(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::T__NUMBER(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 12
    //   T_NUMBER = r#"([0-9]+)\\.([0-9]+)"# (*) [")"]
    //   T_NUMBER = r#"([0-9]+)\\.([0-9]+)"# (*) ["*"]
    //   T_NUMBER = r#"([0-9]+)\\.([0-9]+)"# (*) ["+"]
    //   T_NUMBER = r#"([0-9]+)\\.([0-9]+)"# (*) ["-"]
    //   T_NUMBER = r#"([0-9]+)\\.([0-9]+)"# (*) ["<"]
    //   T_NUMBER = r#"([0-9]+)\\.([0-9]+)"# (*) ["=<"]
    //   T_NUMBER = r#"([0-9]+)\\.([0-9]+)"# (*) ["=="]
    //   T_NUMBER = r#"([0-9]+)\\.([0-9]+)"# (*) ["==="]
    //   T_NUMBER = r#"([0-9]+)\\.([0-9]+)"# (*) ["=>"]
    //   T_NUMBER = r#"([0-9]+)\\.([0-9]+)"# (*) [">"]
    //   T_NUMBER = r#"([0-9]+)\\.([0-9]+)"# (*) ["\\\\"]
    //
    //   ")" -> Reduce(T_NUMBER = r#"([0-9]+)\\.([0-9]+)"# => Call(ActionFn(5));)
    //   "*" -> Reduce(T_NUMBER = r#"([0-9]+)\\.([0-9]+)"# => Call(ActionFn(5));)
    //   "+" -> Reduce(T_NUMBER = r#"([0-9]+)\\.([0-9]+)"# => Call(ActionFn(5));)
    //   "-" -> Reduce(T_NUMBER = r#"([0-9]+)\\.([0-9]+)"# => Call(ActionFn(5));)
    //   "<" -> Reduce(T_NUMBER = r#"([0-9]+)\\.([0-9]+)"# => Call(ActionFn(5));)
    //   "=<" -> Reduce(T_NUMBER = r#"([0-9]+)\\.([0-9]+)"# => Call(ActionFn(5));)
    //   "==" -> Reduce(T_NUMBER = r#"([0-9]+)\\.([0-9]+)"# => Call(ActionFn(5));)
    //   "===" -> Reduce(T_NUMBER = r#"([0-9]+)\\.([0-9]+)"# => Call(ActionFn(5));)
    //   "=>" -> Reduce(T_NUMBER = r#"([0-9]+)\\.([0-9]+)"# => Call(ActionFn(5));)
    //   ">" -> Reduce(T_NUMBER = r#"([0-9]+)\\.([0-9]+)"# => Call(ActionFn(5));)
    //   "\\\\" -> Reduce(T_NUMBER = r#"([0-9]+)\\.([0-9]+)"# => Call(ActionFn(5));)
    //
    pub fn __state12<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (11, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action5(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::T__NUMBER(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 13
    //   T_VARIABLE = r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# (*) [")"]
    //   T_VARIABLE = r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# (*) ["*"]
    //   T_VARIABLE = r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# (*) ["+"]
    //   T_VARIABLE = r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# (*) ["-"]
    //   T_VARIABLE = r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# (*) ["<"]
    //   T_VARIABLE = r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# (*) ["=<"]
    //   T_VARIABLE = r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# (*) ["=="]
    //   T_VARIABLE = r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# (*) ["==="]
    //   T_VARIABLE = r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# (*) ["=>"]
    //   T_VARIABLE = r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# (*) [">"]
    //   T_VARIABLE = r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# (*) ["\\\\"]
    //
    //   ")" -> Reduce(T_VARIABLE = r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# => Call(ActionFn(6));)
    //   "*" -> Reduce(T_VARIABLE = r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# => Call(ActionFn(6));)
    //   "+" -> Reduce(T_VARIABLE = r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# => Call(ActionFn(6));)
    //   "-" -> Reduce(T_VARIABLE = r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# => Call(ActionFn(6));)
    //   "<" -> Reduce(T_VARIABLE = r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# => Call(ActionFn(6));)
    //   "=<" -> Reduce(T_VARIABLE = r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# => Call(ActionFn(6));)
    //   "==" -> Reduce(T_VARIABLE = r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# => Call(ActionFn(6));)
    //   "===" -> Reduce(T_VARIABLE = r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# => Call(ActionFn(6));)
    //   "=>" -> Reduce(T_VARIABLE = r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# => Call(ActionFn(6));)
    //   ">" -> Reduce(T_VARIABLE = r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# => Call(ActionFn(6));)
    //   "\\\\" -> Reduce(T_VARIABLE = r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# => Call(ActionFn(6));)
    //
    pub fn __state13<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (11, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action6(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::T__VARIABLE(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 14
    //   CompareAble = (*) T_NUMBER [")"]
    //   CompareAble = (*) T_NUMBER ["*"]
    //   CompareAble = (*) T_NUMBER ["+"]
    //   CompareAble = (*) T_NUMBER ["-"]
    //   CompareAble = (*) T_NUMBER ["<"]
    //   CompareAble = (*) T_NUMBER ["=<"]
    //   CompareAble = (*) T_NUMBER ["=="]
    //   CompareAble = (*) T_NUMBER ["==="]
    //   CompareAble = (*) T_NUMBER ["=>"]
    //   CompareAble = (*) T_NUMBER [">"]
    //   CompareAble = (*) T_NUMBER ["\\\\"]
    //   CompareAble = (*) T_VARIABLE [")"]
    //   CompareAble = (*) T_VARIABLE ["*"]
    //   CompareAble = (*) T_VARIABLE ["+"]
    //   CompareAble = (*) T_VARIABLE ["-"]
    //   CompareAble = (*) T_VARIABLE ["<"]
    //   CompareAble = (*) T_VARIABLE ["=<"]
    //   CompareAble = (*) T_VARIABLE ["=="]
    //   CompareAble = (*) T_VARIABLE ["==="]
    //   CompareAble = (*) T_VARIABLE ["=>"]
    //   CompareAble = (*) T_VARIABLE [">"]
    //   CompareAble = (*) T_VARIABLE ["\\\\"]
    //   Expression = (*) CompareAble [")"]
    //   Expression = (*) ExpressionMath [")"]
    //   ExpressionMath = (*) CompareAble Operator Expression [")"]
    //   ExpressionMath = CompareAble Operator (*) Expression [")"]
    //   T_NUMBER = (*) r#"([0-9]+)"# [")"]
    //   T_NUMBER = (*) r#"([0-9]+)"# ["*"]
    //   T_NUMBER = (*) r#"([0-9]+)"# ["+"]
    //   T_NUMBER = (*) r#"([0-9]+)"# ["-"]
    //   T_NUMBER = (*) r#"([0-9]+)"# ["<"]
    //   T_NUMBER = (*) r#"([0-9]+)"# ["=<"]
    //   T_NUMBER = (*) r#"([0-9]+)"# ["=="]
    //   T_NUMBER = (*) r#"([0-9]+)"# ["==="]
    //   T_NUMBER = (*) r#"([0-9]+)"# ["=>"]
    //   T_NUMBER = (*) r#"([0-9]+)"# [">"]
    //   T_NUMBER = (*) r#"([0-9]+)"# ["\\\\"]
    //   T_NUMBER = (*) r#"([0-9]+)\\.([0-9]+)"# [")"]
    //   T_NUMBER = (*) r#"([0-9]+)\\.([0-9]+)"# ["*"]
    //   T_NUMBER = (*) r#"([0-9]+)\\.([0-9]+)"# ["+"]
    //   T_NUMBER = (*) r#"([0-9]+)\\.([0-9]+)"# ["-"]
    //   T_NUMBER = (*) r#"([0-9]+)\\.([0-9]+)"# ["<"]
    //   T_NUMBER = (*) r#"([0-9]+)\\.([0-9]+)"# ["=<"]
    //   T_NUMBER = (*) r#"([0-9]+)\\.([0-9]+)"# ["=="]
    //   T_NUMBER = (*) r#"([0-9]+)\\.([0-9]+)"# ["==="]
    //   T_NUMBER = (*) r#"([0-9]+)\\.([0-9]+)"# ["=>"]
    //   T_NUMBER = (*) r#"([0-9]+)\\.([0-9]+)"# [">"]
    //   T_NUMBER = (*) r#"([0-9]+)\\.([0-9]+)"# ["\\\\"]
    //   T_VARIABLE = (*) r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# [")"]
    //   T_VARIABLE = (*) r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# ["*"]
    //   T_VARIABLE = (*) r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# ["+"]
    //   T_VARIABLE = (*) r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# ["-"]
    //   T_VARIABLE = (*) r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# ["<"]
    //   T_VARIABLE = (*) r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# ["=<"]
    //   T_VARIABLE = (*) r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# ["=="]
    //   T_VARIABLE = (*) r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# ["==="]
    //   T_VARIABLE = (*) r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# ["=>"]
    //   T_VARIABLE = (*) r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# [">"]
    //   T_VARIABLE = (*) r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# ["\\\\"]
    //
    //   r#"([0-9]+)"# -> Shift(S11)
    //   r#"([0-9]+)\\.([0-9]+)"# -> Shift(S12)
    //   r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# -> Shift(S13)
    //
    //   CompareAble -> S6
    //   Expression -> S28
    //   ExpressionMath -> S8
    //   T_NUMBER -> S9
    //   T_VARIABLE -> S10
    pub fn __state14<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Box<String>>,
        __sym1: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (18, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state11(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (19, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state12(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (20, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state13(input, __lookbehind, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::CompareAble(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state6(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Expression(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state28(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::ExpressionMath(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state8(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::T__NUMBER(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state9(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::T__VARIABLE(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state10(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 15
    //   Operator = OperatorComparison (*) [r#"([0-9]+)"#]
    //   Operator = OperatorComparison (*) [r#"([0-9]+)\\.([0-9]+)"#]
    //   Operator = OperatorComparison (*) [r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"#]
    //
    //   r#"([0-9]+)"# -> Reduce(Operator = OperatorComparison => Call(ActionFn(18));)
    //   r#"([0-9]+)\\.([0-9]+)"# -> Reduce(Operator = OperatorComparison => Call(ActionFn(18));)
    //   r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# -> Reduce(Operator = OperatorComparison => Call(ActionFn(18));)
    //
    pub fn __state15<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action18(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Operator(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 16
    //   Operator = OperatorMath (*) [r#"([0-9]+)"#]
    //   Operator = OperatorMath (*) [r#"([0-9]+)\\.([0-9]+)"#]
    //   Operator = OperatorMath (*) [r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"#]
    //
    //   r#"([0-9]+)"# -> Reduce(Operator = OperatorMath => Call(ActionFn(19));)
    //   r#"([0-9]+)\\.([0-9]+)"# -> Reduce(Operator = OperatorMath => Call(ActionFn(19));)
    //   r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# -> Reduce(Operator = OperatorMath => Call(ActionFn(19));)
    //
    pub fn __state16<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action19(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Operator(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 17
    //   OperatorMath = "*" (*) [r#"([0-9]+)"#]
    //   OperatorMath = "*" (*) [r#"([0-9]+)\\.([0-9]+)"#]
    //   OperatorMath = "*" (*) [r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"#]
    //
    //   r#"([0-9]+)"# -> Reduce(OperatorMath = "*" => Call(ActionFn(28));)
    //   r#"([0-9]+)\\.([0-9]+)"# -> Reduce(OperatorMath = "*" => Call(ActionFn(28));)
    //   r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# -> Reduce(OperatorMath = "*" => Call(ActionFn(28));)
    //
    pub fn __state17<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action28(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::OperatorMath(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 18
    //   OperatorMath = "+" (*) [r#"([0-9]+)"#]
    //   OperatorMath = "+" (*) [r#"([0-9]+)\\.([0-9]+)"#]
    //   OperatorMath = "+" (*) [r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"#]
    //
    //   r#"([0-9]+)"# -> Reduce(OperatorMath = "+" => Call(ActionFn(26));)
    //   r#"([0-9]+)\\.([0-9]+)"# -> Reduce(OperatorMath = "+" => Call(ActionFn(26));)
    //   r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# -> Reduce(OperatorMath = "+" => Call(ActionFn(26));)
    //
    pub fn __state18<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action26(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::OperatorMath(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 19
    //   OperatorMath = "-" (*) [r#"([0-9]+)"#]
    //   OperatorMath = "-" (*) [r#"([0-9]+)\\.([0-9]+)"#]
    //   OperatorMath = "-" (*) [r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"#]
    //
    //   r#"([0-9]+)"# -> Reduce(OperatorMath = "-" => Call(ActionFn(27));)
    //   r#"([0-9]+)\\.([0-9]+)"# -> Reduce(OperatorMath = "-" => Call(ActionFn(27));)
    //   r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# -> Reduce(OperatorMath = "-" => Call(ActionFn(27));)
    //
    pub fn __state19<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action27(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::OperatorMath(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 20
    //   OperatorComparison = "<" (*) [r#"([0-9]+)"#]
    //   OperatorComparison = "<" (*) [r#"([0-9]+)\\.([0-9]+)"#]
    //   OperatorComparison = "<" (*) [r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"#]
    //
    //   r#"([0-9]+)"# -> Reduce(OperatorComparison = "<" => Call(ActionFn(24));)
    //   r#"([0-9]+)\\.([0-9]+)"# -> Reduce(OperatorComparison = "<" => Call(ActionFn(24));)
    //   r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# -> Reduce(OperatorComparison = "<" => Call(ActionFn(24));)
    //
    pub fn __state20<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action24(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::OperatorComparison(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 21
    //   OperatorComparison = "=<" (*) [r#"([0-9]+)"#]
    //   OperatorComparison = "=<" (*) [r#"([0-9]+)\\.([0-9]+)"#]
    //   OperatorComparison = "=<" (*) [r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"#]
    //
    //   r#"([0-9]+)"# -> Reduce(OperatorComparison = "=<" => Call(ActionFn(22));)
    //   r#"([0-9]+)\\.([0-9]+)"# -> Reduce(OperatorComparison = "=<" => Call(ActionFn(22));)
    //   r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# -> Reduce(OperatorComparison = "=<" => Call(ActionFn(22));)
    //
    pub fn __state21<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action22(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::OperatorComparison(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 22
    //   OperatorComparison = "==" (*) [r#"([0-9]+)"#]
    //   OperatorComparison = "==" (*) [r#"([0-9]+)\\.([0-9]+)"#]
    //   OperatorComparison = "==" (*) [r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"#]
    //
    //   r#"([0-9]+)"# -> Reduce(OperatorComparison = "==" => Call(ActionFn(20));)
    //   r#"([0-9]+)\\.([0-9]+)"# -> Reduce(OperatorComparison = "==" => Call(ActionFn(20));)
    //   r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# -> Reduce(OperatorComparison = "==" => Call(ActionFn(20));)
    //
    pub fn __state22<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action20(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::OperatorComparison(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 23
    //   OperatorComparison = "===" (*) [r#"([0-9]+)"#]
    //   OperatorComparison = "===" (*) [r#"([0-9]+)\\.([0-9]+)"#]
    //   OperatorComparison = "===" (*) [r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"#]
    //
    //   r#"([0-9]+)"# -> Reduce(OperatorComparison = "===" => Call(ActionFn(21));)
    //   r#"([0-9]+)\\.([0-9]+)"# -> Reduce(OperatorComparison = "===" => Call(ActionFn(21));)
    //   r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# -> Reduce(OperatorComparison = "===" => Call(ActionFn(21));)
    //
    pub fn __state23<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action21(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::OperatorComparison(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 24
    //   OperatorComparison = "=>" (*) [r#"([0-9]+)"#]
    //   OperatorComparison = "=>" (*) [r#"([0-9]+)\\.([0-9]+)"#]
    //   OperatorComparison = "=>" (*) [r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"#]
    //
    //   r#"([0-9]+)"# -> Reduce(OperatorComparison = "=>" => Call(ActionFn(23));)
    //   r#"([0-9]+)\\.([0-9]+)"# -> Reduce(OperatorComparison = "=>" => Call(ActionFn(23));)
    //   r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# -> Reduce(OperatorComparison = "=>" => Call(ActionFn(23));)
    //
    pub fn __state24<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action23(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::OperatorComparison(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 25
    //   OperatorComparison = ">" (*) [r#"([0-9]+)"#]
    //   OperatorComparison = ">" (*) [r#"([0-9]+)\\.([0-9]+)"#]
    //   OperatorComparison = ">" (*) [r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"#]
    //
    //   r#"([0-9]+)"# -> Reduce(OperatorComparison = ">" => Call(ActionFn(25));)
    //   r#"([0-9]+)\\.([0-9]+)"# -> Reduce(OperatorComparison = ">" => Call(ActionFn(25));)
    //   r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# -> Reduce(OperatorComparison = ">" => Call(ActionFn(25));)
    //
    pub fn __state25<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action25(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::OperatorComparison(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 26
    //   OperatorMath = "\\\\" (*) [r#"([0-9]+)"#]
    //   OperatorMath = "\\\\" (*) [r#"([0-9]+)\\.([0-9]+)"#]
    //   OperatorMath = "\\\\" (*) [r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"#]
    //
    //   r#"([0-9]+)"# -> Reduce(OperatorMath = "\\\\" => Call(ActionFn(29));)
    //   r#"([0-9]+)\\.([0-9]+)"# -> Reduce(OperatorMath = "\\\\" => Call(ActionFn(29));)
    //   r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# -> Reduce(OperatorMath = "\\\\" => Call(ActionFn(29));)
    //
    pub fn __state26<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action29(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::OperatorMath(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 27
    //   Block = (*) ";" [EOF]
    //   Block = (*) "{}" [EOF]
    //   If = T_IF "(" Expression ")" (*) Block [EOF]
    //
    //   ";" -> Shift(S30)
    //   "{}" -> Shift(S31)
    //
    //   Block -> S29
    pub fn __state27<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<Box<Expression>>,
        __sym3: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (6, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym4 = &mut Some((__tok0));
                __result = try!(__state30(input, __lookbehind, __tokens, __sym4));
            }
            Some((_, (16, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym4 = &mut Some((__tok0));
                __result = try!(__state31(input, __lookbehind, __tokens, __sym4));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym3.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Block(__nt) => {
                    let __sym4 = &mut Some(__nt);
                    __result = try!(__state29(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2, __sym3, __sym4));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 28
    //   ExpressionMath = CompareAble Operator Expression (*) [")"]
    //
    //   ")" -> Reduce(ExpressionMath = CompareAble, Operator, Expression => Call(ActionFn(17));)
    //
    pub fn __state28<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Box<String>>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<Box<Expression>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (2, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action17(input, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::ExpressionMath(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 29
    //   If = T_IF "(" Expression ")" Block (*) [EOF]
    //
    //   EOF -> Reduce(If = T_IF, "(", Expression, ")", Block => Call(ActionFn(7));)
    //
    pub fn __state29<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<Box<Expression>>,
        __sym3: &mut Option<&'input str>,
        __sym4: &mut Option<Box<Block>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __sym3 = __sym3.take().unwrap();
                let __sym4 = __sym4.take().unwrap();
                let __nt = super::__action7(input, __sym0, __sym1, __sym2, __sym3, __sym4);
                return Ok((__lookbehind, __lookahead, __Nonterminal::If(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 30
    //   Block = ";" (*) [EOF]
    //
    //   EOF -> Reduce(Block = ";" => Call(ActionFn(14));)
    //
    pub fn __state30<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action14(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Block(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 31
    //   Block = "{}" (*) [EOF]
    //
    //   EOF -> Reduce(Block = "{}" => Call(ActionFn(13));)
    //
    pub fn __state31<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action13(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Block(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }
}
pub use self::__parse__Start::parse_Start;

mod __parse__simple_variable {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use std::str::FromStr;
    use ast::{If, Expression, Block, Variable};
    extern crate lalrpop_util as __lalrpop_util;
    use self::__lalrpop_util::ParseError as __ParseError;
    pub fn parse_simple_variable<
        'input,
    >(
        input: &'input str,
    ) -> Result<Box<Variable>, __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match try!(__state0(input, None, &mut __tokens, __lookahead)) {
            (_, Some(__lookahead), _) => {
                Err(__ParseError::ExtraToken { token: __lookahead })
            }
            (_, None, __Nonterminal::____simple__variable(__nt)) => {
                Ok(__nt)
            }
            _ => unreachable!(),
        }
    }

    #[allow(dead_code)]
    pub enum __Nonterminal<'input> {
        Block(Box<Block>),
        CompareAble(Box<String>),
        Expression(Box<Expression>),
        ExpressionMath(Box<Expression>),
        If(Box<If>),
        Operator(&'input str),
        OperatorComparison(&'input str),
        OperatorMath(&'input str),
        Start(Box<If>),
        T__IF(&'input str),
        T__NUMBER(Box<String>),
        T__VARIABLE(Box<String>),
        ____Start(Box<If>),
        ____simple__variable(Box<Variable>),
        simple__variable(Box<Variable>),
    }

    // State 0
    //   T_VARIABLE = (*) r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# [EOF]
    //   __simple_variable = (*) simple_variable [EOF]
    //   simple_variable = (*) T_VARIABLE [EOF]
    //   simple_variable = (*) "$" simple_variable [EOF]
    //   simple_variable = (*) "$" "{" simple_variable "}" [EOF]
    //
    //   "$" -> Shift(S3)
    //   r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# -> Shift(S4)
    //
    //   T_VARIABLE -> S1
    //   simple_variable -> S2
    pub fn __state0<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym0 = &mut Some((__tok0));
                __result = try!(__state3(input, __lookbehind, __tokens, __sym0));
            }
            Some((_, (20, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym0 = &mut Some((__tok0));
                __result = try!(__state4(input, __lookbehind, __tokens, __sym0));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        loop {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::T__VARIABLE(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state1(input, __lookbehind, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::simple__variable(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state2(input, __lookbehind, __tokens, __lookahead, __sym0));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
    }

    // State 1
    //   simple_variable = T_VARIABLE (*) [EOF]
    //
    //   EOF -> Reduce(simple_variable = T_VARIABLE => Call(ActionFn(8));)
    //
    pub fn __state1<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Box<String>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action8(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::simple__variable(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 2
    //   __simple_variable = simple_variable (*) [EOF]
    //
    //   EOF -> Reduce(__simple_variable = simple_variable => Call(ActionFn(1));)
    //
    pub fn __state2<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Box<Variable>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action1(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::____simple__variable(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 3
    //   T_VARIABLE = (*) r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# [EOF]
    //   simple_variable = (*) T_VARIABLE [EOF]
    //   simple_variable = (*) "$" simple_variable [EOF]
    //   simple_variable = "$" (*) simple_variable [EOF]
    //   simple_variable = (*) "$" "{" simple_variable "}" [EOF]
    //   simple_variable = "$" (*) "{" simple_variable "}" [EOF]
    //
    //   "$" -> Shift(S3)
    //   "{" -> Shift(S6)
    //   r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# -> Shift(S4)
    //
    //   T_VARIABLE -> S1
    //   simple_variable -> S5
    pub fn __state3<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state3(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (15, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state6(input, __lookbehind, __tokens, __sym0, __sym1));
            }
            Some((_, (20, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state4(input, __lookbehind, __tokens, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::T__VARIABLE(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state1(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::simple__variable(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state5(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 4
    //   T_VARIABLE = r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# (*) [EOF]
    //
    //   EOF -> Reduce(T_VARIABLE = r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# => Call(ActionFn(6));)
    //
    pub fn __state4<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action6(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::T__VARIABLE(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 5
    //   simple_variable = "$" simple_variable (*) [EOF]
    //
    //   EOF -> Reduce(simple_variable = "$", simple_variable => Call(ActionFn(10));)
    //
    pub fn __state5<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<Box<Variable>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __nt = super::__action10(input, __sym0, __sym1);
                return Ok((__lookbehind, __lookahead, __Nonterminal::simple__variable(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 6
    //   T_VARIABLE = (*) r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# ["}"]
    //   simple_variable = (*) T_VARIABLE ["}"]
    //   simple_variable = (*) "$" simple_variable ["}"]
    //   simple_variable = (*) "$" "{" simple_variable "}" ["}"]
    //   simple_variable = "$" "{" (*) simple_variable "}" [EOF]
    //
    //   "$" -> Shift(S9)
    //   r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# -> Shift(S10)
    //
    //   T_VARIABLE -> S7
    //   simple_variable -> S8
    pub fn __state6<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state9(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (20, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state10(input, __lookbehind, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::T__VARIABLE(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state7(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::simple__variable(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state8(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 7
    //   simple_variable = T_VARIABLE (*) ["}"]
    //
    //   "}" -> Reduce(simple_variable = T_VARIABLE => Call(ActionFn(8));)
    //
    pub fn __state7<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Box<String>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (17, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action8(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::simple__variable(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 8
    //   simple_variable = "$" "{" simple_variable (*) "}" [EOF]
    //
    //   "}" -> Shift(S11)
    //
    pub fn __state8<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<Box<Variable>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (17, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state11(input, __lookbehind, __tokens, __sym0, __sym1, __sym2, __sym3));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 9
    //   T_VARIABLE = (*) r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# ["}"]
    //   simple_variable = (*) T_VARIABLE ["}"]
    //   simple_variable = (*) "$" simple_variable ["}"]
    //   simple_variable = "$" (*) simple_variable ["}"]
    //   simple_variable = (*) "$" "{" simple_variable "}" ["}"]
    //   simple_variable = "$" (*) "{" simple_variable "}" ["}"]
    //
    //   "$" -> Shift(S9)
    //   "{" -> Shift(S13)
    //   r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# -> Shift(S10)
    //
    //   T_VARIABLE -> S7
    //   simple_variable -> S12
    pub fn __state9<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state9(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (15, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state13(input, __lookbehind, __tokens, __sym0, __sym1));
            }
            Some((_, (20, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state10(input, __lookbehind, __tokens, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::T__VARIABLE(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state7(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::simple__variable(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state12(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 10
    //   T_VARIABLE = r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# (*) ["}"]
    //
    //   "}" -> Reduce(T_VARIABLE = r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# => Call(ActionFn(6));)
    //
    pub fn __state10<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (17, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action6(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::T__VARIABLE(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 11
    //   simple_variable = "$" "{" simple_variable "}" (*) [EOF]
    //
    //   EOF -> Reduce(simple_variable = "$", "{", simple_variable, "}" => Call(ActionFn(9));)
    //
    pub fn __state11<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<Box<Variable>>,
        __sym3: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __sym3 = __sym3.take().unwrap();
                let __nt = super::__action9(input, __sym0, __sym1, __sym2, __sym3);
                return Ok((__lookbehind, __lookahead, __Nonterminal::simple__variable(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 12
    //   simple_variable = "$" simple_variable (*) ["}"]
    //
    //   "}" -> Reduce(simple_variable = "$", simple_variable => Call(ActionFn(10));)
    //
    pub fn __state12<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<Box<Variable>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (17, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __nt = super::__action10(input, __sym0, __sym1);
                return Ok((__lookbehind, __lookahead, __Nonterminal::simple__variable(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 13
    //   T_VARIABLE = (*) r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# ["}"]
    //   simple_variable = (*) T_VARIABLE ["}"]
    //   simple_variable = (*) "$" simple_variable ["}"]
    //   simple_variable = (*) "$" "{" simple_variable "}" ["}"]
    //   simple_variable = "$" "{" (*) simple_variable "}" ["}"]
    //
    //   "$" -> Shift(S9)
    //   r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# -> Shift(S10)
    //
    //   T_VARIABLE -> S7
    //   simple_variable -> S14
    pub fn __state13<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state9(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (20, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state10(input, __lookbehind, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::T__VARIABLE(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state7(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::simple__variable(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state14(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 14
    //   simple_variable = "$" "{" simple_variable (*) "}" ["}"]
    //
    //   "}" -> Shift(S15)
    //
    pub fn __state14<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<Box<Variable>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (17, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state15(input, __lookbehind, __tokens, __sym0, __sym1, __sym2, __sym3));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 15
    //   simple_variable = "$" "{" simple_variable "}" (*) ["}"]
    //
    //   "}" -> Reduce(simple_variable = "$", "{", simple_variable, "}" => Call(ActionFn(9));)
    //
    pub fn __state15<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<Box<Variable>>,
        __sym3: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (17, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __sym3 = __sym3.take().unwrap();
                let __nt = super::__action9(input, __sym0, __sym1, __sym2, __sym3);
                return Ok((__lookbehind, __lookahead, __Nonterminal::simple__variable(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }
}
pub use self::__parse__simple_variable::parse_simple_variable;
mod __intern_token {
    extern crate lalrpop_util as __lalrpop_util;
    use self::__lalrpop_util::ParseError as __ParseError;
    pub struct __Matcher<'input> {
        text: &'input str,
        consumed: usize,
    }

    fn __tokenize(text: &str) -> Option<(usize, usize)> {
        let mut __chars = text.char_indices();
        let mut __current_match: Option<(usize, usize)> = None;
        let mut __current_state: usize = 0;
        loop {
            match __current_state {
                0 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '$' => {
                            __current_match = Some((0, __index + 1));
                            __current_state = 1;
                            continue;
                        }
                        '(' => {
                            __current_match = Some((1, __index + 1));
                            __current_state = 2;
                            continue;
                        }
                        ')' => {
                            __current_match = Some((2, __index + 1));
                            __current_state = 3;
                            continue;
                        }
                        '*' => {
                            __current_match = Some((3, __index + 1));
                            __current_state = 4;
                            continue;
                        }
                        '+' => {
                            __current_match = Some((4, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '-' => {
                            __current_match = Some((5, __index + 1));
                            __current_state = 6;
                            continue;
                        }
                        '0' => {
                            __current_match = Some((18, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((18, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((18, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((18, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((18, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((18, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((18, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((18, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((18, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((18, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        ';' => {
                            __current_match = Some((6, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        '<' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 9;
                            continue;
                        }
                        '=' => {
                            __current_state = 10;
                            continue;
                        }
                        '>' => {
                            __current_match = Some((12, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        '\\' => {
                            __current_state = 12;
                            continue;
                        }
                        'i' => {
                            __current_state = 13;
                            continue;
                        }
                        '{' => {
                            __current_match = Some((15, __index + 1));
                            __current_state = 14;
                            continue;
                        }
                        '}' => {
                            __current_match = Some((17, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                1 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'A' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        'B' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        'C' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        'D' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        'E' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        'F' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        'G' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        'H' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        'I' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        'J' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        'K' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        'L' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        'M' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        'N' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        'O' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        'P' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        'Q' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        'R' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        'S' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        'T' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        'U' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        'V' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        'W' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        'X' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        'Y' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        'Z' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        '\\' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        '_' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        's' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        't' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                2 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                3 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                4 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                5 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                6 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                7 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '.' => {
                            __current_state = 18;
                            continue;
                        }
                        '0' => {
                            __current_match = Some((18, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((18, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((18, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((18, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((18, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((18, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((18, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((18, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((18, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((18, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                8 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                9 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                10 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '<' => {
                            __current_match = Some((8, __index + 1));
                            __current_state = 19;
                            continue;
                        }
                        '=' => {
                            __current_match = Some((9, __index + 1));
                            __current_state = 20;
                            continue;
                        }
                        '>' => {
                            __current_match = Some((11, __index + 1));
                            __current_state = 21;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                11 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                12 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '\\' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 22;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                13 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'f' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 23;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                14 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '}' => {
                            __current_match = Some((16, __index + 1));
                            __current_state = 24;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                15 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                16 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                17 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '0' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 25;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 25;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 25;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 25;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 25;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 25;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 25;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 25;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 25;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 25;
                            continue;
                        }
                        'A' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        'B' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        'C' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        'D' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        'E' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        'F' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        'G' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        'H' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        'I' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        'J' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        'K' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        'L' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        'M' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        'N' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        'O' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        'P' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        'Q' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        'R' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        'S' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        'T' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        'U' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        'V' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        'W' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        'X' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        'Y' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        'Z' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        '\\' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        '_' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        's' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        't' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                18 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '0' => {
                            __current_match = Some((19, __index + 1));
                            __current_state = 26;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((19, __index + 1));
                            __current_state = 26;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((19, __index + 1));
                            __current_state = 26;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((19, __index + 1));
                            __current_state = 26;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((19, __index + 1));
                            __current_state = 26;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((19, __index + 1));
                            __current_state = 26;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((19, __index + 1));
                            __current_state = 26;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((19, __index + 1));
                            __current_state = 26;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((19, __index + 1));
                            __current_state = 26;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((19, __index + 1));
                            __current_state = 26;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                19 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                20 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '=' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 27;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                21 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                22 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                23 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                24 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                25 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '0' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 25;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 25;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 25;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 25;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 25;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 25;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 25;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 25;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 25;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 25;
                            continue;
                        }
                        'A' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 25;
                            continue;
                        }
                        'B' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 25;
                            continue;
                        }
                        'C' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 25;
                            continue;
                        }
                        'D' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 25;
                            continue;
                        }
                        'E' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 25;
                            continue;
                        }
                        'F' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 25;
                            continue;
                        }
                        'G' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 25;
                            continue;
                        }
                        'H' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 25;
                            continue;
                        }
                        'I' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 25;
                            continue;
                        }
                        'J' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 25;
                            continue;
                        }
                        'K' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 25;
                            continue;
                        }
                        'L' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 25;
                            continue;
                        }
                        'M' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 25;
                            continue;
                        }
                        'N' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 25;
                            continue;
                        }
                        'O' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 25;
                            continue;
                        }
                        'P' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 25;
                            continue;
                        }
                        'Q' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 25;
                            continue;
                        }
                        'R' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 25;
                            continue;
                        }
                        'S' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 25;
                            continue;
                        }
                        'T' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 25;
                            continue;
                        }
                        'U' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 25;
                            continue;
                        }
                        'V' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 25;
                            continue;
                        }
                        'W' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 25;
                            continue;
                        }
                        'X' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 25;
                            continue;
                        }
                        'Y' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 25;
                            continue;
                        }
                        'Z' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 25;
                            continue;
                        }
                        '\\' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 25;
                            continue;
                        }
                        '_' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 25;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 25;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 25;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 25;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 25;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 25;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 25;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 25;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 25;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 25;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 25;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 25;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 25;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 25;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 25;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 25;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 25;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 25;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 25;
                            continue;
                        }
                        's' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 25;
                            continue;
                        }
                        't' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 25;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 25;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 25;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 25;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 25;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 25;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 25;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                26 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '0' => {
                            __current_match = Some((19, __index + 1));
                            __current_state = 26;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((19, __index + 1));
                            __current_state = 26;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((19, __index + 1));
                            __current_state = 26;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((19, __index + 1));
                            __current_state = 26;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((19, __index + 1));
                            __current_state = 26;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((19, __index + 1));
                            __current_state = 26;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((19, __index + 1));
                            __current_state = 26;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((19, __index + 1));
                            __current_state = 26;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((19, __index + 1));
                            __current_state = 26;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((19, __index + 1));
                            __current_state = 26;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                27 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                _ => { panic!("invalid state {}", __current_state); }
            }
        }
    }

    impl<'input> __Matcher<'input> {
        pub fn new(s: &'input str) -> __Matcher<'input> {
            __Matcher { text: s, consumed: 0 }
        }
    }

    impl<'input> Iterator for __Matcher<'input> {
        type Item = Result<(usize, (usize, &'input str), usize), __ParseError<usize,(usize, &'input str),()>>;

        fn next(&mut self) -> Option<Self::Item> {
            let __text = self.text.trim_left();
            let __whitespace = self.text.len() - __text.len();
            let __start_offset = self.consumed + __whitespace;
            if __text.is_empty() {
                self.text = __text;
                self.consumed = __start_offset;
                None
            } else {
                match __tokenize(__text) {
                    Some((__index, __length)) => {
                        let __result = &__text[..__length];
                        let __remaining = &__text[__length..];
                        let __end_offset = __start_offset + __length;
                        self.text = __remaining;
                        self.consumed = __end_offset;
                        Some(Ok((__start_offset, (__index, __result), __end_offset)))
                    }
                    None => {
                        Some(Err(__ParseError::InvalidToken { location: __start_offset }))
                    }
                }
            }
        }
    }
}

pub fn __action0<
    'input,
>(
    input: &'input str,
    __0: Box<If>,
) -> Box<If>
{
    (__0)
}

pub fn __action1<
    'input,
>(
    input: &'input str,
    __0: Box<Variable>,
) -> Box<Variable>
{
    (__0)
}

pub fn __action2<
    'input,
>(
    input: &'input str,
    __0: Box<If>,
) -> Box<If>
{
    (__0)
}

pub fn __action3<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action4<
    'input,
>(
    input: &'input str,
    s: &'input str,
) -> Box<String>
{
    Box::new(s.to_string())
}

pub fn __action5<
    'input,
>(
    input: &'input str,
    s: &'input str,
) -> Box<String>
{
    Box::new(s.to_string())
}

pub fn __action6<
    'input,
>(
    input: &'input str,
    s: &'input str,
) -> Box<String>
{
    Box::new(s.to_string())
}

pub fn __action7<
    'input,
>(
    input: &'input str,
    a: &'input str,
    b: &'input str,
    expression: Box<Expression>,
    c: &'input str,
    block: Box<Block>,
) -> Box<If>
{
    { Box::new(If::new(expression, block)) }
}

pub fn __action8<
    'input,
>(
    input: &'input str,
    var: Box<String>,
) -> Box<Variable>
{
    { Box::new(Variable::Identifier(var)) }
}

pub fn __action9<
    'input,
>(
    input: &'input str,
    a: &'input str,
    b: &'input str,
    var: Box<Variable>,
    c: &'input str,
) -> Box<Variable>
{
    { Box::new(Variable::Variable(var)) }
}

pub fn __action10<
    'input,
>(
    input: &'input str,
    a: &'input str,
    var: Box<Variable>,
) -> Box<Variable>
{
    { Box::new(Variable::Variable(var)) }
}

pub fn __action11<
    'input,
>(
    input: &'input str,
    __0: Box<String>,
) -> Box<String>
{
    (__0)
}

pub fn __action12<
    'input,
>(
    input: &'input str,
    __0: Box<String>,
) -> Box<String>
{
    (__0)
}

pub fn __action13<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> Box<Block>
{
    { Box::new(Block) }
}

pub fn __action14<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> Box<Block>
{
    { Box::new(Block) } // Empty Block
}

pub fn __action15<
    'input,
>(
    input: &'input str,
    __0: Box<Expression>,
) -> Box<Expression>
{
    { Box::new(Expression) }
}

pub fn __action16<
    'input,
>(
    input: &'input str,
    __0: Box<String>,
) -> Box<Expression>
{
    { Box::new(Expression) }
}

pub fn __action17<
    'input,
>(
    input: &'input str,
    __0: Box<String>,
    __1: &'input str,
    __2: Box<Expression>,
) -> Box<Expression>
{
    { Box::new(Expression) }
}

pub fn __action18<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action19<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action20<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action21<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action22<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action23<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action24<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action25<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action26<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action27<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action28<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action29<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub trait __ToTriple<'input, > {
    type Error;
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),Self::Error>;
}

impl<'input, > __ToTriple<'input, > for (usize, (usize, &'input str), usize) {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),()> {
        Ok(value)
    }
}
impl<'input, > __ToTriple<'input, > for Result<(usize, (usize, &'input str), usize),()> {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),()> {
        value
    }
}
