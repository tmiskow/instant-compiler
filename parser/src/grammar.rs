// auto-generated: "lalrpop 0.17.2"
// sha256: 8f3271ce6ecec79985f6b95443174edbee2b7055f59cdebc886ce42cea25f25d
use std::str::FromStr;
use crate::ast::{Expression, OperationType};
#[allow(unused_extern_crates)]
extern crate lalrpop_util as __lalrpop_util;
#[allow(unused_imports)]
use self::__lalrpop_util::state_machine as __state_machine;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod __parse__Program {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use std::str::FromStr;
    use crate::ast::{Expression, OperationType};
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    use super::__intern_token::Token;
    #[allow(dead_code)]
    pub enum __Symbol<'input>
     {
        Variant0(&'input str),
        Variant1(Box<Expression>),
        Variant2(::std::vec::Vec<Box<Expression>>),
        Variant3(OperationType),
        Variant4(::std::option::Option<Box<Expression>>),
        Variant5(i32),
        Variant6(Vec<Box<Expression>>),
    }
    const __ACTION: &'static [i8] = &[
        // State 0
        0, 0, 0, 0, 0, 11,
        // State 1
        0, 0, 0, 0, 0, 11,
        // State 2
        0, 0, 0, 0, 13, 0,
        // State 3
        0, 0, 0, 0, -8, 0,
        // State 4
        0, 16, 17, 0, -10, 0,
        // State 5
        20, -12, -12, 21, -12, 0,
        // State 6
        -15, -15, -15, -15, -15, 0,
        // State 7
        -16, -16, -16, -16, -16, 0,
        // State 8
        0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0,
        // State 10
        -20, -20, -20, -20, -20, 0,
        // State 11
        0, 0, 0, 0, 22, 0,
        // State 12
        0, 0, 0, 0, 0, -4,
        // State 13
        0, 0, 0, 0, 0, 11,
        // State 14
        0, 0, 0, 0, 0, 11,
        // State 15
        0, 0, 0, 0, 0, -6,
        // State 16
        0, 0, 0, 0, 0, -26,
        // State 17
        0, 0, 0, 0, 0, 11,
        // State 18
        0, 0, 0, 0, 0, 11,
        // State 19
        0, 0, 0, 0, 0, -19,
        // State 20
        0, 0, 0, 0, 0, -7,
        // State 21
        0, 0, 0, 0, 0, -5,
        // State 22
        0, 0, 0, 0, -9, 0,
        // State 23
        20, -11, -11, 21, -11, 0,
        // State 24
        -14, -14, -14, -14, -14, 0,
        // State 25
        -13, -13, -13, -13, -13, 0,
    ];
    const __EOF_ACTION: &'static [i8] = &[
        // State 0
        -23,
        // State 1
        -25,
        // State 2
        -22,
        // State 3
        -8,
        // State 4
        -10,
        // State 5
        -12,
        // State 6
        -15,
        // State 7
        -16,
        // State 8
        -27,
        // State 9
        -21,
        // State 10
        -20,
        // State 11
        -24,
        // State 12
        -4,
        // State 13
        0,
        // State 14
        0,
        // State 15
        0,
        // State 16
        0,
        // State 17
        0,
        // State 18
        0,
        // State 19
        0,
        // State 20
        0,
        // State 21
        -5,
        // State 22
        -9,
        // State 23
        -11,
        // State 24
        -14,
        // State 25
        -13,
    ];
    const __GOTO: &'static [i8] = &[
        // State 0
        0, 0, 2, 0, 0, 3, 4, 5, 6, 7, 0, 0, 8, 9, 10, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 12, 4, 5, 6, 7, 0, 0, 8, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 15, 0,
        // State 5
        0, 0, 0, 0, 18, 0, 0, 0, 0, 0, 0, 19, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 23, 5, 6, 7, 0, 0, 8, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 24, 7, 0, 0, 8, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 8, 0, 0, 0, 0,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 0, 26, 0, 0, 8, 0, 0, 0, 0,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        0, 0, 0, 0, 18, 0, 0, 0, 0, 0, 0, 19, 0, 0, 0, 0, 0,
        // State 24
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 25
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""*""###,
            r###""+""###,
            r###""-""###,
            r###""/""###,
            r###"";""###,
            r###"r#"[0-9]+"#"###,
        ];
        __ACTION[(__state * 6)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub struct __StateMachine<'input>
    where 
    {
        input: &'input str,
        __phantom: ::std::marker::PhantomData<(&'input ())>,
    }
    impl<'input> __state_machine::ParserDefinition for __StateMachine<'input>
    where 
    {
        type Location = usize;
        type Error = &'static str;
        type Token = Token<'input>;
        type TokenIndex = usize;
        type Symbol = __Symbol<'input>;
        type Success = Vec<Box<Expression>>;
        type StateIndex = i8;
        type Action = i8;
        type ReduceIndex = i8;
        type NonterminalIndex = usize;

        #[inline]
        fn start_location(&self) -> Self::Location {
              Default::default()
        }

        #[inline]
        fn start_state(&self) -> Self::StateIndex {
              0
        }

        #[inline]
        fn token_to_index(&self, token: &Self::Token) -> Option<usize> {
            __token_to_integer(token, ::std::marker::PhantomData::<(&())>)
        }

        #[inline]
        fn action(&self, state: i8, integer: usize) -> i8 {
            __ACTION[(state as usize) * 6 + integer]
        }

        #[inline]
        fn error_action(&self, state: i8) -> i8 {
            __ACTION[(state as usize) * 6 + (6 - 1)]
        }

        #[inline]
        fn eof_action(&self, state: i8) -> i8 {
            __EOF_ACTION[state as usize]
        }

        #[inline]
        fn goto(&self, state: i8, nt: usize) -> i8 {
            __GOTO[(state as usize) * 17 + nt] - 1
        }

        fn token_to_symbol(&self, token_index: usize, token: Self::Token) -> Self::Symbol {
            __token_to_symbol(token_index, token, ::std::marker::PhantomData::<(&())>)
        }

        fn expected_tokens(&self, state: i8) -> Vec<String> {
            __expected_tokens(state as usize)
        }

        #[inline]
        fn uses_error_recovery(&self) -> bool {
            false
        }

        #[inline]
        fn error_recovery_symbol(
            &self,
            recovery: __state_machine::ErrorRecovery<Self>,
        ) -> Self::Symbol {
            panic!("error recovery not enabled for this grammar")
        }

        fn reduce(
            &mut self,
            action: i8,
            start_location: Option<&Self::Location>,
            states: &mut Vec<i8>,
            symbols: &mut Vec<__state_machine::SymbolTriple<Self>>,
        ) -> Option<__state_machine::ParseResult<Self>> {
            __reduce(
                self.input,
                action,
                start_location,
                states,
                symbols,
                ::std::marker::PhantomData::<(&())>,
            )
        }

        fn simulate_reduce(&self, action: i8) -> __state_machine::SimulatedReduce<Self> {
            __simulate_reduce(action, ::std::marker::PhantomData::<(&())>)
        }
    }
    fn __token_to_integer<
        'input,
    >(
        __token: &Token<'input>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> Option<usize>
    {
        match *__token {
            Token(1, _) if true => Some(0),
            Token(2, _) if true => Some(1),
            Token(3, _) if true => Some(2),
            Token(4, _) if true => Some(3),
            Token(5, _) if true => Some(4),
            Token(0, _) if true => Some(5),
            _ => None,
        }
    }
    fn __token_to_symbol<
        'input,
    >(
        __token_index: usize,
        __token: Token<'input>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> __Symbol<'input>
    {
        match __token_index {
            0 => match __token {
                Token(1, __tok0) => __Symbol::Variant0((__tok0)),
                _ => unreachable!(),
            },
            1 => match __token {
                Token(2, __tok0) => __Symbol::Variant0((__tok0)),
                _ => unreachable!(),
            },
            2 => match __token {
                Token(3, __tok0) => __Symbol::Variant0((__tok0)),
                _ => unreachable!(),
            },
            3 => match __token {
                Token(4, __tok0) => __Symbol::Variant0((__tok0)),
                _ => unreachable!(),
            },
            4 => match __token {
                Token(5, __tok0) => __Symbol::Variant0((__tok0)),
                _ => unreachable!(),
            },
            5 => match __token {
                Token(0, __tok0) => __Symbol::Variant0((__tok0)),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    fn __simulate_reduce<
        'input,
    >(
        __reduce_index: i8,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> __state_machine::SimulatedReduce<__StateMachine<'input>>
    {
        match __reduce_index {
            0 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 0,
                }
            }
            1 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 1,
                }
            }
            2 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 1,
                }
            }
            3 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 2,
                }
            }
            4 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 2,
                }
            }
            5 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 3,
                }
            }
            6 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 4,
                }
            }
            7 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 5,
                }
            }
            8 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 6,
                }
            }
            9 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 6,
                }
            }
            10 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 7,
                }
            }
            11 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 7,
                }
            }
            12 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 8,
                }
            }
            13 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 8,
                }
            }
            14 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 8,
                }
            }
            15 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 9,
                }
            }
            16 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 10,
                }
            }
            17 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 10,
                }
            }
            18 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 11,
                }
            }
            19 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 12,
                }
            }
            20 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 13,
                }
            }
            21 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 14,
                }
            }
            22 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 14,
                }
            }
            23 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 14,
                }
            }
            24 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 14,
                }
            }
            25 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 15,
                }
            }
            26 => __state_machine::SimulatedReduce::Accept,
            _ => panic!("invalid reduction index {}", __reduce_index)
        }
    }
    pub struct ProgramParser {
        builder: super::__intern_token::__MatcherBuilder,
        _priv: (),
    }

    impl ProgramParser {
        pub fn new() -> ProgramParser {
            let __builder = super::__intern_token::__MatcherBuilder::new();
            ProgramParser {
                builder: __builder,
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<
            'input,
        >(
            &self,
            input: &'input str,
        ) -> Result<Vec<Box<Expression>>, __lalrpop_util::ParseError<usize, Token<'input>, &'static str>>
        {
            let mut __tokens = self.builder.matcher(input);
            let __r = __state_machine::Parser::drive(
                __StateMachine {
                    input,
                    __phantom: ::std::marker::PhantomData::<(&())>,
                },
                __tokens,
            );
            __r
        }
    }
    pub(crate) fn __reduce<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> Option<Result<Vec<Box<Expression>>,__lalrpop_util::ParseError<usize, Token<'input>, &'static str>>>
    {
        let (__pop_states, __nonterminal) = match __action {
            0 => {
                __reduce0(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            1 => {
                __reduce1(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            2 => {
                __reduce2(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            3 => {
                __reduce3(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            4 => {
                __reduce4(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            5 => {
                __reduce5(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            6 => {
                __reduce6(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            7 => {
                __reduce7(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            8 => {
                __reduce8(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            9 => {
                __reduce9(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            10 => {
                __reduce10(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            11 => {
                __reduce11(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            12 => {
                __reduce12(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            13 => {
                __reduce13(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            14 => {
                __reduce14(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            15 => {
                __reduce15(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            16 => {
                __reduce16(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            17 => {
                __reduce17(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            18 => {
                __reduce18(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            19 => {
                __reduce19(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            20 => {
                __reduce20(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            21 => {
                __reduce21(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            22 => {
                __reduce22(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            23 => {
                __reduce23(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            24 => {
                __reduce24(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            25 => {
                __reduce25(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            26 => {
                // __Program = Program => ActionFn(0);
                let __sym0 = __pop_Variant6(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(input, __sym0);
                return Some(Ok(__nt));
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __states_len = __states.len();
        __states.truncate(__states_len - __pop_states);
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 17 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Variant1<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Expression>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant1(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant3<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, OperationType, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant3(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant6<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Box<Expression>>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant6(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant5<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, i32, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant5(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant4<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<Box<Expression>>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant4(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant2<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Box<Expression>>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant2(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant0<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant0(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    pub(crate) fn __reduce0<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<Expression> ";") = Expression, ";" => ActionFn(21);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action21::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (2, 0)
    }
    pub(crate) fn __reduce1<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<Expression> ";")* =  => ActionFn(19);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action19::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (0, 1)
    }
    pub(crate) fn __reduce2<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<Expression> ";")* = (<Expression> ";")+ => ActionFn(20);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action20::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 1)
    }
    pub(crate) fn __reduce3<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<Expression> ";")+ = Expression, ";" => ActionFn(24);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action24::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (2, 2)
    }
    pub(crate) fn __reduce4<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<Expression> ";")+ = (<Expression> ";")+, Expression, ";" => ActionFn(25);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action25::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (3, 2)
    }
    pub(crate) fn __reduce5<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // AdditionOperator = "+" => ActionFn(12);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action12::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 3)
    }
    pub(crate) fn __reduce6<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // DivisionOperator = "/" => ActionFn(15);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action15::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 4)
    }
    pub(crate) fn __reduce7<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Expression = Expression1 => ActionFn(2);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action2::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 5)
    }
    pub(crate) fn __reduce8<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Expression1 = Expression2, AdditionOperator, Expression1 => ActionFn(3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action3::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 6)
    }
    pub(crate) fn __reduce9<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Expression1 = Expression2 => ActionFn(4);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action4::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 6)
    }
    pub(crate) fn __reduce10<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Expression2 = Expression2, SubtractionOperator, Expression3 => ActionFn(5);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action5::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 7)
    }
    pub(crate) fn __reduce11<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Expression2 = Expression3 => ActionFn(6);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action6::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 7)
    }
    pub(crate) fn __reduce12<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Expression3 = Expression3, MultiplicationOperator, Expression4 => ActionFn(7);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action7::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 8)
    }
    pub(crate) fn __reduce13<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Expression3 = Expression3, DivisionOperator, Expression4 => ActionFn(8);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action8::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 8)
    }
    pub(crate) fn __reduce14<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Expression3 = Expression4 => ActionFn(9);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action9::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 8)
    }
    pub(crate) fn __reduce15<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Expression4 = Number => ActionFn(10);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action10::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 9)
    }
    pub(crate) fn __reduce16<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Expression? = Expression => ActionFn(17);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action17::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 10)
    }
    pub(crate) fn __reduce17<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Expression? =  => ActionFn(18);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action18::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (0, 10)
    }
    pub(crate) fn __reduce18<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // MultiplicationOperator = "*" => ActionFn(14);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action14::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 11)
    }
    pub(crate) fn __reduce19<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Number = r#"[0-9]+"# => ActionFn(11);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action11::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 12)
    }
    pub(crate) fn __reduce20<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Program = Separated<Expression, ";"> => ActionFn(1);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action1::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 13)
    }
    pub(crate) fn __reduce21<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Separated<Expression, ";"> = Expression => ActionFn(28);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action28::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 14)
    }
    pub(crate) fn __reduce22<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Separated<Expression, ";"> =  => ActionFn(29);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action29::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (0, 14)
    }
    pub(crate) fn __reduce23<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Separated<Expression, ";"> = (<Expression> ";")+, Expression => ActionFn(30);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action30::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (2, 14)
    }
    pub(crate) fn __reduce24<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Separated<Expression, ";"> = (<Expression> ";")+ => ActionFn(31);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action31::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 14)
    }
    pub(crate) fn __reduce25<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // SubtractionOperator = "-" => ActionFn(13);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action13::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 15)
    }
}
pub use self::__parse__Program::ProgramParser;
#[cfg_attr(rustfmt, rustfmt_skip)]
mod __intern_token {
    #![allow(unused_imports)]
    use std::str::FromStr;
    use crate::ast::{Expression, OperationType};
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    extern crate regex as __regex;
    use std::fmt as __fmt;

    #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct Token<'input>(pub usize, pub &'input str);
    impl<'a> __fmt::Display for Token<'a> {
        fn fmt<'f>(&self, formatter: &mut __fmt::Formatter<'f>) -> Result<(), __fmt::Error> {
            __fmt::Display::fmt(self.1, formatter)
        }
    }

    pub struct __MatcherBuilder {
        regex_set: __regex::RegexSet,
        regex_vec: Vec<__regex::Regex>,
    }

    impl __MatcherBuilder {
        pub fn new() -> __MatcherBuilder {
            let __strs: &[&str] = &[
                "^([0-9]+)",
                "^(\\*)",
                "^(\\+)",
                "^(\\-)",
                "^(/)",
                "^(;)",
            ];
            let __regex_set = __regex::RegexSet::new(__strs).unwrap();
            let __regex_vec = vec![
                __regex::Regex::new("^([0-9]+)").unwrap(),
                __regex::Regex::new("^(\\*)").unwrap(),
                __regex::Regex::new("^(\\+)").unwrap(),
                __regex::Regex::new("^(\\-)").unwrap(),
                __regex::Regex::new("^(/)").unwrap(),
                __regex::Regex::new("^(;)").unwrap(),
            ];
            __MatcherBuilder { regex_set: __regex_set, regex_vec: __regex_vec }
        }
        pub fn matcher<'input, 'builder>(&'builder self, s: &'input str) -> __Matcher<'input, 'builder> {
            __Matcher {
                text: s,
                consumed: 0,
                regex_set: &self.regex_set,
                regex_vec: &self.regex_vec,
            }
        }
    }

    pub struct __Matcher<'input, 'builder> {
        text: &'input str,
        consumed: usize,
        regex_set: &'builder __regex::RegexSet,
        regex_vec: &'builder Vec<__regex::Regex>,
    }

    impl<'input, 'builder> Iterator for __Matcher<'input, 'builder> {
        type Item = Result<(usize, Token<'input>, usize), __lalrpop_util::ParseError<usize,Token<'input>,&'static str>>;

        fn next(&mut self) -> Option<Self::Item> {
            let __text = self.text.trim_start();
            let __whitespace = self.text.len() - __text.len();
            let __start_offset = self.consumed + __whitespace;
            if __text.is_empty() {
                self.text = __text;
                self.consumed = __start_offset;
                None
            } else {
                let __matches = self.regex_set.matches(__text);
                if !__matches.matched_any() {
                    Some(Err(__lalrpop_util::ParseError::InvalidToken {
                        location: __start_offset,
                    }))
                } else {
                    let mut __longest_match = 0;
                    let mut __index = 0;
                    for __i in 0 .. 6 {
                        if __matches.matched(__i) {
                            let __match = self.regex_vec[__i].find(__text).unwrap();
                            let __len = __match.end();
                            if __len >= __longest_match {
                                __longest_match = __len;
                                __index = __i;
                            }
                        }
                    }
                    let __result = &__text[..__longest_match];
                    let __remaining = &__text[__longest_match..];
                    let __end_offset = __start_offset + __longest_match;
                    self.text = __remaining;
                    self.consumed = __end_offset;
                    Some(Ok((__start_offset, Token(__index, __result), __end_offset)))
                }
            }
        }
    }
}
pub use self::__intern_token::Token;

#[allow(unused_variables)]
fn __action0<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Vec<Box<Expression>>, usize),
) -> Vec<Box<Expression>>
{
    (__0)
}

#[allow(unused_variables)]
fn __action1<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Vec<Box<Expression>>, usize),
) -> Vec<Box<Expression>>
{
    (__0)
}

#[allow(unused_variables)]
fn __action2<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Box<Expression>, usize),
) -> Box<Expression>
{
    (__0)
}

#[allow(unused_variables)]
fn __action3<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Box<Expression>, usize),
    (_, __1, _): (usize, OperationType, usize),
    (_, __2, _): (usize, Box<Expression>, usize),
) -> Box<Expression>
{
    Box::new(Expression::Operation(__0, __1, __2))
}

#[allow(unused_variables)]
fn __action4<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Box<Expression>, usize),
) -> Box<Expression>
{
    (__0)
}

#[allow(unused_variables)]
fn __action5<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Box<Expression>, usize),
    (_, __1, _): (usize, OperationType, usize),
    (_, __2, _): (usize, Box<Expression>, usize),
) -> Box<Expression>
{
    Box::new(Expression::Operation(__0, __1, __2))
}

#[allow(unused_variables)]
fn __action6<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Box<Expression>, usize),
) -> Box<Expression>
{
    (__0)
}

#[allow(unused_variables)]
fn __action7<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Box<Expression>, usize),
    (_, __1, _): (usize, OperationType, usize),
    (_, __2, _): (usize, Box<Expression>, usize),
) -> Box<Expression>
{
    Box::new(Expression::Operation(__0, __1, __2))
}

#[allow(unused_variables)]
fn __action8<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Box<Expression>, usize),
    (_, __1, _): (usize, OperationType, usize),
    (_, __2, _): (usize, Box<Expression>, usize),
) -> Box<Expression>
{
    Box::new(Expression::Operation(__0, __1, __2))
}

#[allow(unused_variables)]
fn __action9<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Box<Expression>, usize),
) -> Box<Expression>
{
    (__0)
}

