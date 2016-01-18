#![allow(unused_imports)]
#![allow(unused_variables)]
use std::str::FromStr;
use ast::{If, Expression, Block, Variable};
extern crate lalrpop_util as __lalrpop_util;
use self::__lalrpop_util::ParseError as __ParseError;

mod __parse__expr {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use std::str::FromStr;
    use ast::{If, Expression, Block, Variable};
    extern crate lalrpop_util as __lalrpop_util;
    use self::__lalrpop_util::ParseError as __ParseError;
    pub fn parse_expr<
        'input,
    >(
        input: &'input str,
    ) -> Result<Box<Expression>, __ParseError<usize,(usize, &'input str),()>>
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
            (_, None, __Nonterminal::____expr(__nt)) => {
                Ok(__nt)
            }
            _ => unreachable!(),
        }
    }

    #[allow(dead_code)]
    pub enum __Nonterminal<'input> {
        T__ABSTRACT(&'input str),
        T__AND__EQUAL(&'input str),
        T__ARRAY(&'input str),
        T__ARRAY__CAST(&'input str),
        T__AS(&'input str),
        T__BOOLEAN__AND(&'input str),
        T__BOOLEAN__OR(&'input str),
        T__BOOL__CAST(&'input str),
        T__BREAK(&'input str),
        T__CALLABLE(&'input str),
        T__CASE(&'input str),
        T__CATCH(&'input str),
        T__CLASS(&'input str),
        T__CLASS__C(&'input str),
        T__CLONE(&'input str),
        T__CLOSE__TAG(&'input str),
        T__COALESCE(&'input str),
        T__COMMENT(&'input str),
        T__CONCAT__EQUAL(&'input str),
        T__CONST(&'input str),
        T__CONTINUE(&'input str),
        T__CURLY__OPEN(&'input str),
        T__DEC(&'input str),
        T__DECLARE(&'input str),
        T__DEFAULT(&'input str),
        T__DIR(&'input str),
        T__DIV__EQUAL(&'input str),
        T__DO(&'input str),
        T__DOC__COMMENT(&'input str),
        T__DOLLAR__OPEN__CURLY__BRACES(&'input str),
        T__DOUBLE__ARROW(&'input str),
        T__DOUBLE__CAST(&'input str),
        T__ECHO(&'input str),
        T__ELLIPSIS(&'input str),
        T__ELSE(&'input str),
        T__ELSEIF(&'input str),
        T__EMPTY(&'input str),
        T__ENDDECLARE(&'input str),
        T__ENDFOR(&'input str),
        T__ENDFOREACH(&'input str),
        T__ENDIF(&'input str),
        T__ENDSWITCH(&'input str),
        T__ENDWHILE(&'input str),
        T__END__HEREDOC(&'input str),
        T__EVAL(&'input str),
        T__EXIT(&'input str),
        T__EXTENDS(&'input str),
        T__FILE(&'input str),
        T__FINAL(&'input str),
        T__FINALLY(&'input str),
        T__FOR(&'input str),
        T__FOREACH(&'input str),
        T__FUNCTION(&'input str),
        T__FUNC__C(&'input str),
        T__GLOBAL(&'input str),
        T__GOTO(&'input str),
        T__HALT__COMPILER(&'input str),
        T__IF(&'input str),
        T__IMPLEMENTS(&'input str),
        T__INC(&'input str),
        T__INCLUDE(&'input str),
        T__INCLUDE__ONCE(&'input str),
        T__INSTANCEOF(&'input str),
        T__INSTEADOF(&'input str),
        T__INTERFACE(&'input str),
        T__INT__CAST(&'input str),
        T__ISSET(&'input str),
        T__IS__EQUAL(&'input str),
        T__IS__GREATER__OR__EQUAL(&'input str),
        T__IS__IDENTICAL(&'input str),
        T__IS__NOT__EQUAL(&'input str),
        T__IS__NOT__IDENTICAL(&'input str),
        T__IS__SMALLER__OR__EQUAL(&'input str),
        T__LINE(&'input str),
        T__LIST(&'input str),
        T__LOGICAL__AND(&'input str),
        T__LOGICAL__OR(&'input str),
        T__LOGICAL__XOR(&'input str),
        T__METHOD__C(&'input str),
        T__MINUS__EQUAL(&'input str),
        T__MOD__EQUAL(&'input str),
        T__MUL__EQUAL(&'input str),
        T__NAMESPACE(&'input str),
        T__NEW(&'input str),
        T__NS__C(&'input str),
        T__NS__SEPARATOR(&'input str),
        T__NUMBER(Box<String>),
        T__OBJECT__CAST(&'input str),
        T__OBJECT__OPERATOR(&'input str),
        T__OPEN__TAG(&'input str),
        T__OPEN__TAG__WITH__ECHO(&'input str),
        T__OR__EQUAL(&'input str),
        T__PAAMAYIM__NEKUDOTAYIM(&'input str),
        T__PLUS__EQUAL(&'input str),
        T__POW(&'input str),
        T__POW__EQUAL(&'input str),
        T__PRINT(&'input str),
        T__PRIVATE(&'input str),
        T__PROTECTED(&'input str),
        T__PUBLIC(&'input str),
        T__REQUIRE(&'input str),
        T__REQUIRE__ONCE(&'input str),
        T__RETURN(&'input str),
        T__SL(&'input str),
        T__SL__EQUAL(&'input str),
        T__SPACESHIP(&'input str),
        T__SR(&'input str),
        T__SR__EQUAL(&'input str),
        T__START__HEREDOC(&'input str),
        T__STATIC(&'input str),
        T__STRING__CAST(&'input str),
        T__SWITCH(&'input str),
        T__THROW(&'input str),
        T__TRAIT(&'input str),
        T__TRAIT__C(&'input str),
        T__TRY(&'input str),
        T__UNSET(&'input str),
        T__UNSET__CAST(&'input str),
        T__USE(&'input str),
        T__VAR(&'input str),
        T__VARIABLE(Box<String>),
        T__WHILE(&'input str),
        T__WHITESPACE(&'input str),
        T__XOR__EQUAL(&'input str),
        T__YIELD(&'input str),
        T__YIELD__FROM(&'input str),
        ____expr(Box<Expression>),
        ____simple__variable(Box<Variable>),
        ____variable(Box<Variable>),
        expr(Box<Expression>),
        expr__without__variable(Box<Expression>),
        simple__variable(Box<Variable>),
        variable(Box<Variable>),
    }

    // State 0
    //   T_VARIABLE = (*) r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# [EOF]
    //   T_VARIABLE = (*) r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# ["+="]
    //   __expr = (*) expr [EOF]
    //   expr = (*) expr_without_variable [EOF]
    //   expr = (*) variable [EOF]
    //   expr_without_variable = (*) variable T_PLUS_EQUAL expr [EOF]
    //   simple_variable = (*) T_VARIABLE [EOF]
    //   simple_variable = (*) T_VARIABLE ["+="]
    //   simple_variable = (*) "$" simple_variable [EOF]
    //   simple_variable = (*) "$" simple_variable ["+="]
    //   simple_variable = (*) "${" expr "}" [EOF]
    //   simple_variable = (*) "${" expr "}" ["+="]
    //   variable = (*) simple_variable [EOF]
    //   variable = (*) simple_variable ["+="]
    //
    //   "$" -> Shift(S6)
    //   "${" -> Shift(S7)
    //   r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# -> Shift(S8)
    //
    //   T_VARIABLE -> S1
    //   expr -> S2
    //   expr_without_variable -> S3
    //   simple_variable -> S4
    //   variable -> S5
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
            Some((_, (2, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym0 = &mut Some((__tok0));
                __result = try!(__state6(input, __lookbehind, __tokens, __sym0));
            }
            Some((_, (3, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym0 = &mut Some((__tok0));
                __result = try!(__state7(input, __lookbehind, __tokens, __sym0));
            }
            Some((_, (128, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym0 = &mut Some((__tok0));
                __result = try!(__state8(input, __lookbehind, __tokens, __sym0));
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
                __Nonterminal::expr(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state2(input, __lookbehind, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::expr__without__variable(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state3(input, __lookbehind, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::simple__variable(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state4(input, __lookbehind, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::variable(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state5(input, __lookbehind, __tokens, __lookahead, __sym0));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
    }

    // State 1
    //   simple_variable = T_VARIABLE (*) [EOF]
    //   simple_variable = T_VARIABLE (*) ["+="]
    //
    //   EOF -> Reduce(simple_variable = T_VARIABLE => Call(ActionFn(133));)
    //   "+=" -> Reduce(simple_variable = T_VARIABLE => Call(ActionFn(133));)
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
            None |
            Some((_, (18, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action133(input, __sym0);
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
    //   __expr = expr (*) [EOF]
    //
    //   EOF -> Reduce(__expr = expr => Call(ActionFn(2));)
    //
    pub fn __state2<
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
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action2(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::____expr(__nt)));
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
    //   expr = expr_without_variable (*) [EOF]
    //
    //   EOF -> Reduce(expr = expr_without_variable => Call(ActionFn(135));)
    //
    pub fn __state3<
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
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action135(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::expr(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 4
    //   variable = simple_variable (*) [EOF]
    //   variable = simple_variable (*) ["+="]
    //
    //   EOF -> Reduce(variable = simple_variable => Call(ActionFn(130));)
    //   "+=" -> Reduce(variable = simple_variable => Call(ActionFn(130));)
    //
    pub fn __state4<
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
            None |
            Some((_, (18, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action130(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::variable(__nt)));
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
    //   T_PLUS_EQUAL = (*) "+=" ["$"]
    //   T_PLUS_EQUAL = (*) "+=" ["${"]
    //   T_PLUS_EQUAL = (*) "+=" [r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"#]
    //   expr = variable (*) [EOF]
    //   expr_without_variable = variable (*) T_PLUS_EQUAL expr [EOF]
    //
    //   EOF -> Reduce(expr = variable => Call(ActionFn(134));)
    //   "+=" -> Shift(S10)
    //
    //   T_PLUS_EQUAL -> S9
    pub fn __state5<
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
            Some((_, (18, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state10(input, __lookbehind, __tokens, __sym1));
            }
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action134(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::expr(__nt)));
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
                __Nonterminal::T__PLUS__EQUAL(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state9(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 6
    //   T_VARIABLE = (*) r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# [EOF]
    //   T_VARIABLE = (*) r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# ["+="]
    //   simple_variable = (*) T_VARIABLE [EOF]
    //   simple_variable = (*) T_VARIABLE ["+="]
    //   simple_variable = (*) "$" simple_variable [EOF]
    //   simple_variable = (*) "$" simple_variable ["+="]
    //   simple_variable = "$" (*) simple_variable [EOF]
    //   simple_variable = "$" (*) simple_variable ["+="]
    //   simple_variable = (*) "${" expr "}" [EOF]
    //   simple_variable = (*) "${" expr "}" ["+="]
    //
    //   "$" -> Shift(S6)
    //   "${" -> Shift(S7)
    //   r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# -> Shift(S8)
    //
    //   T_VARIABLE -> S1
    //   simple_variable -> S11
    pub fn __state6<
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
            Some((_, (2, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state6(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (3, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state7(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (128, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state8(input, __lookbehind, __tokens, __sym1));
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
                    __result = try!(__state11(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 7
    //   T_VARIABLE = (*) r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# ["+="]
    //   T_VARIABLE = (*) r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# ["}"]
    //   expr = (*) expr_without_variable ["}"]
    //   expr = (*) variable ["}"]
    //   expr_without_variable = (*) variable T_PLUS_EQUAL expr ["}"]
    //   simple_variable = (*) T_VARIABLE ["+="]
    //   simple_variable = (*) T_VARIABLE ["}"]
    //   simple_variable = (*) "$" simple_variable ["+="]
    //   simple_variable = (*) "$" simple_variable ["}"]
    //   simple_variable = (*) "${" expr "}" ["+="]
    //   simple_variable = (*) "${" expr "}" ["}"]
    //   simple_variable = "${" (*) expr "}" [EOF]
    //   simple_variable = "${" (*) expr "}" ["+="]
    //   variable = (*) simple_variable ["+="]
    //   variable = (*) simple_variable ["}"]
    //
    //   "$" -> Shift(S17)
    //   "${" -> Shift(S18)
    //   r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# -> Shift(S19)
    //
    //   T_VARIABLE -> S12
    //   expr -> S13
    //   expr_without_variable -> S14
    //   simple_variable -> S15
    //   variable -> S16
    pub fn __state7<
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
            Some((_, (2, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state17(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (3, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state18(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (128, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state19(input, __lookbehind, __tokens, __sym1));
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
                    __result = try!(__state12(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::expr(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state13(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                __Nonterminal::expr__without__variable(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state14(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::simple__variable(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state15(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::variable(__nt) => {
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

    // State 8
    //   T_VARIABLE = r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# (*) [EOF]
    //   T_VARIABLE = r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# (*) ["+="]
    //
    //   EOF -> Reduce(T_VARIABLE = r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# => Call(ActionFn(5));)
    //   "+=" -> Reduce(T_VARIABLE = r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# => Call(ActionFn(5));)
    //
    pub fn __state8<
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
            None |
            Some((_, (18, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action5(input, __sym0);
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

    // State 9
    //   T_VARIABLE = (*) r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# [EOF]
    //   T_VARIABLE = (*) r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# ["+="]
    //   expr = (*) expr_without_variable [EOF]
    //   expr = (*) variable [EOF]
    //   expr_without_variable = (*) variable T_PLUS_EQUAL expr [EOF]
    //   expr_without_variable = variable T_PLUS_EQUAL (*) expr [EOF]
    //   simple_variable = (*) T_VARIABLE [EOF]
    //   simple_variable = (*) T_VARIABLE ["+="]
    //   simple_variable = (*) "$" simple_variable [EOF]
    //   simple_variable = (*) "$" simple_variable ["+="]
    //   simple_variable = (*) "${" expr "}" [EOF]
    //   simple_variable = (*) "${" expr "}" ["+="]
    //   variable = (*) simple_variable [EOF]
    //   variable = (*) simple_variable ["+="]
    //
    //   "$" -> Shift(S6)
    //   "${" -> Shift(S7)
    //   r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# -> Shift(S8)
    //
    //   T_VARIABLE -> S1
    //   expr -> S20
    //   expr_without_variable -> S3
    //   simple_variable -> S4
    //   variable -> S5
    pub fn __state9<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Box<Variable>>,
        __sym1: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (2, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state6(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (3, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state7(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (128, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state8(input, __lookbehind, __tokens, __sym2));
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
                    __result = try!(__state1(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::expr(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state20(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::expr__without__variable(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state3(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::simple__variable(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state4(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::variable(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state5(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 10
    //   T_PLUS_EQUAL = "+=" (*) ["$"]
    //   T_PLUS_EQUAL = "+=" (*) ["${"]
    //   T_PLUS_EQUAL = "+=" (*) [r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"#]
    //
    //   "$" -> Reduce(T_PLUS_EQUAL = "+=" => Call(ActionFn(17));)
    //   "${" -> Reduce(T_PLUS_EQUAL = "+=" => Call(ActionFn(17));)
    //   r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# -> Reduce(T_PLUS_EQUAL = "+=" => Call(ActionFn(17));)
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
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (128, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action17(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::T__PLUS__EQUAL(__nt)));
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
    //   simple_variable = "$" simple_variable (*) [EOF]
    //   simple_variable = "$" simple_variable (*) ["+="]
    //
    //   EOF -> Reduce(simple_variable = "$", simple_variable => Call(ActionFn(132));)
    //   "+=" -> Reduce(simple_variable = "$", simple_variable => Call(ActionFn(132));)
    //
    pub fn __state11<
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
            None |
            Some((_, (18, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __nt = super::__action132(input, __sym0, __sym1);
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
    //   simple_variable = T_VARIABLE (*) ["+="]
    //   simple_variable = T_VARIABLE (*) ["}"]
    //
    //   "+=" -> Reduce(simple_variable = T_VARIABLE => Call(ActionFn(133));)
    //   "}" -> Reduce(simple_variable = T_VARIABLE => Call(ActionFn(133));)
    //
    pub fn __state12<
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
            Some((_, (18, _), _)) |
            Some((_, (125, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action133(input, __sym0);
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
    //   simple_variable = "${" expr (*) "}" [EOF]
    //   simple_variable = "${" expr (*) "}" ["+="]
    //
    //   "}" -> Shift(S21)
    //
    pub fn __state13<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<Box<Expression>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (125, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state21(input, __lookbehind, __tokens, __sym0, __sym1, __sym2));
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

    // State 14
    //   expr = expr_without_variable (*) ["}"]
    //
    //   "}" -> Reduce(expr = expr_without_variable => Call(ActionFn(135));)
    //
    pub fn __state14<
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
            Some((_, (125, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action135(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::expr(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 15
    //   variable = simple_variable (*) ["+="]
    //   variable = simple_variable (*) ["}"]
    //
    //   "+=" -> Reduce(variable = simple_variable => Call(ActionFn(130));)
    //   "}" -> Reduce(variable = simple_variable => Call(ActionFn(130));)
    //
    pub fn __state15<
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
            Some((_, (18, _), _)) |
            Some((_, (125, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action130(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::variable(__nt)));
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
    //   T_PLUS_EQUAL = (*) "+=" ["$"]
    //   T_PLUS_EQUAL = (*) "+=" ["${"]
    //   T_PLUS_EQUAL = (*) "+=" [r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"#]
    //   expr = variable (*) ["}"]
    //   expr_without_variable = variable (*) T_PLUS_EQUAL expr ["}"]
    //
    //   "+=" -> Shift(S10)
    //   "}" -> Reduce(expr = variable => Call(ActionFn(134));)
    //
    //   T_PLUS_EQUAL -> S22
    pub fn __state16<
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
            Some((_, (18, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state10(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (125, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action134(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::expr(__nt)));
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
                __Nonterminal::T__PLUS__EQUAL(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state22(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 17
    //   T_VARIABLE = (*) r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# ["+="]
    //   T_VARIABLE = (*) r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# ["}"]
    //   simple_variable = (*) T_VARIABLE ["+="]
    //   simple_variable = (*) T_VARIABLE ["}"]
    //   simple_variable = (*) "$" simple_variable ["+="]
    //   simple_variable = (*) "$" simple_variable ["}"]
    //   simple_variable = "$" (*) simple_variable ["+="]
    //   simple_variable = "$" (*) simple_variable ["}"]
    //   simple_variable = (*) "${" expr "}" ["+="]
    //   simple_variable = (*) "${" expr "}" ["}"]
    //
    //   "$" -> Shift(S17)
    //   "${" -> Shift(S18)
    //   r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# -> Shift(S19)
    //
    //   T_VARIABLE -> S12
    //   simple_variable -> S23
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
            Some((_, (2, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state17(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (3, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state18(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (128, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state19(input, __lookbehind, __tokens, __sym1));
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
                    __result = try!(__state12(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::simple__variable(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state23(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 18
    //   T_VARIABLE = (*) r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# ["+="]
    //   T_VARIABLE = (*) r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# ["}"]
    //   expr = (*) expr_without_variable ["}"]
    //   expr = (*) variable ["}"]
    //   expr_without_variable = (*) variable T_PLUS_EQUAL expr ["}"]
    //   simple_variable = (*) T_VARIABLE ["+="]
    //   simple_variable = (*) T_VARIABLE ["}"]
    //   simple_variable = (*) "$" simple_variable ["+="]
    //   simple_variable = (*) "$" simple_variable ["}"]
    //   simple_variable = (*) "${" expr "}" ["+="]
    //   simple_variable = (*) "${" expr "}" ["}"]
    //   simple_variable = "${" (*) expr "}" ["+="]
    //   simple_variable = "${" (*) expr "}" ["}"]
    //   variable = (*) simple_variable ["+="]
    //   variable = (*) simple_variable ["}"]
    //
    //   "$" -> Shift(S17)
    //   "${" -> Shift(S18)
    //   r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# -> Shift(S19)
    //
    //   T_VARIABLE -> S12
    //   expr -> S24
    //   expr_without_variable -> S14
    //   simple_variable -> S15
    //   variable -> S16
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
            Some((_, (2, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state17(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (3, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state18(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (128, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state19(input, __lookbehind, __tokens, __sym1));
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
                    __result = try!(__state12(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::expr(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state24(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                __Nonterminal::expr__without__variable(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state14(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::simple__variable(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state15(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::variable(__nt) => {
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

    // State 19
    //   T_VARIABLE = r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# (*) ["+="]
    //   T_VARIABLE = r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# (*) ["}"]
    //
    //   "+=" -> Reduce(T_VARIABLE = r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# => Call(ActionFn(5));)
    //   "}" -> Reduce(T_VARIABLE = r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# => Call(ActionFn(5));)
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
            Some((_, (125, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action5(input, __sym0);
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

    // State 20
    //   expr_without_variable = variable T_PLUS_EQUAL expr (*) [EOF]
    //
    //   EOF -> Reduce(expr_without_variable = variable, T_PLUS_EQUAL, expr => Call(ActionFn(136));)
    //
    pub fn __state20<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Box<Variable>>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<Box<Expression>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action136(input, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::expr__without__variable(__nt)));
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
    //   simple_variable = "${" expr "}" (*) [EOF]
    //   simple_variable = "${" expr "}" (*) ["+="]
    //
    //   EOF -> Reduce(simple_variable = "${", expr, "}" => Call(ActionFn(131));)
    //   "+=" -> Reduce(simple_variable = "${", expr, "}" => Call(ActionFn(131));)
    //
    pub fn __state21<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<Box<Expression>>,
        __sym2: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None |
            Some((_, (18, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action131(input, __sym0, __sym1, __sym2);
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

    // State 22
    //   T_VARIABLE = (*) r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# ["+="]
    //   T_VARIABLE = (*) r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# ["}"]
    //   expr = (*) expr_without_variable ["}"]
    //   expr = (*) variable ["}"]
    //   expr_without_variable = (*) variable T_PLUS_EQUAL expr ["}"]
    //   expr_without_variable = variable T_PLUS_EQUAL (*) expr ["}"]
    //   simple_variable = (*) T_VARIABLE ["+="]
    //   simple_variable = (*) T_VARIABLE ["}"]
    //   simple_variable = (*) "$" simple_variable ["+="]
    //   simple_variable = (*) "$" simple_variable ["}"]
    //   simple_variable = (*) "${" expr "}" ["+="]
    //   simple_variable = (*) "${" expr "}" ["}"]
    //   variable = (*) simple_variable ["+="]
    //   variable = (*) simple_variable ["}"]
    //
    //   "$" -> Shift(S17)
    //   "${" -> Shift(S18)
    //   r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# -> Shift(S19)
    //
    //   T_VARIABLE -> S12
    //   expr -> S25
    //   expr_without_variable -> S14
    //   simple_variable -> S15
    //   variable -> S16
    pub fn __state22<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Box<Variable>>,
        __sym1: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (2, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state17(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (3, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state18(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (128, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state19(input, __lookbehind, __tokens, __sym2));
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
                    __result = try!(__state12(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::expr(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state25(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::expr__without__variable(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state14(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::simple__variable(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state15(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::variable(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state16(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 23
    //   simple_variable = "$" simple_variable (*) ["+="]
    //   simple_variable = "$" simple_variable (*) ["}"]
    //
    //   "+=" -> Reduce(simple_variable = "$", simple_variable => Call(ActionFn(132));)
    //   "}" -> Reduce(simple_variable = "$", simple_variable => Call(ActionFn(132));)
    //
    pub fn __state23<
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
            Some((_, (18, _), _)) |
            Some((_, (125, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __nt = super::__action132(input, __sym0, __sym1);
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

    // State 24
    //   simple_variable = "${" expr (*) "}" ["+="]
    //   simple_variable = "${" expr (*) "}" ["}"]
    //
    //   "}" -> Shift(S26)
    //
    pub fn __state24<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<Box<Expression>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (125, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state26(input, __lookbehind, __tokens, __sym0, __sym1, __sym2));
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

    // State 25
    //   expr_without_variable = variable T_PLUS_EQUAL expr (*) ["}"]
    //
    //   "}" -> Reduce(expr_without_variable = variable, T_PLUS_EQUAL, expr => Call(ActionFn(136));)
    //
    pub fn __state25<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Box<Variable>>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<Box<Expression>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (125, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action136(input, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::expr__without__variable(__nt)));
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
    //   simple_variable = "${" expr "}" (*) ["+="]
    //   simple_variable = "${" expr "}" (*) ["}"]
    //
    //   "+=" -> Reduce(simple_variable = "${", expr, "}" => Call(ActionFn(131));)
    //   "}" -> Reduce(simple_variable = "${", expr, "}" => Call(ActionFn(131));)
    //
    pub fn __state26<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<Box<Expression>>,
        __sym2: &mut Option<&'input str>,
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
            Some((_, (125, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action131(input, __sym0, __sym1, __sym2);
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
pub use self::__parse__expr::parse_expr;

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
        T__ABSTRACT(&'input str),
        T__AND__EQUAL(&'input str),
        T__ARRAY(&'input str),
        T__ARRAY__CAST(&'input str),
        T__AS(&'input str),
        T__BOOLEAN__AND(&'input str),
        T__BOOLEAN__OR(&'input str),
        T__BOOL__CAST(&'input str),
        T__BREAK(&'input str),
        T__CALLABLE(&'input str),
        T__CASE(&'input str),
        T__CATCH(&'input str),
        T__CLASS(&'input str),
        T__CLASS__C(&'input str),
        T__CLONE(&'input str),
        T__CLOSE__TAG(&'input str),
        T__COALESCE(&'input str),
        T__COMMENT(&'input str),
        T__CONCAT__EQUAL(&'input str),
        T__CONST(&'input str),
        T__CONTINUE(&'input str),
        T__CURLY__OPEN(&'input str),
        T__DEC(&'input str),
        T__DECLARE(&'input str),
        T__DEFAULT(&'input str),
        T__DIR(&'input str),
        T__DIV__EQUAL(&'input str),
        T__DO(&'input str),
        T__DOC__COMMENT(&'input str),
        T__DOLLAR__OPEN__CURLY__BRACES(&'input str),
        T__DOUBLE__ARROW(&'input str),
        T__DOUBLE__CAST(&'input str),
        T__ECHO(&'input str),
        T__ELLIPSIS(&'input str),
        T__ELSE(&'input str),
        T__ELSEIF(&'input str),
        T__EMPTY(&'input str),
        T__ENDDECLARE(&'input str),
        T__ENDFOR(&'input str),
        T__ENDFOREACH(&'input str),
        T__ENDIF(&'input str),
        T__ENDSWITCH(&'input str),
        T__ENDWHILE(&'input str),
        T__END__HEREDOC(&'input str),
        T__EVAL(&'input str),
        T__EXIT(&'input str),
        T__EXTENDS(&'input str),
        T__FILE(&'input str),
        T__FINAL(&'input str),
        T__FINALLY(&'input str),
        T__FOR(&'input str),
        T__FOREACH(&'input str),
        T__FUNCTION(&'input str),
        T__FUNC__C(&'input str),
        T__GLOBAL(&'input str),
        T__GOTO(&'input str),
        T__HALT__COMPILER(&'input str),
        T__IF(&'input str),
        T__IMPLEMENTS(&'input str),
        T__INC(&'input str),
        T__INCLUDE(&'input str),
        T__INCLUDE__ONCE(&'input str),
        T__INSTANCEOF(&'input str),
        T__INSTEADOF(&'input str),
        T__INTERFACE(&'input str),
        T__INT__CAST(&'input str),
        T__ISSET(&'input str),
        T__IS__EQUAL(&'input str),
        T__IS__GREATER__OR__EQUAL(&'input str),
        T__IS__IDENTICAL(&'input str),
        T__IS__NOT__EQUAL(&'input str),
        T__IS__NOT__IDENTICAL(&'input str),
        T__IS__SMALLER__OR__EQUAL(&'input str),
        T__LINE(&'input str),
        T__LIST(&'input str),
        T__LOGICAL__AND(&'input str),
        T__LOGICAL__OR(&'input str),
        T__LOGICAL__XOR(&'input str),
        T__METHOD__C(&'input str),
        T__MINUS__EQUAL(&'input str),
        T__MOD__EQUAL(&'input str),
        T__MUL__EQUAL(&'input str),
        T__NAMESPACE(&'input str),
        T__NEW(&'input str),
        T__NS__C(&'input str),
        T__NS__SEPARATOR(&'input str),
        T__NUMBER(Box<String>),
        T__OBJECT__CAST(&'input str),
        T__OBJECT__OPERATOR(&'input str),
        T__OPEN__TAG(&'input str),
        T__OPEN__TAG__WITH__ECHO(&'input str),
        T__OR__EQUAL(&'input str),
        T__PAAMAYIM__NEKUDOTAYIM(&'input str),
        T__PLUS__EQUAL(&'input str),
        T__POW(&'input str),
        T__POW__EQUAL(&'input str),
        T__PRINT(&'input str),
        T__PRIVATE(&'input str),
        T__PROTECTED(&'input str),
        T__PUBLIC(&'input str),
        T__REQUIRE(&'input str),
        T__REQUIRE__ONCE(&'input str),
        T__RETURN(&'input str),
        T__SL(&'input str),
        T__SL__EQUAL(&'input str),
        T__SPACESHIP(&'input str),
        T__SR(&'input str),
        T__SR__EQUAL(&'input str),
        T__START__HEREDOC(&'input str),
        T__STATIC(&'input str),
        T__STRING__CAST(&'input str),
        T__SWITCH(&'input str),
        T__THROW(&'input str),
        T__TRAIT(&'input str),
        T__TRAIT__C(&'input str),
        T__TRY(&'input str),
        T__UNSET(&'input str),
        T__UNSET__CAST(&'input str),
        T__USE(&'input str),
        T__VAR(&'input str),
        T__VARIABLE(Box<String>),
        T__WHILE(&'input str),
        T__WHITESPACE(&'input str),
        T__XOR__EQUAL(&'input str),
        T__YIELD(&'input str),
        T__YIELD__FROM(&'input str),
        ____expr(Box<Expression>),
        ____simple__variable(Box<Variable>),
        ____variable(Box<Variable>),
        expr(Box<Expression>),
        expr__without__variable(Box<Expression>),
        simple__variable(Box<Variable>),
        variable(Box<Variable>),
    }

    // State 0
    //   T_VARIABLE = (*) r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# [EOF]
    //   __simple_variable = (*) simple_variable [EOF]
    //   simple_variable = (*) T_VARIABLE [EOF]
    //   simple_variable = (*) "$" simple_variable [EOF]
    //   simple_variable = (*) "${" expr "}" [EOF]
    //
    //   "$" -> Shift(S3)
    //   "${" -> Shift(S4)
    //   r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# -> Shift(S5)
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
            Some((_, (2, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym0 = &mut Some((__tok0));
                __result = try!(__state3(input, __lookbehind, __tokens, __sym0));
            }
            Some((_, (3, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym0 = &mut Some((__tok0));
                __result = try!(__state4(input, __lookbehind, __tokens, __sym0));
            }
            Some((_, (128, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym0 = &mut Some((__tok0));
                __result = try!(__state5(input, __lookbehind, __tokens, __sym0));
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
    //   EOF -> Reduce(simple_variable = T_VARIABLE => Call(ActionFn(133));)
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
                let __nt = super::__action133(input, __sym0);
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
    //   simple_variable = (*) "${" expr "}" [EOF]
    //
    //   "$" -> Shift(S3)
    //   "${" -> Shift(S4)
    //   r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# -> Shift(S5)
    //
    //   T_VARIABLE -> S1
    //   simple_variable -> S6
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
            Some((_, (2, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state3(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (3, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state4(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (128, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state5(input, __lookbehind, __tokens, __sym1));
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
                    __result = try!(__state6(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 4
    //   T_VARIABLE = (*) r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# ["+="]
    //   T_VARIABLE = (*) r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# ["}"]
    //   expr = (*) expr_without_variable ["}"]
    //   expr = (*) variable ["}"]
    //   expr_without_variable = (*) variable T_PLUS_EQUAL expr ["}"]
    //   simple_variable = (*) T_VARIABLE ["+="]
    //   simple_variable = (*) T_VARIABLE ["}"]
    //   simple_variable = (*) "$" simple_variable ["+="]
    //   simple_variable = (*) "$" simple_variable ["}"]
    //   simple_variable = (*) "${" expr "}" ["+="]
    //   simple_variable = (*) "${" expr "}" ["}"]
    //   simple_variable = "${" (*) expr "}" [EOF]
    //   variable = (*) simple_variable ["+="]
    //   variable = (*) simple_variable ["}"]
    //
    //   "$" -> Shift(S12)
    //   "${" -> Shift(S13)
    //   r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# -> Shift(S14)
    //
    //   T_VARIABLE -> S7
    //   expr -> S8
    //   expr_without_variable -> S9
    //   simple_variable -> S10
    //   variable -> S11
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
            Some((_, (2, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state12(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (3, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state13(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (128, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state14(input, __lookbehind, __tokens, __sym1));
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
                __Nonterminal::expr(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state8(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                __Nonterminal::expr__without__variable(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state9(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::simple__variable(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state10(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::variable(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state11(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 5
    //   T_VARIABLE = r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# (*) [EOF]
    //
    //   EOF -> Reduce(T_VARIABLE = r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# => Call(ActionFn(5));)
    //
    pub fn __state5<
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
                let __nt = super::__action5(input, __sym0);
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

    // State 6
    //   simple_variable = "$" simple_variable (*) [EOF]
    //
    //   EOF -> Reduce(simple_variable = "$", simple_variable => Call(ActionFn(132));)
    //
    pub fn __state6<
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
                let __nt = super::__action132(input, __sym0, __sym1);
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

    // State 7
    //   simple_variable = T_VARIABLE (*) ["+="]
    //   simple_variable = T_VARIABLE (*) ["}"]
    //
    //   "+=" -> Reduce(simple_variable = T_VARIABLE => Call(ActionFn(133));)
    //   "}" -> Reduce(simple_variable = T_VARIABLE => Call(ActionFn(133));)
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
            Some((_, (18, _), _)) |
            Some((_, (125, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action133(input, __sym0);
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
    //   simple_variable = "${" expr (*) "}" [EOF]
    //
    //   "}" -> Shift(S15)
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
        __sym1: &mut Option<Box<Expression>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (125, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state15(input, __lookbehind, __tokens, __sym0, __sym1, __sym2));
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
    //   expr = expr_without_variable (*) ["}"]
    //
    //   "}" -> Reduce(expr = expr_without_variable => Call(ActionFn(135));)
    //
    pub fn __state9<
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
            Some((_, (125, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action135(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::expr(__nt)));
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
    //   variable = simple_variable (*) ["+="]
    //   variable = simple_variable (*) ["}"]
    //
    //   "+=" -> Reduce(variable = simple_variable => Call(ActionFn(130));)
    //   "}" -> Reduce(variable = simple_variable => Call(ActionFn(130));)
    //
    pub fn __state10<
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
            Some((_, (18, _), _)) |
            Some((_, (125, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action130(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::variable(__nt)));
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
    //   T_PLUS_EQUAL = (*) "+=" ["$"]
    //   T_PLUS_EQUAL = (*) "+=" ["${"]
    //   T_PLUS_EQUAL = (*) "+=" [r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"#]
    //   expr = variable (*) ["}"]
    //   expr_without_variable = variable (*) T_PLUS_EQUAL expr ["}"]
    //
    //   "+=" -> Shift(S17)
    //   "}" -> Reduce(expr = variable => Call(ActionFn(134));)
    //
    //   T_PLUS_EQUAL -> S16
    pub fn __state11<
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
            Some((_, (18, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state17(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (125, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action134(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::expr(__nt)));
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
                __Nonterminal::T__PLUS__EQUAL(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state16(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 12
    //   T_VARIABLE = (*) r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# ["+="]
    //   T_VARIABLE = (*) r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# ["}"]
    //   simple_variable = (*) T_VARIABLE ["+="]
    //   simple_variable = (*) T_VARIABLE ["}"]
    //   simple_variable = (*) "$" simple_variable ["+="]
    //   simple_variable = (*) "$" simple_variable ["}"]
    //   simple_variable = "$" (*) simple_variable ["+="]
    //   simple_variable = "$" (*) simple_variable ["}"]
    //   simple_variable = (*) "${" expr "}" ["+="]
    //   simple_variable = (*) "${" expr "}" ["}"]
    //
    //   "$" -> Shift(S12)
    //   "${" -> Shift(S13)
    //   r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# -> Shift(S14)
    //
    //   T_VARIABLE -> S7
    //   simple_variable -> S18
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
            Some((_, (2, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state12(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (3, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state13(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (128, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state14(input, __lookbehind, __tokens, __sym1));
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
                    __result = try!(__state18(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 13
    //   T_VARIABLE = (*) r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# ["+="]
    //   T_VARIABLE = (*) r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# ["}"]
    //   expr = (*) expr_without_variable ["}"]
    //   expr = (*) variable ["}"]
    //   expr_without_variable = (*) variable T_PLUS_EQUAL expr ["}"]
    //   simple_variable = (*) T_VARIABLE ["+="]
    //   simple_variable = (*) T_VARIABLE ["}"]
    //   simple_variable = (*) "$" simple_variable ["+="]
    //   simple_variable = (*) "$" simple_variable ["}"]
    //   simple_variable = (*) "${" expr "}" ["+="]
    //   simple_variable = (*) "${" expr "}" ["}"]
    //   simple_variable = "${" (*) expr "}" ["+="]
    //   simple_variable = "${" (*) expr "}" ["}"]
    //   variable = (*) simple_variable ["+="]
    //   variable = (*) simple_variable ["}"]
    //
    //   "$" -> Shift(S12)
    //   "${" -> Shift(S13)
    //   r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# -> Shift(S14)
    //
    //   T_VARIABLE -> S7
    //   expr -> S19
    //   expr_without_variable -> S9
    //   simple_variable -> S10
    //   variable -> S11
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
            Some((_, (2, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state12(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (3, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state13(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (128, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state14(input, __lookbehind, __tokens, __sym1));
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
                __Nonterminal::expr(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state19(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                __Nonterminal::expr__without__variable(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state9(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::simple__variable(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state10(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::variable(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state11(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 14
    //   T_VARIABLE = r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# (*) ["+="]
    //   T_VARIABLE = r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# (*) ["}"]
    //
    //   "+=" -> Reduce(T_VARIABLE = r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# => Call(ActionFn(5));)
    //   "}" -> Reduce(T_VARIABLE = r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# => Call(ActionFn(5));)
    //
    pub fn __state14<
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
            Some((_, (125, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action5(input, __sym0);
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

    // State 15
    //   simple_variable = "${" expr "}" (*) [EOF]
    //
    //   EOF -> Reduce(simple_variable = "${", expr, "}" => Call(ActionFn(131));)
    //
    pub fn __state15<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<Box<Expression>>,
        __sym2: &mut Option<&'input str>,
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
                let __nt = super::__action131(input, __sym0, __sym1, __sym2);
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

    // State 16
    //   T_VARIABLE = (*) r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# ["+="]
    //   T_VARIABLE = (*) r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# ["}"]
    //   expr = (*) expr_without_variable ["}"]
    //   expr = (*) variable ["}"]
    //   expr_without_variable = (*) variable T_PLUS_EQUAL expr ["}"]
    //   expr_without_variable = variable T_PLUS_EQUAL (*) expr ["}"]
    //   simple_variable = (*) T_VARIABLE ["+="]
    //   simple_variable = (*) T_VARIABLE ["}"]
    //   simple_variable = (*) "$" simple_variable ["+="]
    //   simple_variable = (*) "$" simple_variable ["}"]
    //   simple_variable = (*) "${" expr "}" ["+="]
    //   simple_variable = (*) "${" expr "}" ["}"]
    //   variable = (*) simple_variable ["+="]
    //   variable = (*) simple_variable ["}"]
    //
    //   "$" -> Shift(S12)
    //   "${" -> Shift(S13)
    //   r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# -> Shift(S14)
    //
    //   T_VARIABLE -> S7
    //   expr -> S20
    //   expr_without_variable -> S9
    //   simple_variable -> S10
    //   variable -> S11
    pub fn __state16<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Box<Variable>>,
        __sym1: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (2, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state12(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (3, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state13(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (128, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state14(input, __lookbehind, __tokens, __sym2));
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
                __Nonterminal::expr(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state20(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::expr__without__variable(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state9(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::simple__variable(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state10(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::variable(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state11(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 17
    //   T_PLUS_EQUAL = "+=" (*) ["$"]
    //   T_PLUS_EQUAL = "+=" (*) ["${"]
    //   T_PLUS_EQUAL = "+=" (*) [r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"#]
    //
    //   "$" -> Reduce(T_PLUS_EQUAL = "+=" => Call(ActionFn(17));)
    //   "${" -> Reduce(T_PLUS_EQUAL = "+=" => Call(ActionFn(17));)
    //   r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# -> Reduce(T_PLUS_EQUAL = "+=" => Call(ActionFn(17));)
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
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (128, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action17(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::T__PLUS__EQUAL(__nt)));
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
    //   simple_variable = "$" simple_variable (*) ["+="]
    //   simple_variable = "$" simple_variable (*) ["}"]
    //
    //   "+=" -> Reduce(simple_variable = "$", simple_variable => Call(ActionFn(132));)
    //   "}" -> Reduce(simple_variable = "$", simple_variable => Call(ActionFn(132));)
    //
    pub fn __state18<
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
            Some((_, (18, _), _)) |
            Some((_, (125, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __nt = super::__action132(input, __sym0, __sym1);
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

    // State 19
    //   simple_variable = "${" expr (*) "}" ["+="]
    //   simple_variable = "${" expr (*) "}" ["}"]
    //
    //   "}" -> Shift(S21)
    //
    pub fn __state19<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<Box<Expression>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (125, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state21(input, __lookbehind, __tokens, __sym0, __sym1, __sym2));
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

    // State 20
    //   expr_without_variable = variable T_PLUS_EQUAL expr (*) ["}"]
    //
    //   "}" -> Reduce(expr_without_variable = variable, T_PLUS_EQUAL, expr => Call(ActionFn(136));)
    //
    pub fn __state20<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Box<Variable>>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<Box<Expression>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (125, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action136(input, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::expr__without__variable(__nt)));
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
    //   simple_variable = "${" expr "}" (*) ["+="]
    //   simple_variable = "${" expr "}" (*) ["}"]
    //
    //   "+=" -> Reduce(simple_variable = "${", expr, "}" => Call(ActionFn(131));)
    //   "}" -> Reduce(simple_variable = "${", expr, "}" => Call(ActionFn(131));)
    //
    pub fn __state21<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<Box<Expression>>,
        __sym2: &mut Option<&'input str>,
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
            Some((_, (125, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action131(input, __sym0, __sym1, __sym2);
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

mod __parse__variable {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use std::str::FromStr;
    use ast::{If, Expression, Block, Variable};
    extern crate lalrpop_util as __lalrpop_util;
    use self::__lalrpop_util::ParseError as __ParseError;
    pub fn parse_variable<
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
            (_, None, __Nonterminal::____variable(__nt)) => {
                Ok(__nt)
            }
            _ => unreachable!(),
        }
    }

    #[allow(dead_code)]
    pub enum __Nonterminal<'input> {
        T__ABSTRACT(&'input str),
        T__AND__EQUAL(&'input str),
        T__ARRAY(&'input str),
        T__ARRAY__CAST(&'input str),
        T__AS(&'input str),
        T__BOOLEAN__AND(&'input str),
        T__BOOLEAN__OR(&'input str),
        T__BOOL__CAST(&'input str),
        T__BREAK(&'input str),
        T__CALLABLE(&'input str),
        T__CASE(&'input str),
        T__CATCH(&'input str),
        T__CLASS(&'input str),
        T__CLASS__C(&'input str),
        T__CLONE(&'input str),
        T__CLOSE__TAG(&'input str),
        T__COALESCE(&'input str),
        T__COMMENT(&'input str),
        T__CONCAT__EQUAL(&'input str),
        T__CONST(&'input str),
        T__CONTINUE(&'input str),
        T__CURLY__OPEN(&'input str),
        T__DEC(&'input str),
        T__DECLARE(&'input str),
        T__DEFAULT(&'input str),
        T__DIR(&'input str),
        T__DIV__EQUAL(&'input str),
        T__DO(&'input str),
        T__DOC__COMMENT(&'input str),
        T__DOLLAR__OPEN__CURLY__BRACES(&'input str),
        T__DOUBLE__ARROW(&'input str),
        T__DOUBLE__CAST(&'input str),
        T__ECHO(&'input str),
        T__ELLIPSIS(&'input str),
        T__ELSE(&'input str),
        T__ELSEIF(&'input str),
        T__EMPTY(&'input str),
        T__ENDDECLARE(&'input str),
        T__ENDFOR(&'input str),
        T__ENDFOREACH(&'input str),
        T__ENDIF(&'input str),
        T__ENDSWITCH(&'input str),
        T__ENDWHILE(&'input str),
        T__END__HEREDOC(&'input str),
        T__EVAL(&'input str),
        T__EXIT(&'input str),
        T__EXTENDS(&'input str),
        T__FILE(&'input str),
        T__FINAL(&'input str),
        T__FINALLY(&'input str),
        T__FOR(&'input str),
        T__FOREACH(&'input str),
        T__FUNCTION(&'input str),
        T__FUNC__C(&'input str),
        T__GLOBAL(&'input str),
        T__GOTO(&'input str),
        T__HALT__COMPILER(&'input str),
        T__IF(&'input str),
        T__IMPLEMENTS(&'input str),
        T__INC(&'input str),
        T__INCLUDE(&'input str),
        T__INCLUDE__ONCE(&'input str),
        T__INSTANCEOF(&'input str),
        T__INSTEADOF(&'input str),
        T__INTERFACE(&'input str),
        T__INT__CAST(&'input str),
        T__ISSET(&'input str),
        T__IS__EQUAL(&'input str),
        T__IS__GREATER__OR__EQUAL(&'input str),
        T__IS__IDENTICAL(&'input str),
        T__IS__NOT__EQUAL(&'input str),
        T__IS__NOT__IDENTICAL(&'input str),
        T__IS__SMALLER__OR__EQUAL(&'input str),
        T__LINE(&'input str),
        T__LIST(&'input str),
        T__LOGICAL__AND(&'input str),
        T__LOGICAL__OR(&'input str),
        T__LOGICAL__XOR(&'input str),
        T__METHOD__C(&'input str),
        T__MINUS__EQUAL(&'input str),
        T__MOD__EQUAL(&'input str),
        T__MUL__EQUAL(&'input str),
        T__NAMESPACE(&'input str),
        T__NEW(&'input str),
        T__NS__C(&'input str),
        T__NS__SEPARATOR(&'input str),
        T__NUMBER(Box<String>),
        T__OBJECT__CAST(&'input str),
        T__OBJECT__OPERATOR(&'input str),
        T__OPEN__TAG(&'input str),
        T__OPEN__TAG__WITH__ECHO(&'input str),
        T__OR__EQUAL(&'input str),
        T__PAAMAYIM__NEKUDOTAYIM(&'input str),
        T__PLUS__EQUAL(&'input str),
        T__POW(&'input str),
        T__POW__EQUAL(&'input str),
        T__PRINT(&'input str),
        T__PRIVATE(&'input str),
        T__PROTECTED(&'input str),
        T__PUBLIC(&'input str),
        T__REQUIRE(&'input str),
        T__REQUIRE__ONCE(&'input str),
        T__RETURN(&'input str),
        T__SL(&'input str),
        T__SL__EQUAL(&'input str),
        T__SPACESHIP(&'input str),
        T__SR(&'input str),
        T__SR__EQUAL(&'input str),
        T__START__HEREDOC(&'input str),
        T__STATIC(&'input str),
        T__STRING__CAST(&'input str),
        T__SWITCH(&'input str),
        T__THROW(&'input str),
        T__TRAIT(&'input str),
        T__TRAIT__C(&'input str),
        T__TRY(&'input str),
        T__UNSET(&'input str),
        T__UNSET__CAST(&'input str),
        T__USE(&'input str),
        T__VAR(&'input str),
        T__VARIABLE(Box<String>),
        T__WHILE(&'input str),
        T__WHITESPACE(&'input str),
        T__XOR__EQUAL(&'input str),
        T__YIELD(&'input str),
        T__YIELD__FROM(&'input str),
        ____expr(Box<Expression>),
        ____simple__variable(Box<Variable>),
        ____variable(Box<Variable>),
        expr(Box<Expression>),
        expr__without__variable(Box<Expression>),
        simple__variable(Box<Variable>),
        variable(Box<Variable>),
    }

    // State 0
    //   T_VARIABLE = (*) r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# [EOF]
    //   __variable = (*) variable [EOF]
    //   simple_variable = (*) T_VARIABLE [EOF]
    //   simple_variable = (*) "$" simple_variable [EOF]
    //   simple_variable = (*) "${" expr "}" [EOF]
    //   variable = (*) simple_variable [EOF]
    //
    //   "$" -> Shift(S4)
    //   "${" -> Shift(S5)
    //   r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# -> Shift(S6)
    //
    //   T_VARIABLE -> S1
    //   simple_variable -> S2
    //   variable -> S3
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
            Some((_, (2, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym0 = &mut Some((__tok0));
                __result = try!(__state4(input, __lookbehind, __tokens, __sym0));
            }
            Some((_, (3, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym0 = &mut Some((__tok0));
                __result = try!(__state5(input, __lookbehind, __tokens, __sym0));
            }
            Some((_, (128, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym0 = &mut Some((__tok0));
                __result = try!(__state6(input, __lookbehind, __tokens, __sym0));
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
                __Nonterminal::variable(__nt) => {
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
    //   simple_variable = T_VARIABLE (*) [EOF]
    //
    //   EOF -> Reduce(simple_variable = T_VARIABLE => Call(ActionFn(133));)
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
                let __nt = super::__action133(input, __sym0);
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
    //   variable = simple_variable (*) [EOF]
    //
    //   EOF -> Reduce(variable = simple_variable => Call(ActionFn(130));)
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
                let __nt = super::__action130(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::variable(__nt)));
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
    //   __variable = variable (*) [EOF]
    //
    //   EOF -> Reduce(__variable = variable => Call(ActionFn(0));)
    //
    pub fn __state3<
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
                let __nt = super::__action0(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::____variable(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 4
    //   T_VARIABLE = (*) r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# [EOF]
    //   simple_variable = (*) T_VARIABLE [EOF]
    //   simple_variable = (*) "$" simple_variable [EOF]
    //   simple_variable = "$" (*) simple_variable [EOF]
    //   simple_variable = (*) "${" expr "}" [EOF]
    //
    //   "$" -> Shift(S4)
    //   "${" -> Shift(S5)
    //   r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# -> Shift(S6)
    //
    //   T_VARIABLE -> S1
    //   simple_variable -> S7
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
            Some((_, (2, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state4(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (3, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state5(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (128, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state6(input, __lookbehind, __tokens, __sym1));
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
                    __result = try!(__state7(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 5
    //   T_VARIABLE = (*) r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# ["+="]
    //   T_VARIABLE = (*) r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# ["}"]
    //   expr = (*) expr_without_variable ["}"]
    //   expr = (*) variable ["}"]
    //   expr_without_variable = (*) variable T_PLUS_EQUAL expr ["}"]
    //   simple_variable = (*) T_VARIABLE ["+="]
    //   simple_variable = (*) T_VARIABLE ["}"]
    //   simple_variable = (*) "$" simple_variable ["+="]
    //   simple_variable = (*) "$" simple_variable ["}"]
    //   simple_variable = (*) "${" expr "}" ["+="]
    //   simple_variable = (*) "${" expr "}" ["}"]
    //   simple_variable = "${" (*) expr "}" [EOF]
    //   variable = (*) simple_variable ["+="]
    //   variable = (*) simple_variable ["}"]
    //
    //   "$" -> Shift(S13)
    //   "${" -> Shift(S14)
    //   r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# -> Shift(S15)
    //
    //   T_VARIABLE -> S8
    //   expr -> S9
    //   expr_without_variable -> S10
    //   simple_variable -> S11
    //   variable -> S12
    pub fn __state5<
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
            Some((_, (2, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state13(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (3, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state14(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (128, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state15(input, __lookbehind, __tokens, __sym1));
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
                    __result = try!(__state8(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::expr(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state9(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                __Nonterminal::expr__without__variable(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state10(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::simple__variable(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state11(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::variable(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state12(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 6
    //   T_VARIABLE = r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# (*) [EOF]
    //
    //   EOF -> Reduce(T_VARIABLE = r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# => Call(ActionFn(5));)
    //
    pub fn __state6<
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
                let __nt = super::__action5(input, __sym0);
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

    // State 7
    //   simple_variable = "$" simple_variable (*) [EOF]
    //
    //   EOF -> Reduce(simple_variable = "$", simple_variable => Call(ActionFn(132));)
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
        __sym1: &mut Option<Box<Variable>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __nt = super::__action132(input, __sym0, __sym1);
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
    //   simple_variable = T_VARIABLE (*) ["+="]
    //   simple_variable = T_VARIABLE (*) ["}"]
    //
    //   "+=" -> Reduce(simple_variable = T_VARIABLE => Call(ActionFn(133));)
    //   "}" -> Reduce(simple_variable = T_VARIABLE => Call(ActionFn(133));)
    //
    pub fn __state8<
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
            Some((_, (18, _), _)) |
            Some((_, (125, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action133(input, __sym0);
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

    // State 9
    //   simple_variable = "${" expr (*) "}" [EOF]
    //
    //   "}" -> Shift(S16)
    //
    pub fn __state9<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<Box<Expression>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (125, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state16(input, __lookbehind, __tokens, __sym0, __sym1, __sym2));
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

    // State 10
    //   expr = expr_without_variable (*) ["}"]
    //
    //   "}" -> Reduce(expr = expr_without_variable => Call(ActionFn(135));)
    //
    pub fn __state10<
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
            Some((_, (125, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action135(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::expr(__nt)));
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
    //   variable = simple_variable (*) ["+="]
    //   variable = simple_variable (*) ["}"]
    //
    //   "+=" -> Reduce(variable = simple_variable => Call(ActionFn(130));)
    //   "}" -> Reduce(variable = simple_variable => Call(ActionFn(130));)
    //
    pub fn __state11<
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
            Some((_, (18, _), _)) |
            Some((_, (125, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action130(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::variable(__nt)));
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
    //   T_PLUS_EQUAL = (*) "+=" ["$"]
    //   T_PLUS_EQUAL = (*) "+=" ["${"]
    //   T_PLUS_EQUAL = (*) "+=" [r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"#]
    //   expr = variable (*) ["}"]
    //   expr_without_variable = variable (*) T_PLUS_EQUAL expr ["}"]
    //
    //   "+=" -> Shift(S18)
    //   "}" -> Reduce(expr = variable => Call(ActionFn(134));)
    //
    //   T_PLUS_EQUAL -> S17
    pub fn __state12<
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
            Some((_, (18, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state18(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (125, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action134(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::expr(__nt)));
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
                __Nonterminal::T__PLUS__EQUAL(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state17(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 13
    //   T_VARIABLE = (*) r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# ["+="]
    //   T_VARIABLE = (*) r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# ["}"]
    //   simple_variable = (*) T_VARIABLE ["+="]
    //   simple_variable = (*) T_VARIABLE ["}"]
    //   simple_variable = (*) "$" simple_variable ["+="]
    //   simple_variable = (*) "$" simple_variable ["}"]
    //   simple_variable = "$" (*) simple_variable ["+="]
    //   simple_variable = "$" (*) simple_variable ["}"]
    //   simple_variable = (*) "${" expr "}" ["+="]
    //   simple_variable = (*) "${" expr "}" ["}"]
    //
    //   "$" -> Shift(S13)
    //   "${" -> Shift(S14)
    //   r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# -> Shift(S15)
    //
    //   T_VARIABLE -> S8
    //   simple_variable -> S19
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
            Some((_, (2, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state13(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (3, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state14(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (128, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state15(input, __lookbehind, __tokens, __sym1));
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
                    __result = try!(__state8(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::simple__variable(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state19(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 14
    //   T_VARIABLE = (*) r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# ["+="]
    //   T_VARIABLE = (*) r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# ["}"]
    //   expr = (*) expr_without_variable ["}"]
    //   expr = (*) variable ["}"]
    //   expr_without_variable = (*) variable T_PLUS_EQUAL expr ["}"]
    //   simple_variable = (*) T_VARIABLE ["+="]
    //   simple_variable = (*) T_VARIABLE ["}"]
    //   simple_variable = (*) "$" simple_variable ["+="]
    //   simple_variable = (*) "$" simple_variable ["}"]
    //   simple_variable = (*) "${" expr "}" ["+="]
    //   simple_variable = (*) "${" expr "}" ["}"]
    //   simple_variable = "${" (*) expr "}" ["+="]
    //   simple_variable = "${" (*) expr "}" ["}"]
    //   variable = (*) simple_variable ["+="]
    //   variable = (*) simple_variable ["}"]
    //
    //   "$" -> Shift(S13)
    //   "${" -> Shift(S14)
    //   r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# -> Shift(S15)
    //
    //   T_VARIABLE -> S8
    //   expr -> S20
    //   expr_without_variable -> S10
    //   simple_variable -> S11
    //   variable -> S12
    pub fn __state14<
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
            Some((_, (2, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state13(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (3, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state14(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (128, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state15(input, __lookbehind, __tokens, __sym1));
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
                    __result = try!(__state8(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::expr(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state20(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                __Nonterminal::expr__without__variable(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state10(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::simple__variable(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state11(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::variable(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state12(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 15
    //   T_VARIABLE = r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# (*) ["+="]
    //   T_VARIABLE = r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# (*) ["}"]
    //
    //   "+=" -> Reduce(T_VARIABLE = r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# => Call(ActionFn(5));)
    //   "}" -> Reduce(T_VARIABLE = r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# => Call(ActionFn(5));)
    //
    pub fn __state15<
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
            Some((_, (125, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action5(input, __sym0);
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

    // State 16
    //   simple_variable = "${" expr "}" (*) [EOF]
    //
    //   EOF -> Reduce(simple_variable = "${", expr, "}" => Call(ActionFn(131));)
    //
    pub fn __state16<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<Box<Expression>>,
        __sym2: &mut Option<&'input str>,
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
                let __nt = super::__action131(input, __sym0, __sym1, __sym2);
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

    // State 17
    //   T_VARIABLE = (*) r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# ["+="]
    //   T_VARIABLE = (*) r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# ["}"]
    //   expr = (*) expr_without_variable ["}"]
    //   expr = (*) variable ["}"]
    //   expr_without_variable = (*) variable T_PLUS_EQUAL expr ["}"]
    //   expr_without_variable = variable T_PLUS_EQUAL (*) expr ["}"]
    //   simple_variable = (*) T_VARIABLE ["+="]
    //   simple_variable = (*) T_VARIABLE ["}"]
    //   simple_variable = (*) "$" simple_variable ["+="]
    //   simple_variable = (*) "$" simple_variable ["}"]
    //   simple_variable = (*) "${" expr "}" ["+="]
    //   simple_variable = (*) "${" expr "}" ["}"]
    //   variable = (*) simple_variable ["+="]
    //   variable = (*) simple_variable ["}"]
    //
    //   "$" -> Shift(S13)
    //   "${" -> Shift(S14)
    //   r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# -> Shift(S15)
    //
    //   T_VARIABLE -> S8
    //   expr -> S21
    //   expr_without_variable -> S10
    //   simple_variable -> S11
    //   variable -> S12
    pub fn __state17<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Box<Variable>>,
        __sym1: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (2, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state13(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (3, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state14(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (128, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state15(input, __lookbehind, __tokens, __sym2));
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
                    __result = try!(__state8(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::expr(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state21(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::expr__without__variable(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state10(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::simple__variable(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state11(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::variable(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state12(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 18
    //   T_PLUS_EQUAL = "+=" (*) ["$"]
    //   T_PLUS_EQUAL = "+=" (*) ["${"]
    //   T_PLUS_EQUAL = "+=" (*) [r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"#]
    //
    //   "$" -> Reduce(T_PLUS_EQUAL = "+=" => Call(ActionFn(17));)
    //   "${" -> Reduce(T_PLUS_EQUAL = "+=" => Call(ActionFn(17));)
    //   r#"\\$([a-zA-Z\\_]+)([0-9a-zA-Z\\_]*)"# -> Reduce(T_PLUS_EQUAL = "+=" => Call(ActionFn(17));)
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
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (128, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action17(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::T__PLUS__EQUAL(__nt)));
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
    //   simple_variable = "$" simple_variable (*) ["+="]
    //   simple_variable = "$" simple_variable (*) ["}"]
    //
    //   "+=" -> Reduce(simple_variable = "$", simple_variable => Call(ActionFn(132));)
    //   "}" -> Reduce(simple_variable = "$", simple_variable => Call(ActionFn(132));)
    //
    pub fn __state19<
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
            Some((_, (18, _), _)) |
            Some((_, (125, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __nt = super::__action132(input, __sym0, __sym1);
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

    // State 20
    //   simple_variable = "${" expr (*) "}" ["+="]
    //   simple_variable = "${" expr (*) "}" ["}"]
    //
    //   "}" -> Shift(S22)
    //
    pub fn __state20<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<Box<Expression>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (125, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state22(input, __lookbehind, __tokens, __sym0, __sym1, __sym2));
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

    // State 21
    //   expr_without_variable = variable T_PLUS_EQUAL expr (*) ["}"]
    //
    //   "}" -> Reduce(expr_without_variable = variable, T_PLUS_EQUAL, expr => Call(ActionFn(136));)
    //
    pub fn __state21<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Box<Variable>>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<Box<Expression>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (125, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action136(input, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::expr__without__variable(__nt)));
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
    //   simple_variable = "${" expr "}" (*) ["+="]
    //   simple_variable = "${" expr "}" (*) ["}"]
    //
    //   "+=" -> Reduce(simple_variable = "${", expr, "}" => Call(ActionFn(131));)
    //   "}" -> Reduce(simple_variable = "${", expr, "}" => Call(ActionFn(131));)
    //
    pub fn __state22<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<Box<Expression>>,
        __sym2: &mut Option<&'input str>,
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
            Some((_, (125, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action131(input, __sym0, __sym1, __sym2);
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
pub use self::__parse__variable::parse_variable;
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
                        '!' => {
                            __current_state = 1;
                            continue;
                        }
                        '$' => {
                            __current_match = Some((2, __index + 1));
                            __current_state = 2;
                            continue;
                        }
                        '%' => {
                            __current_state = 3;
                            continue;
                        }
                        '&' => {
                            __current_state = 4;
                            continue;
                        }
                        '(' => {
                            __current_state = 5;
                            continue;
                        }
                        '*' => {
                            __current_state = 6;
                            continue;
                        }
                        '+' => {
                            __current_state = 7;
                            continue;
                        }
                        '-' => {
                            __current_state = 8;
                            continue;
                        }
                        '.' => {
                            __current_state = 9;
                            continue;
                        }
                        '/' => {
                            __current_state = 10;
                            continue;
                        }
                        '0' => {
                            __current_match = Some((126, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((126, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((126, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((126, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((126, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((126, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((126, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((126, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((126, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((126, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        ':' => {
                            __current_state = 12;
                            continue;
                        }
                        '<' => {
                            __current_state = 13;
                            continue;
                        }
                        '=' => {
                            __current_state = 14;
                            continue;
                        }
                        '>' => {
                            __current_state = 15;
                            continue;
                        }
                        '?' => {
                            __current_state = 16;
                            continue;
                        }
                        '\\' => {
                            __current_state = 17;
                            continue;
                        }
                        '^' => {
                            __current_state = 18;
                            continue;
                        }
                        '_' => {
                            __current_state = 19;
                            continue;
                        }
                        'a' => {
                            __current_state = 20;
                            continue;
                        }
                        'b' => {
                            __current_state = 21;
                            continue;
                        }
                        'c' => {
                            __current_state = 22;
                            continue;
                        }
                        'd' => {
                            __current_state = 23;
                            continue;
                        }
                        'e' => {
                            __current_state = 24;
                            continue;
                        }
                        'f' => {
                            __current_state = 25;
                            continue;
                        }
                        'g' => {
                            __current_state = 26;
                            continue;
                        }
                        'h' => {
                            __current_state = 27;
                            continue;
                        }
                        'i' => {
                            __current_state = 28;
                            continue;
                        }
                        'l' => {
                            __current_state = 29;
                            continue;
                        }
                        'n' => {
                            __current_state = 30;
                            continue;
                        }
                        'o' => {
                            __current_state = 31;
                            continue;
                        }
                        'p' => {
                            __current_state = 32;
                            continue;
                        }
                        'r' => {
                            __current_state = 33;
                            continue;
                        }
                        's' => {
                            __current_state = 34;
                            continue;
                        }
                        't' => {
                            __current_state = 35;
                            continue;
                        }
                        'u' => {
                            __current_state = 36;
                            continue;
                        }
                        'v' => {
                            __current_state = 37;
                            continue;
                        }
                        'w' => {
                            __current_state = 38;
                            continue;
                        }
                        'x' => {
                            __current_state = 39;
                            continue;
                        }
                        'y' => {
                            __current_state = 40;
                            continue;
                        }
                        '{' => {
                            __current_state = 41;
                            continue;
                        }
                        '|' => {
                            __current_state = 42;
                            continue;
                        }
                        '}' => {
                            __current_match = Some((125, __index + 1));
                            __current_state = 43;
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
                        '=' => {
                            __current_match = Some((0, __index + 1));
                            __current_state = 45;
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
                        'A' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        'B' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        'C' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        'D' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        'E' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        'F' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        'G' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        'H' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        'I' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        'J' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        'K' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        'L' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        'M' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        'N' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        'O' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        'P' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        'Q' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        'R' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        'S' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        'T' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        'U' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        'V' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        'W' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        'X' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        'Y' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        'Z' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        '\\' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        '_' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        's' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        't' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        '{' => {
                            __current_match = Some((3, __index + 1));
                            __current_state = 47;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                3 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '=' => {
                            __current_match = Some((4, __index + 1));
                            __current_state = 48;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                4 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '&' => {
                            __current_match = Some((5, __index + 1));
                            __current_state = 49;
                            continue;
                        }
                        '=' => {
                            __current_match = Some((6, __index + 1));
                            __current_state = 50;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                5 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'a' => {
                            __current_state = 51;
                            continue;
                        }
                        'b' => {
                            __current_state = 52;
                            continue;
                        }
                        'd' => {
                            __current_state = 53;
                            continue;
                        }
                        'i' => {
                            __current_state = 54;
                            continue;
                        }
                        'o' => {
                            __current_state = 55;
                            continue;
                        }
                        's' => {
                            __current_state = 56;
                            continue;
                        }
                        'u' => {
                            __current_state = 57;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                6 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '*' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 58;
                            continue;
                        }
                        '=' => {
                            __current_match = Some((16, __index + 1));
                            __current_state = 59;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                7 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '+' => {
                            __current_match = Some((17, __index + 1));
                            __current_state = 60;
                            continue;
                        }
                        '=' => {
                            __current_match = Some((18, __index + 1));
                            __current_state = 61;
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
                        '-' => {
                            __current_match = Some((19, __index + 1));
                            __current_state = 62;
                            continue;
                        }
                        '=' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 63;
                            continue;
                        }
                        '>' => {
                            __current_match = Some((21, __index + 1));
                            __current_state = 64;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                9 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '.' => {
                            __current_state = 65;
                            continue;
                        }
                        '=' => {
                            __current_match = Some((23, __index + 1));
                            __current_state = 66;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                10 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '=' => {
                            __current_match = Some((24, __index + 1));
                            __current_state = 67;
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
                        '.' => {
                            __current_state = 68;
                            continue;
                        }
                        '0' => {
                            __current_match = Some((126, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((126, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((126, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((126, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((126, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((126, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((126, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((126, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((126, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((126, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                12 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        ':' => {
                            __current_match = Some((25, __index + 1));
                            __current_state = 69;
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
                        '<' => {
                            __current_match = Some((26, __index + 1));
                            __current_state = 70;
                            continue;
                        }
                        '=' => {
                            __current_match = Some((28, __index + 1));
                            __current_state = 71;
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
                        '=' => {
                            __current_match = Some((30, __index + 1));
                            __current_state = 72;
                            continue;
                        }
                        '>' => {
                            __current_match = Some((32, __index + 1));
                            __current_state = 73;
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
                        '=' => {
                            __current_match = Some((33, __index + 1));
                            __current_state = 74;
                            continue;
                        }
                        '>' => {
                            __current_match = Some((34, __index + 1));
                            __current_state = 75;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                16 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '?' => {
                            __current_match = Some((36, __index + 1));
                            __current_state = 76;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                17 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '\\' => {
                            __current_match = Some((37, __index + 1));
                            __current_state = 77;
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
                        '=' => {
                            __current_match = Some((38, __index + 1));
                            __current_state = 78;
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
                        '_' => {
                            __current_state = 79;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                20 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'b' => {
                            __current_state = 80;
                            continue;
                        }
                        'n' => {
                            __current_state = 81;
                            continue;
                        }
                        'r' => {
                            __current_state = 82;
                            continue;
                        }
                        's' => {
                            __current_match = Some((51, __index + 1));
                            __current_state = 83;
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
                        'r' => {
                            __current_state = 84;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                22 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'a' => {
                            __current_state = 85;
                            continue;
                        }
                        'l' => {
                            __current_state = 86;
                            continue;
                        }
                        'o' => {
                            __current_state = 87;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                23 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'e' => {
                            __current_state = 88;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((64, __index + 1));
                            __current_state = 89;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                24 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'c' => {
                            __current_state = 90;
                            continue;
                        }
                        'l' => {
                            __current_state = 91;
                            continue;
                        }
                        'm' => {
                            __current_state = 92;
                            continue;
                        }
                        'n' => {
                            __current_state = 93;
                            continue;
                        }
                        'v' => {
                            __current_state = 94;
                            continue;
                        }
                        'x' => {
                            __current_state = 95;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                25 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'i' => {
                            __current_state = 96;
                            continue;
                        }
                        'o' => {
                            __current_state = 97;
                            continue;
                        }
                        'u' => {
                            __current_state = 98;
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
                        'l' => {
                            __current_state = 99;
                            continue;
                        }
                        'o' => {
                            __current_state = 100;
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
                        'e' => {
                            __current_state = 101;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                28 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'f' => {
                            __current_match = Some((88, __index + 1));
                            __current_state = 102;
                            continue;
                        }
                        'm' => {
                            __current_state = 103;
                            continue;
                        }
                        'n' => {
                            __current_state = 104;
                            continue;
                        }
                        's' => {
                            __current_state = 105;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                29 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'i' => {
                            __current_state = 106;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                30 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'a' => {
                            __current_state = 107;
                            continue;
                        }
                        'e' => {
                            __current_state = 108;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                31 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'p' => {
                            __current_state = 109;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((101, __index + 1));
                            __current_state = 110;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                32 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'r' => {
                            __current_state = 111;
                            continue;
                        }
                        'u' => {
                            __current_state = 112;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                33 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'e' => {
                            __current_state = 113;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                34 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        't' => {
                            __current_state = 114;
                            continue;
                        }
                        'w' => {
                            __current_state = 115;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                35 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'h' => {
                            __current_state = 116;
                            continue;
                        }
                        'r' => {
                            __current_state = 117;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                36 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'n' => {
                            __current_state = 118;
                            continue;
                        }
                        's' => {
                            __current_state = 119;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                37 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'a' => {
                            __current_state = 120;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                38 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'h' => {
                            __current_state = 121;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                39 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'o' => {
                            __current_state = 122;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                40 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'i' => {
                            __current_state = 123;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                41 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '$' => {
                            __current_match = Some((122, __index + 1));
                            __current_state = 124;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                42 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '=' => {
                            __current_match = Some((123, __index + 1));
                            __current_state = 125;
                            continue;
                        }
                        '|' => {
                            __current_match = Some((124, __index + 1));
                            __current_state = 126;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                43 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                44 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                45 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '=' => {
                            __current_match = Some((1, __index + 1));
                            __current_state = 127;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                46 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '0' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 128;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 128;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 128;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 128;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 128;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 128;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 128;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 128;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 128;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 128;
                            continue;
                        }
                        'A' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        'B' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        'C' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        'D' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        'E' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        'F' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        'G' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        'H' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        'I' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        'J' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        'K' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        'L' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        'M' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        'N' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        'O' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        'P' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        'Q' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        'R' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        'S' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        'T' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        'U' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        'V' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        'W' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        'X' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        'Y' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        'Z' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        '\\' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        '_' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        's' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        't' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                47 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                48 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                49 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                50 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                51 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'r' => {
                            __current_state = 129;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                52 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'o' => {
                            __current_state = 130;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                53 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'o' => {
                            __current_state = 131;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                54 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'n' => {
                            __current_state = 132;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                55 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'b' => {
                            __current_state = 133;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                56 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        't' => {
                            __current_state = 134;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                57 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'n' => {
                            __current_state = 135;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                58 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '=' => {
                            __current_match = Some((15, __index + 1));
                            __current_state = 136;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                59 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                60 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                61 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                62 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                63 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                64 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                65 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '.' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 137;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                66 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                67 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                68 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '0' => {
                            __current_match = Some((127, __index + 1));
                            __current_state = 138;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((127, __index + 1));
                            __current_state = 138;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((127, __index + 1));
                            __current_state = 138;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((127, __index + 1));
                            __current_state = 138;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((127, __index + 1));
                            __current_state = 138;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((127, __index + 1));
                            __current_state = 138;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((127, __index + 1));
                            __current_state = 138;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((127, __index + 1));
                            __current_state = 138;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((127, __index + 1));
                            __current_state = 138;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((127, __index + 1));
                            __current_state = 138;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                69 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                70 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '=' => {
                            __current_match = Some((27, __index + 1));
                            __current_state = 139;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                71 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '>' => {
                            __current_match = Some((29, __index + 1));
                            __current_state = 140;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                72 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '=' => {
                            __current_match = Some((31, __index + 1));
                            __current_state = 141;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                73 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                74 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                75 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '=' => {
                            __current_match = Some((35, __index + 1));
                            __current_state = 142;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                76 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                77 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                78 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                79 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'C' => {
                            __current_state = 143;
                            continue;
                        }
                        'D' => {
                            __current_state = 144;
                            continue;
                        }
                        'F' => {
                            __current_state = 145;
                            continue;
                        }
                        'L' => {
                            __current_state = 146;
                            continue;
                        }
                        'M' => {
                            __current_state = 147;
                            continue;
                        }
                        'N' => {
                            __current_state = 148;
                            continue;
                        }
                        'T' => {
                            __current_state = 149;
                            continue;
                        }
                        'h' => {
                            __current_state = 150;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                80 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        's' => {
                            __current_state = 151;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                81 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'd' => {
                            __current_match = Some((49, __index + 1));
                            __current_state = 152;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                82 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'r' => {
                            __current_state = 153;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                83 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                84 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'e' => {
                            __current_state = 154;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                85 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'l' => {
                            __current_state = 155;
                            continue;
                        }
                        's' => {
                            __current_state = 156;
                            continue;
                        }
                        't' => {
                            __current_state = 157;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                86 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'a' => {
                            __current_state = 158;
                            continue;
                        }
                        'o' => {
                            __current_state = 159;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                87 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'm' => {
                            __current_state = 160;
                            continue;
                        }
                        'n' => {
                            __current_state = 161;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                88 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'c' => {
                            __current_state = 162;
                            continue;
                        }
                        'f' => {
                            __current_state = 163;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                89 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'c' => {
                            __current_state = 164;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                90 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'h' => {
                            __current_state = 165;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                91 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        's' => {
                            __current_state = 166;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                92 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'p' => {
                            __current_state = 167;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                93 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'd' => {
                            __current_state = 168;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                94 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'a' => {
                            __current_state = 169;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                95 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'i' => {
                            __current_state = 170;
                            continue;
                        }
                        't' => {
                            __current_state = 171;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                96 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'n' => {
                            __current_state = 172;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                97 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'r' => {
                            __current_match = Some((81, __index + 1));
                            __current_state = 173;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                98 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'n' => {
                            __current_state = 174;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                99 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'o' => {
                            __current_state = 175;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                100 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        't' => {
                            __current_state = 176;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                101 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'r' => {
                            __current_state = 177;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                102 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                103 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'p' => {
                            __current_state = 178;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                104 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'c' => {
                            __current_state = 179;
                            continue;
                        }
                        's' => {
                            __current_state = 180;
                            continue;
                        }
                        't' => {
                            __current_state = 181;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                105 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        's' => {
                            __current_state = 182;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                106 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        's' => {
                            __current_state = 183;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                107 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'm' => {
                            __current_state = 184;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                108 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'w' => {
                            __current_state = 185;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                109 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'e' => {
                            __current_state = 186;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                110 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                111 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'i' => {
                            __current_state = 187;
                            continue;
                        }
                        'o' => {
                            __current_state = 188;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                112 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'b' => {
                            __current_state = 189;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                113 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'q' => {
                            __current_state = 190;
                            continue;
                        }
                        't' => {
                            __current_state = 191;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                114 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'a' => {
                            __current_state = 192;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                115 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'i' => {
                            __current_state = 193;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                116 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'r' => {
                            __current_state = 194;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                117 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'a' => {
                            __current_state = 195;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((113, __index + 1));
                            __current_state = 196;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                118 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        's' => {
                            __current_state = 197;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                119 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'e' => {
                            __current_match = Some((115, __index + 1));
                            __current_state = 198;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                120 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'r' => {
                            __current_match = Some((116, __index + 1));
                            __current_state = 199;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                121 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'i' => {
                            __current_state = 200;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                122 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'r' => {
                            __current_match = Some((119, __index + 1));
                            __current_state = 201;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                123 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'e' => {
                            __current_state = 202;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                124 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                125 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                126 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                127 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                128 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '0' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 128;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 128;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 128;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 128;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 128;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 128;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 128;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 128;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 128;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 128;
                            continue;
                        }
                        'A' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 128;
                            continue;
                        }
                        'B' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 128;
                            continue;
                        }
                        'C' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 128;
                            continue;
                        }
                        'D' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 128;
                            continue;
                        }
                        'E' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 128;
                            continue;
                        }
                        'F' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 128;
                            continue;
                        }
                        'G' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 128;
                            continue;
                        }
                        'H' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 128;
                            continue;
                        }
                        'I' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 128;
                            continue;
                        }
                        'J' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 128;
                            continue;
                        }
                        'K' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 128;
                            continue;
                        }
                        'L' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 128;
                            continue;
                        }
                        'M' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 128;
                            continue;
                        }
                        'N' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 128;
                            continue;
                        }
                        'O' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 128;
                            continue;
                        }
                        'P' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 128;
                            continue;
                        }
                        'Q' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 128;
                            continue;
                        }
                        'R' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 128;
                            continue;
                        }
                        'S' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 128;
                            continue;
                        }
                        'T' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 128;
                            continue;
                        }
                        'U' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 128;
                            continue;
                        }
                        'V' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 128;
                            continue;
                        }
                        'W' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 128;
                            continue;
                        }
                        'X' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 128;
                            continue;
                        }
                        'Y' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 128;
                            continue;
                        }
                        'Z' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 128;
                            continue;
                        }
                        '\\' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 128;
                            continue;
                        }
                        '_' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 128;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 128;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 128;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 128;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 128;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 128;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 128;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 128;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 128;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 128;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 128;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 128;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 128;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 128;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 128;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 128;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 128;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 128;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 128;
                            continue;
                        }
                        's' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 128;
                            continue;
                        }
                        't' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 128;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 128;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 128;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 128;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 128;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 128;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((128, __index + 1));
                            __current_state = 128;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                129 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'r' => {
                            __current_state = 203;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                130 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'o' => {
                            __current_state = 204;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                131 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'u' => {
                            __current_state = 205;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                132 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        't' => {
                            __current_state = 206;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                133 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'j' => {
                            __current_state = 207;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                134 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'r' => {
                            __current_state = 208;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                135 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        's' => {
                            __current_state = 209;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                136 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                137 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                138 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '0' => {
                            __current_match = Some((127, __index + 1));
                            __current_state = 138;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((127, __index + 1));
                            __current_state = 138;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((127, __index + 1));
                            __current_state = 138;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((127, __index + 1));
                            __current_state = 138;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((127, __index + 1));
                            __current_state = 138;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((127, __index + 1));
                            __current_state = 138;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((127, __index + 1));
                            __current_state = 138;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((127, __index + 1));
                            __current_state = 138;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((127, __index + 1));
                            __current_state = 138;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((127, __index + 1));
                            __current_state = 138;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                139 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                140 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                141 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                142 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                143 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'L' => {
                            __current_state = 210;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                144 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'I' => {
                            __current_state = 211;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                145 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'I' => {
                            __current_state = 212;
                            continue;
                        }
                        'U' => {
                            __current_state = 213;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                146 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'I' => {
                            __current_state = 214;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                147 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'E' => {
                            __current_state = 215;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                148 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'A' => {
                            __current_state = 216;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                149 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'R' => {
                            __current_state = 217;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                150 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'a' => {
                            __current_state = 218;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                151 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        't' => {
                            __current_state = 219;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                152 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                153 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'a' => {
                            __current_state = 220;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                154 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'a' => {
                            __current_state = 221;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                155 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'l' => {
                            __current_state = 222;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                156 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'e' => {
                            __current_match = Some((54, __index + 1));
                            __current_state = 223;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                157 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'c' => {
                            __current_state = 224;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                158 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        's' => {
                            __current_state = 225;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                159 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'n' => {
                            __current_state = 226;
                            continue;
                        }
                        's' => {
                            __current_state = 227;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                160 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'm' => {
                            __current_state = 228;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                161 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        's' => {
                            __current_match = Some((60, __index + 1));
                            __current_state = 229;
                            continue;
                        }
                        't' => {
                            __current_state = 230;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                162 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'l' => {
                            __current_state = 231;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                163 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'a' => {
                            __current_state = 232;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                164 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        ' ' => {
                            __current_state = 233;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                165 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'o' => {
                            __current_match = Some((66, __index + 1));
                            __current_state = 234;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                166 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'e' => {
                            __current_match = Some((67, __index + 1));
                            __current_state = 235;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                167 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        't' => {
                            __current_state = 236;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                168 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'd' => {
                            __current_state = 237;
                            continue;
                        }
                        'f' => {
                            __current_state = 238;
                            continue;
                        }
                        'i' => {
                            __current_state = 239;
                            continue;
                        }
                        's' => {
                            __current_state = 240;
                            continue;
                        }
                        'w' => {
                            __current_state = 241;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                169 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'l' => {
                            __current_match = Some((76, __index + 1));
                            __current_state = 242;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                170 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        't' => {
                            __current_match = Some((77, __index + 1));
                            __current_state = 243;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                171 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'e' => {
                            __current_state = 244;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                172 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'a' => {
                            __current_state = 245;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                173 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'e' => {
                            __current_state = 246;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                174 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'c' => {
                            __current_state = 247;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                175 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'b' => {
                            __current_state = 248;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                176 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'o' => {
                            __current_match = Some((85, __index + 1));
                            __current_state = 249;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                177 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'e' => {
                            __current_state = 250;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                178 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'l' => {
                            __current_state = 251;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                179 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'l' => {
                            __current_state = 252;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                180 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        't' => {
                            __current_state = 253;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                181 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'e' => {
                            __current_state = 254;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                182 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'e' => {
                            __current_state = 255;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                183 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        't' => {
                            __current_match = Some((96, __index + 1));
                            __current_state = 256;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                184 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'e' => {
                            __current_state = 257;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                185 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        ' ' => {
                            __current_state = 258;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                186 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'n' => {
                            __current_state = 259;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                187 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'n' => {
                            __current_state = 260;
                            continue;
                        }
                        'v' => {
                            __current_state = 261;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                188 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        't' => {
                            __current_state = 262;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                189 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'l' => {
                            __current_state = 263;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                190 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'u' => {
                            __current_state = 264;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                191 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'u' => {
                            __current_state = 265;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                192 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        't' => {
                            __current_state = 266;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                193 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        't' => {
                            __current_state = 267;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                194 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'o' => {
                            __current_state = 268;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                195 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'i' => {
                            __current_state = 269;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                196 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                197 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'e' => {
                            __current_state = 270;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                198 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                199 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                200 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'l' => {
                            __current_state = 271;
                            continue;
                        }
                        't' => {
                            __current_state = 272;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                201 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                202 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'l' => {
                            __current_state = 273;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                203 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'a' => {
                            __current_state = 274;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                204 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'l' => {
                            __current_state = 275;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                205 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'b' => {
                            __current_state = 276;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                206 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        ')' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 277;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                207 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'e' => {
                            __current_state = 278;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                208 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'i' => {
                            __current_state = 279;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                209 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'e' => {
                            __current_state = 280;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                210 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'A' => {
                            __current_state = 281;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                211 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'R' => {
                            __current_state = 282;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                212 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'L' => {
                            __current_state = 283;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                213 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'N' => {
                            __current_state = 284;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                214 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'N' => {
                            __current_state = 285;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                215 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'T' => {
                            __current_state = 286;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                216 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'M' => {
                            __current_state = 287;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                217 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'A' => {
                            __current_state = 288;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                218 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'l' => {
                            __current_state = 289;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                219 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'r' => {
                            __current_state = 290;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                220 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'y' => {
                            __current_match = Some((50, __index + 1));
                            __current_state = 291;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                221 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'k' => {
                            __current_match = Some((52, __index + 1));
                            __current_state = 292;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                222 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'a' => {
                            __current_state = 293;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                223 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                224 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'h' => {
                            __current_match = Some((55, __index + 1));
                            __current_state = 294;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                225 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        's' => {
                            __current_match = Some((56, __index + 1));
                            __current_state = 295;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                226 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'e' => {
                            __current_match = Some((57, __index + 1));
                            __current_state = 296;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                227 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'e' => {
                            __current_state = 297;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                228 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'e' => {
                            __current_state = 298;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                229 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                230 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'i' => {
                            __current_state = 299;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                231 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'a' => {
                            __current_state = 300;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                232 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'u' => {
                            __current_state = 301;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                233 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'c' => {
                            __current_state = 302;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                234 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                235 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'i' => {
                            __current_state = 303;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                236 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'y' => {
                            __current_match = Some((69, __index + 1));
                            __current_state = 304;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                237 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'e' => {
                            __current_state = 305;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                238 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'o' => {
                            __current_state = 306;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                239 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'f' => {
                            __current_match = Some((73, __index + 1));
                            __current_state = 307;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                240 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'w' => {
                            __current_state = 308;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                241 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'h' => {
                            __current_state = 309;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                242 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                243 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                244 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'n' => {
                            __current_state = 310;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                245 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'l' => {
                            __current_match = Some((79, __index + 1));
                            __current_state = 311;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                246 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'a' => {
                            __current_state = 312;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                247 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        't' => {
                            __current_state = 313;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                248 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'a' => {
                            __current_state = 314;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                249 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                250 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'd' => {
                            __current_state = 315;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                251 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'e' => {
                            __current_state = 316;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                252 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'u' => {
                            __current_state = 317;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                253 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'a' => {
                            __current_state = 318;
                            continue;
                        }
                        'e' => {
                            __current_state = 319;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                254 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'r' => {
                            __current_state = 320;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                255 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        't' => {
                            __current_match = Some((95, __index + 1));
                            __current_state = 321;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                256 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                257 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        's' => {
                            __current_state = 322;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                258 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '(' => {
                            __current_state = 323;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                259 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        ' ' => {
                            __current_state = 324;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                260 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        't' => {
                            __current_match = Some((102, __index + 1));
                            __current_state = 325;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                261 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'a' => {
                            __current_state = 326;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                262 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'e' => {
                            __current_state = 327;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                263 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'i' => {
                            __current_state = 328;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                264 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'i' => {
                            __current_state = 329;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                265 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'r' => {
                            __current_state = 330;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                266 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'i' => {
                            __current_state = 331;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                267 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'c' => {
                            __current_state = 332;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                268 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'w' => {
                            __current_match = Some((111, __index + 1));
                            __current_state = 333;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                269 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        't' => {
                            __current_match = Some((112, __index + 1));
                            __current_state = 334;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                270 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        't' => {
                            __current_match = Some((114, __index + 1));
                            __current_state = 335;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                271 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'e' => {
                            __current_match = Some((117, __index + 1));
                            __current_state = 336;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                272 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'e' => {
                            __current_state = 337;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                273 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'd' => {
                            __current_match = Some((120, __index + 1));
                            __current_state = 338;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                274 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'y' => {
                            __current_state = 339;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                275 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        ')' => {
                            __current_match = Some((8, __index + 1));
                            __current_state = 340;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                276 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'l' => {
                            __current_state = 341;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                277 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                278 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'c' => {
                            __current_state = 342;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                279 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'n' => {
                            __current_state = 343;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                280 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        't' => {
                            __current_state = 344;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                281 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'S' => {
                            __current_state = 345;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                282 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '_' => {
                            __current_state = 346;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                283 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'E' => {
                            __current_state = 347;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                284 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'C' => {
                            __current_state = 348;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                285 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'E' => {
                            __current_state = 349;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                286 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'H' => {
                            __current_state = 350;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                287 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'E' => {
                            __current_state = 351;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                288 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'I' => {
                            __current_state = 352;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                289 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        't' => {
                            __current_state = 353;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                290 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'a' => {
                            __current_state = 354;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                291 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                292 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                293 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'b' => {
                            __current_state = 355;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                294 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                295 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                296 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                297 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        ' ' => {
                            __current_state = 356;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                298 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'n' => {
                            __current_state = 357;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                299 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'n' => {
                            __current_state = 358;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                300 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'r' => {
                            __current_state = 359;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                301 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'l' => {
                            __current_state = 360;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                302 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'o' => {
                            __current_state = 361;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                303 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'f' => {
                            __current_match = Some((68, __index + 1));
                            __current_state = 362;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                304 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                305 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'c' => {
                            __current_state = 363;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                306 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'r' => {
                            __current_match = Some((71, __index + 1));
                            __current_state = 364;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                307 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                308 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'i' => {
                            __current_state = 365;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                309 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'i' => {
                            __current_state = 366;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                310 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'd' => {
                            __current_state = 367;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                311 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'l' => {
                            __current_state = 368;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                312 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'c' => {
                            __current_state = 369;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                313 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'i' => {
                            __current_state = 370;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                314 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'l' => {
                            __current_match = Some((84, __index + 1));
                            __current_state = 371;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                315 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'o' => {
                            __current_state = 372;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                316 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'm' => {
                            __current_state = 373;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                317 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'd' => {
                            __current_state = 374;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                318 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'n' => {
                            __current_state = 375;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                319 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'a' => {
                            __current_state = 376;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                320 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'f' => {
                            __current_state = 377;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                321 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                322 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'p' => {
                            __current_state = 378;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                323 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'T' => {
                            __current_state = 379;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                324 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        't' => {
                            __current_state = 380;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                325 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                326 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        't' => {
                            __current_state = 381;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                327 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'c' => {
                            __current_state = 382;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                328 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'c' => {
                            __current_match = Some((105, __index + 1));
                            __current_state = 383;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                329 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'r' => {
                            __current_state = 384;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                330 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'n' => {
                            __current_match = Some((108, __index + 1));
                            __current_state = 385;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                331 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'c' => {
                            __current_match = Some((109, __index + 1));
                            __current_state = 386;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                332 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'h' => {
                            __current_match = Some((110, __index + 1));
                            __current_state = 387;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                333 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                334 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                335 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                336 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                337 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        's' => {
                            __current_state = 388;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                338 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        ' ' => {
                            __current_state = 389;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                339 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        ')' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 390;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                340 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                341 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'e' => {
                            __current_state = 391;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                342 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        't' => {
                            __current_state = 392;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                343 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'g' => {
                            __current_state = 393;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                344 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        ')' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 394;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                345 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'S' => {
                            __current_state = 395;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                346 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '_' => {
                            __current_match = Some((40, __index + 1));
                            __current_state = 396;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                347 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '_' => {
                            __current_state = 397;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                348 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'T' => {
                            __current_state = 398;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                349 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '_' => {
                            __current_state = 399;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                350 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'O' => {
                            __current_state = 400;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                351 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'S' => {
                            __current_state = 401;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                352 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'T' => {
                            __current_state = 402;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                353 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '_' => {
                            __current_state = 403;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                354 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'c' => {
                            __current_state = 404;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                355 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'l' => {
                            __current_state = 405;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                356 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        't' => {
                            __current_state = 406;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                357 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        't' => {
                            __current_match = Some((59, __index + 1));
                            __current_state = 407;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                358 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'u' => {
                            __current_state = 408;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                359 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'e' => {
                            __current_match = Some((62, __index + 1));
                            __current_state = 409;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                360 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        't' => {
                            __current_match = Some((63, __index + 1));
                            __current_state = 410;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                361 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'm' => {
                            __current_state = 411;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                362 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                363 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'l' => {
                            __current_state = 412;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                364 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'e' => {
                            __current_state = 413;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                365 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        't' => {
                            __current_state = 414;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                366 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'l' => {
                            __current_state = 415;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                367 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        's' => {
                            __current_match = Some((78, __index + 1));
                            __current_state = 416;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                368 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'y' => {
                            __current_match = Some((80, __index + 1));
                            __current_state = 417;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                369 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'h' => {
                            __current_match = Some((82, __index + 1));
                            __current_state = 418;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                370 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'o' => {
                            __current_state = 419;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                371 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                372 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'c' => {
                            __current_state = 420;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                373 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'e' => {
                            __current_state = 421;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                374 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'e' => {
                            __current_match = Some((90, __index + 1));
                            __current_state = 422;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                375 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'c' => {
                            __current_state = 423;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                376 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'd' => {
                            __current_state = 424;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                377 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'a' => {
                            __current_state = 425;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                378 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'a' => {
                            __current_state = 426;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                379 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '_' => {
                            __current_state = 427;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                380 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'a' => {
                            __current_state = 428;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                381 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'e' => {
                            __current_match = Some((103, __index + 1));
                            __current_state = 429;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                382 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        't' => {
                            __current_state = 430;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                383 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                384 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'e' => {
                            __current_match = Some((106, __index + 1));
                            __current_state = 431;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                385 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                386 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                387 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                388 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'p' => {
                            __current_state = 432;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                389 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'f' => {
                            __current_state = 433;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                390 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                391 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        ')' => {
                            __current_match = Some((9, __index + 1));
                            __current_state = 434;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                392 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        ')' => {
                            __current_match = Some((11, __index + 1));
                            __current_state = 435;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                393 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        ')' => {
                            __current_match = Some((12, __index + 1));
                            __current_state = 436;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                394 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                395 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '_' => {
                            __current_state = 437;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                396 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                397 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '_' => {
                            __current_match = Some((41, __index + 1));
                            __current_state = 438;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                398 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'I' => {
                            __current_state = 439;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                399 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '_' => {
                            __current_match = Some((43, __index + 1));
                            __current_state = 440;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                400 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'D' => {
                            __current_state = 441;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                401 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'P' => {
                            __current_state = 442;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                402 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '_' => {
                            __current_state = 443;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                403 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'c' => {
                            __current_state = 444;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                404 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        't' => {
                            __current_match = Some((48, __index + 1));
                            __current_state = 445;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                405 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'e' => {
                            __current_match = Some((53, __index + 1));
                            __current_state = 446;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                406 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'a' => {
                            __current_state = 447;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                407 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                408 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'e' => {
                            __current_match = Some((61, __index + 1));
                            __current_state = 448;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                409 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                410 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                411 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'm' => {
                            __current_state = 449;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                412 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'a' => {
                            __current_state = 450;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                413 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'a' => {
                            __current_state = 451;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                414 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'c' => {
                            __current_state = 452;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                415 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'e' => {
                            __current_match = Some((75, __index + 1));
                            __current_state = 453;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                416 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                417 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                418 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                419 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'n' => {
                            __current_match = Some((83, __index + 1));
                            __current_state = 454;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                420 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        ' ' => {
                            __current_state = 455;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                421 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'n' => {
                            __current_state = 456;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                422 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '_' => {
                            __current_state = 457;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                423 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'e' => {
                            __current_state = 458;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                424 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'o' => {
                            __current_state = 459;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                425 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'c' => {
                            __current_state = 460;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                426 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'c' => {
                            __current_state = 461;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                427 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'N' => {
                            __current_state = 462;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                428 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'g' => {
                            __current_match = Some((99, __index + 1));
                            __current_state = 463;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                429 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                430 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'e' => {
                            __current_state = 464;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                431 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '_' => {
                            __current_state = 465;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                432 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'a' => {
                            __current_state = 466;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                433 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'r' => {
                            __current_state = 467;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                434 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                435 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                436 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                437 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '_' => {
                            __current_match = Some((39, __index + 1));
                            __current_state = 468;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                438 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                439 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'O' => {
                            __current_state = 469;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                440 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                441 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '_' => {
                            __current_state = 470;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                442 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'A' => {
                            __current_state = 471;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                443 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '_' => {
                            __current_match = Some((46, __index + 1));
                            __current_state = 472;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                444 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'o' => {
                            __current_state = 473;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                445 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                446 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                447 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'g' => {
                            __current_match = Some((58, __index + 1));
                            __current_state = 474;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                448 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                449 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'e' => {
                            __current_state = 475;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                450 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'r' => {
                            __current_state = 476;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                451 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'c' => {
                            __current_state = 477;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                452 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'h' => {
                            __current_match = Some((74, __index + 1));
                            __current_state = 478;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                453 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                454 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                455 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'e' => {
                            __current_state = 479;
                            continue;
                        }
                        's' => {
                            __current_state = 480;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                456 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        't' => {
                            __current_state = 481;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                457 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'o' => {
                            __current_state = 482;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                458 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'o' => {
                            __current_state = 483;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                459 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'f' => {
                            __current_match = Some((93, __index + 1));
                            __current_state = 484;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                460 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'e' => {
                            __current_match = Some((94, __index + 1));
                            __current_state = 485;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                461 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'e' => {
                            __current_match = Some((97, __index + 1));
                            __current_state = 486;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                462 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'E' => {
                            __current_state = 487;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                463 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        ' ' => {
                            __current_state = 488;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                464 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'd' => {
                            __current_match = Some((104, __index + 1));
                            __current_state = 489;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                465 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'o' => {
                            __current_state = 490;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                466 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'c' => {
                            __current_state = 491;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                467 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'o' => {
                            __current_state = 492;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                468 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                469 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'N' => {
                            __current_state = 493;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                470 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '_' => {
                            __current_match = Some((44, __index + 1));
                            __current_state = 494;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                471 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'C' => {
                            __current_state = 495;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                472 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                473 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'm' => {
                            __current_state = 496;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                474 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                475 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'n' => {
                            __current_state = 497;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                476 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'e' => {
                            __current_match = Some((70, __index + 1));
                            __current_state = 498;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                477 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'h' => {
                            __current_match = Some((72, __index + 1));
                            __current_state = 499;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                478 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                479 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'n' => {
                            __current_state = 500;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                480 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        't' => {
                            __current_state = 501;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                481 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        's' => {
                            __current_match = Some((89, __index + 1));
                            __current_state = 502;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                482 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'n' => {
                            __current_state = 503;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                483 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'f' => {
                            __current_match = Some((92, __index + 1));
                            __current_state = 504;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                484 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                485 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                486 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                487 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'W' => {
                            __current_state = 505;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                488 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'w' => {
                            __current_state = 506;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                489 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                490 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'n' => {
                            __current_state = 507;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                491 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'e' => {
                            __current_match = Some((118, __index + 1));
                            __current_state = 508;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                492 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'm' => {
                            __current_match = Some((121, __index + 1));
                            __current_state = 509;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                493 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '_' => {
                            __current_state = 510;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                494 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                495 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'E' => {
                            __current_state = 511;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                496 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'p' => {
                            __current_state = 512;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                497 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        't' => {
                            __current_match = Some((65, __index + 1));
                            __current_state = 513;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                498 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                499 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                500 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'd' => {
                            __current_match = Some((86, __index + 1));
                            __current_state = 514;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                501 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'a' => {
                            __current_state = 515;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                502 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                503 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'c' => {
                            __current_state = 516;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                504 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                505 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        ')' => {
                            __current_match = Some((98, __index + 1));
                            __current_state = 517;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                506 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'i' => {
                            __current_state = 518;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                507 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'c' => {
                            __current_state = 519;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                508 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                509 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                510 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '_' => {
                            __current_match = Some((42, __index + 1));
                            __current_state = 520;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                511 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '_' => {
                            __current_state = 521;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                512 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'i' => {
                            __current_state = 522;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                513 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                514 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                515 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'r' => {
                            __current_state = 523;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                516 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'e' => {
                            __current_match = Some((91, __index + 1));
                            __current_state = 524;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                517 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                518 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        't' => {
                            __current_state = 525;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                519 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'e' => {
                            __current_match = Some((107, __index + 1));
                            __current_state = 526;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                520 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                521 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '_' => {
                            __current_match = Some((45, __index + 1));
                            __current_state = 527;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                522 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'l' => {
                            __current_state = 528;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                523 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        't' => {
                            __current_state = 529;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                524 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                525 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'h' => {
                            __current_state = 530;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                526 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                527 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                528 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'e' => {
                            __current_state = 531;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                529 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        ' ' => {
                            __current_match = Some((87, __index + 1));
                            __current_state = 532;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                530 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        ' ' => {
                            __current_state = 533;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                531 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'r' => {
                            __current_match = Some((47, __index + 1));
                            __current_state = 534;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                532 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                533 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'e' => {
                            __current_state = 535;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                534 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                535 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'c' => {
                            __current_state = 536;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                536 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'h' => {
                            __current_state = 537;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                537 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'o' => {
                            __current_match = Some((100, __index + 1));
                            __current_state = 538;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                538 => {
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
    __0: Box<Variable>,
) -> Box<Variable>
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
    __0: Box<Expression>,
) -> Box<Expression>
{
    (__0)
}

pub fn __action3<
    'input,
>(
    input: &'input str,
    s: &'input str,
) -> Box<String>
{
    Box::new(s.to_string())
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
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action7<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action8<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action9<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action10<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action11<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action12<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action13<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action14<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action15<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action16<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action17<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
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

pub fn __action30<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action31<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action32<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action33<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action34<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action35<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action36<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action37<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action38<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action39<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action40<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action41<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action42<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action43<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action44<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action45<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action46<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action47<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action48<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action49<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action50<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action51<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action52<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action53<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action54<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action55<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action56<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action57<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action58<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action59<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action60<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action61<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action62<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action63<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action64<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action65<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action66<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action67<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action68<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action69<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action70<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action71<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action72<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action73<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action74<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action75<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action76<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action77<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action78<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action79<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action80<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action81<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action82<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action83<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action84<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action85<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action86<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action87<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action88<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action89<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action90<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action91<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action92<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action93<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action94<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action95<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action96<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action97<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action98<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action99<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action100<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action101<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action102<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action103<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action104<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action105<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action106<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action107<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action108<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action109<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action110<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action111<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action112<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action113<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action114<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action115<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action116<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action117<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action118<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action119<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action120<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action121<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action122<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action123<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action124<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action125<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action126<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action127<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action128<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action129<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action130<
    'input,
>(
    input: &'input str,
    __0: Box<Variable>,
) -> Box<Variable>
{
    (__0)
}

pub fn __action131<
    'input,
>(
    input: &'input str,
    a: &'input str,
    expr: Box<Expression>,
    c: &'input str,
) -> Box<Variable>
{
    { Box::new(Variable::Expression(expr)) }
}

pub fn __action132<
    'input,
>(
    input: &'input str,
    a: &'input str,
    var: Box<Variable>,
) -> Box<Variable>
{
    { Box::new(Variable::Variable(var)) }
}

pub fn __action133<
    'input,
>(
    input: &'input str,
    var: Box<String>,
) -> Box<Variable>
{
    { Box::new(Variable::Identifier(var)) }
}

pub fn __action134<
    'input,
>(
    input: &'input str,
    var: Box<Variable>,
) -> Box<Expression>
{
    { Box::new(Expression::Variable(var)) }
}

pub fn __action135<
    'input,
>(
    input: &'input str,
    expr: Box<Expression>,
) -> Box<Expression>
{
    { expr }
}

pub fn __action136<
    'input,
>(
    input: &'input str,
    v1: Box<Variable>,
    a: &'input str,
    expr: Box<Expression>,
) -> Box<Expression>
{
    { Box::new(Expression::AssignOp { op: Box::new("+=".to_string()), expr_left: Box::new(Expression::Variable(v1)), expr_right: Box::new(Expression::Expression(expr)) }) }
//	|	variable T_MINUS_EQUAL expr
//	|	variable T_MUL_EQUAL expr
//	|	variable T_POW_EQUAL expr
//	|	variable T_DIV_EQUAL expr
//	|	variable T_CONCAT_EQUAL expr
//	|	variable T_MOD_EQUAL expr
//	|	variable T_AND_EQUAL expr
//	|	variable T_OR_EQUAL expr
//	|	variable T_XOR_EQUAL expr
//	|	variable T_SL_EQUAL expr
//	|	variable T_SR_EQUAL expr
//	|	variable T_INC { $$ = zend_ast_create(ZEND_AST_POST_INC, $1); }
//	|	T_INC variable { $$ = zend_ast_create(ZEND_AST_PRE_INC, $2); }
//	|	variable T_DEC { $$ = zend_ast_create(ZEND_AST_POST_DEC, $1); }
//	|	T_DEC variable { $$ = zend_ast_create(ZEND_AST_PRE_DEC, $2); }
//	|	expr T_BOOLEAN_OR expr
//	|	expr T_BOOLEAN_AND expr
//	|	expr T_LOGICAL_OR expr
//	|	expr T_LOGICAL_AND expr
//	|	expr T_LOGICAL_XOR expr
//	|	expr '|' expr	{ $$ = zend_ast_create_binary_op(ZEND_BW_OR, $1, $3); }
//	|	expr '&' expr	{ $$ = zend_ast_create_binary_op(ZEND_BW_AND, $1, $3); }
//	|	expr '^' expr	{ $$ = zend_ast_create_binary_op(ZEND_BW_XOR, $1, $3); }
//	|	expr '.' expr 	{ $$ = zend_ast_create_binary_op(ZEND_CONCAT, $1, $3); }
//	|	expr '+' expr 	{ $$ = zend_ast_create_binary_op(ZEND_ADD, $1, $3); }
//	|	expr '-' expr 	{ $$ = zend_ast_create_binary_op(ZEND_SUB, $1, $3); }
//	|	expr '*' expr	{ $$ = zend_ast_create_binary_op(ZEND_MUL, $1, $3); }
//	|	expr T_POW expr	{ $$ = zend_ast_create_binary_op(ZEND_POW, $1, $3); }
//	|	expr '/' expr	{ $$ = zend_ast_create_binary_op(ZEND_DIV, $1, $3); }
//	|	expr '%' expr 	{ $$ = zend_ast_create_binary_op(ZEND_MOD, $1, $3); }
//	| 	expr T_SL expr	{ $$ = zend_ast_create_binary_op(ZEND_SL, $1, $3); }
//	|	expr T_SR expr	{ $$ = zend_ast_create_binary_op(ZEND_SR, $1, $3); }
//	|	'+' expr %prec T_INC { $$ = zend_ast_create(ZEND_AST_UNARY_PLUS, $2); }
//	|	'-' expr %prec T_INC { $$ = zend_ast_create(ZEND_AST_UNARY_MINUS, $2); }
//	|	'!' expr { $$ = zend_ast_create_ex(ZEND_AST_UNARY_OP, ZEND_BOOL_NOT, $2); }
//	|	'~' expr { $$ = zend_ast_create_ex(ZEND_AST_UNARY_OP, ZEND_BW_NOT, $2); }
//	|	expr T_IS_IDENTICAL expr
//	|	expr T_IS_NOT_IDENTICAL expr
//	|	expr T_IS_EQUAL expr
//	|	expr T_IS_NOT_EQUAL expr
//	|	expr '<' expr
//	|	expr T_IS_SMALLER_OR_EQUAL expr
//	|	expr '>' expr
//	|	expr T_IS_GREATER_OR_EQUAL expr
//	|	expr T_SPACESHIP expr
//	|	expr T_INSTANCEOF class_name_reference
//	|	'(' expr ')' { $$ = $2; }
//	|	new_expr { $$ = $1; }
//	|	expr '?' expr ':' expr
//	|	expr '?' ':' expr
//	|	expr T_COALESCE expr
//	|	internal_functions_in_yacc { $$ = $1; }
//	|	T_INT_CAST expr		{ $$ = zend_ast_create_cast(IS_LONG, $2); }
//	|	T_DOUBLE_CAST expr	{ $$ = zend_ast_create_cast(IS_DOUBLE, $2); }
//	|	T_STRING_CAST expr	{ $$ = zend_ast_create_cast(IS_STRING, $2); }
//	|	T_ARRAY_CAST expr	{ $$ = zend_ast_create_cast(IS_ARRAY, $2); }
//	|	T_OBJECT_CAST expr	{ $$ = zend_ast_create_cast(IS_OBJECT, $2); }
//	|	T_BOOL_CAST expr	{ $$ = zend_ast_create_cast(_IS_BOOL, $2); }
//	|	T_UNSET_CAST expr	{ $$ = zend_ast_create_cast(IS_NULL, $2); }
//	|	T_EXIT exit_expr	{ $$ = zend_ast_create(ZEND_AST_EXIT, $2); }
//	|	'@' expr			{ $$ = zend_ast_create(ZEND_AST_SILENCE, $2); }
//	|	scalar { $$ = $1; }
//	|	'`' backticks_expr '`' { $$ = zend_ast_create(ZEND_AST_SHELL_EXEC, $2); }
//	|	T_PRINT expr { $$ = zend_ast_create(ZEND_AST_PRINT, $2); }
//	|	T_YIELD { $$ = zend_ast_create(ZEND_AST_YIELD, NULL, NULL); }
//	|	T_YIELD expr { $$ = zend_ast_create(ZEND_AST_YIELD, $2, NULL); }
//	|	T_YIELD expr T_DOUBLE_ARROW expr { $$ = zend_ast_create(ZEND_AST_YIELD, $4, $2); }
//	|	T_YIELD_FROM expr { $$ = zend_ast_create(ZEND_AST_YIELD_FROM, $2); }
//	|	function returns_ref '(' parameter_list ')' lexical_vars return_type
//	|	T_STATIC function returns_ref '(' parameter_list ')' lexical_vars
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