#[allow(unused_variables)]
fn __action10<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, i32, usize),
) -> Box<Expression>
{
    Box::new(Expression::Number(__0))
}

#[allow(unused_variables)]
fn __action11<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> i32
{
    i32::from_str(__0).unwrap()
}

#[allow(unused_variables)]
fn __action12<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> OperationType
{
    OperationType::Add
}

#[allow(unused_variables)]
fn __action13<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> OperationType
{
    OperationType::Sub
}

#[allow(unused_variables)]
fn __action14<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> OperationType
{
    OperationType::Mul
}

#[allow(unused_variables)]
fn __action15<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> OperationType
{
    OperationType::Div
}

#[allow(unused_variables)]
fn __action16<
    'input,
>(
    input: &'input str,
    (_, vector, _): (usize, ::std::vec::Vec<Box<Expression>>, usize),
    (_, end, _): (usize, ::std::option::Option<Box<Expression>>, usize),
) -> Vec<Box<Expression>>
{
    match end {
        None => vector,
        Some(end) => {
            let mut vector = vector;
            vector.push(end);
            vector
        }
    }
}

#[allow(unused_variables)]
fn __action17<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Box<Expression>, usize),
) -> ::std::option::Option<Box<Expression>>
{
    Some(__0)
}

#[allow(unused_variables)]
fn __action18<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::option::Option<Box<Expression>>
{
    None
}

#[allow(unused_variables)]
fn __action19<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<Box<Expression>>
{
    vec![]
}

#[allow(unused_variables)]
fn __action20<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<Box<Expression>>, usize),
) -> ::std::vec::Vec<Box<Expression>>
{
    v
}

#[allow(unused_variables)]
fn __action21<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Box<Expression>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Box<Expression>
{
    (__0)
}

#[allow(unused_variables)]
fn __action22<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Box<Expression>, usize),
) -> ::std::vec::Vec<Box<Expression>>
{
    vec![__0]
}

#[allow(unused_variables)]
fn __action23<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<Box<Expression>>, usize),
    (_, e, _): (usize, Box<Expression>, usize),
) -> ::std::vec::Vec<Box<Expression>>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
fn __action24<
    'input,
>(
    input: &'input str,
    __0: (usize, Box<Expression>, usize),
    __1: (usize, &'input str, usize),
) -> ::std::vec::Vec<Box<Expression>>
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action21(
        input,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action22(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action25<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<Box<Expression>>, usize),
    __1: (usize, Box<Expression>, usize),
    __2: (usize, &'input str, usize),
) -> ::std::vec::Vec<Box<Expression>>
{
    let __start0 = __1.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action21(
        input,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action23(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action26<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::option::Option<Box<Expression>>, usize),
) -> Vec<Box<Expression>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action19(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action16(
        input,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
fn __action27<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<Box<Expression>>, usize),
    __1: (usize, ::std::option::Option<Box<Expression>>, usize),
) -> Vec<Box<Expression>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action20(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action16(
        input,
        __temp0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action28<
    'input,
>(
    input: &'input str,
    __0: (usize, Box<Expression>, usize),
) -> Vec<Box<Expression>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action17(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action26(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action29<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Vec<Box<Expression>>
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action18(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action26(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action30<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<Box<Expression>>, usize),
    __1: (usize, Box<Expression>, usize),
) -> Vec<Box<Expression>>
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action17(
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action27(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action31<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<Box<Expression>>, usize),
) -> Vec<Box<Expression>>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action18(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action27(
        input,
        __0,
        __temp0,
    )
}

pub trait __ToTriple<'input, > {
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize), __lalrpop_util::ParseError<usize, Token<'input>, &'static str>>;
}

impl<'input, > __ToTriple<'input, > for (usize, Token<'input>, usize) {
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize), __lalrpop_util::ParseError<usize, Token<'input>, &'static str>> {
        Ok(value)
    }
}
impl<'input, > __ToTriple<'input, > for Result<(usize, Token<'input>, usize), &'static str> {
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize), __lalrpop_util::ParseError<usize, Token<'input>, &'static str>> {
        match value {
            Ok(v) => Ok(v),
            Err(error) => Err(__lalrpop_util::ParseError::User { error }),
        }
    }
}
