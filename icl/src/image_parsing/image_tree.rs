// auto-generated: "lalrpop 0.19.6"
// sha3: e5602d739718ddc5497e7ad075ab35135b7d5d65bbe0571612704638e08521e4
use super::ast::{Expr, Op, Stmt};
use super::token::{self, Token, LexicalError};
#[allow(unused_extern_crates)]
extern crate lalrpop_util as __lalrpop_util;
#[allow(unused_imports)]
use self::__lalrpop_util::state_machine as __state_machine;
extern crate core;
extern crate alloc;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod __parse__Program {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use super::super::ast::{Expr, Op, Stmt};
    use super::super::token::{self, Token, LexicalError};
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    extern crate core;
    extern crate alloc;
    use super::__ToTriple;
    #[allow(dead_code)]
    pub(crate) enum __Symbol<>
     {
        Variant0(token::Token),
        Variant1(usize),
        Variant2(i64),
        Variant3(Op),
        Variant4(Expr),
        Variant5(Vec<Stmt>),
        Variant6(Stmt),
        Variant7(alloc::vec::Vec<Stmt>),
    }
    const __ACTION: &[i8] = &[
        // State 0
        10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 1
        10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        5, 0, 0, 0, 0, 0, 0, 0, 14, 15, 0,
        // State 3
        5, 0, 0, 0, 0, 0, 0, 0, 14, 15, 0,
        // State 4
        0, 0, 19, 20, 21, 22, 0, 0, 0, 0, 0,
        // State 5
        5, 0, 0, 0, 0, 0, 0, 0, 14, 15, 0,
        // State 6
        5, 0, 0, 0, 0, 0, 0, 0, 14, 15, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        -15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 3, 12, 0, 0, 4,
        // State 10
        -16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 18, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        -6, -6, 0, 0, 0, 0, 0, 0, -6, -6, 0,
        // State 14
        -7, -7, 0, 0, 0, 0, 0, 0, -7, -7, 0,
        // State 15
        -10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, 23, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        -11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        -3, 0, 0, 0, 0, 0, 0, 0, -3, -3, 0,
        // State 19
        -1, 0, 0, 0, 0, 0, 0, 0, -1, -1, 0,
        // State 20
        -2, 0, 0, 0, 0, 0, 0, 0, -2, -2, 0,
        // State 21
        -4, 0, 0, 0, 0, 0, 0, 0, -4, -4, 0,
        // State 22
        -12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        0, 25, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 24
        -5, -5, 0, 0, 0, 0, 0, 0, -5, -5, 0,
    ];
    fn __action(state: i8, integer: usize) -> i8 {
        __ACTION[(state as usize) * 11 + integer]
    }
    const __EOF_ACTION: &[i8] = &[
        // State 0
        -8,
        // State 1
        -9,
        // State 2
        0,
        // State 3
        0,
        // State 4
        0,
        // State 5
        0,
        // State 6
        0,
        // State 7
        -17,
        // State 8
        -15,
        // State 9
        0,
        // State 10
        -16,
        // State 11
        0,
        // State 12
        0,
        // State 13
        0,
        // State 14
        0,
        // State 15
        -10,
        // State 16
        0,
        // State 17
        -11,
        // State 18
        0,
        // State 19
        0,
        // State 20
        0,
        // State 21
        0,
        // State 22
        -12,
        // State 23
        0,
        // State 24
        0,
    ];
    fn __goto(state: i8, nt: usize) -> i8 {
        match nt {
            0 => 5,
            1 => match state {
                2 => 12,
                3 => 16,
                6 => 23,
                _ => 6,
            },
            2 => 7,
            3 => match state {
                1 => 10,
                _ => 8,
            },
            5 => 1,
            _ => 0,
        }
    }
    fn __expected_tokens(__state: i8) -> alloc::vec::Vec<alloc::string::String> {
        const __TERMINAL: &[&str] = &[
            r###""(""###,
            r###"")""###,
            r###""*""###,
            r###""+""###,
            r###""-""###,
            r###""/""###,
            r###"Assign"###,
            r###"Define"###,
            r###"Name"###,
            r###"Num"###,
            r###"Print"###,
        ];
        __TERMINAL.iter().enumerate().filter_map(|(index, terminal)| {
            let next_state = __action(__state, index);
            if next_state == 0 {
                None
            } else {
                Some(alloc::string::ToString::to_string(terminal))
            }
        }).collect()
    }
    pub(crate) struct __StateMachine<>
    where 
    {
        __phantom: core::marker::PhantomData<()>,
    }
    impl<> __state_machine::ParserDefinition for __StateMachine<>
    where 
    {
        type Location = usize;
        type Error = LexicalError;
        type Token = token::Token;
        type TokenIndex = usize;
        type Symbol = __Symbol<>;
        type Success = Vec<Stmt>;
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
            __token_to_integer(token, core::marker::PhantomData::<()>)
        }

        #[inline]
        fn action(&self, state: i8, integer: usize) -> i8 {
            __action(state, integer)
        }

        #[inline]
        fn error_action(&self, state: i8) -> i8 {
            __action(state, 11 - 1)
        }

        #[inline]
        fn eof_action(&self, state: i8) -> i8 {
            __EOF_ACTION[state as usize]
        }

        #[inline]
        fn goto(&self, state: i8, nt: usize) -> i8 {
            __goto(state, nt)
        }

        fn token_to_symbol(&self, token_index: usize, token: Self::Token) -> Self::Symbol {
            __token_to_symbol(token_index, token, core::marker::PhantomData::<()>)
        }

        fn expected_tokens(&self, state: i8) -> alloc::vec::Vec<alloc::string::String> {
            __expected_tokens(state)
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
            states: &mut alloc::vec::Vec<i8>,
            symbols: &mut alloc::vec::Vec<__state_machine::SymbolTriple<Self>>,
        ) -> Option<__state_machine::ParseResult<Self>> {
            __reduce(
                action,
                start_location,
                states,
                symbols,
                core::marker::PhantomData::<()>,
            )
        }

        fn simulate_reduce(&self, action: i8) -> __state_machine::SimulatedReduce<Self> {
            panic!("error recovery not enabled for this grammar")
        }
    }
    fn __token_to_integer<
    >(
        __token: &token::Token,
        _: core::marker::PhantomData<()>,
    ) -> Option<usize>
    {
        match *__token {
            Token::LParen if true => Some(0),
            Token::RParen if true => Some(1),
            Token::Star if true => Some(2),
            Token::Plus if true => Some(3),
            Token::Minus if true => Some(4),
            Token::Slash if true => Some(5),
            Token::Assign(_) if true => Some(6),
            Token::Define(_) if true => Some(7),
            Token::Name(_) if true => Some(8),
            Token::Num(_) if true => Some(9),
            Token::Print if true => Some(10),
            _ => None,
        }
    }
    fn __token_to_symbol<
    >(
        __token_index: usize,
        __token: token::Token,
        _: core::marker::PhantomData<()>,
    ) -> __Symbol<>
    {
        match __token_index {
            0 | 1 | 2 | 3 | 4 | 5 | 10 => __Symbol::Variant0(__token),
            6 | 7 | 8 => match __token {
                Token::Assign(__tok0) | Token::Define(__tok0) | Token::Name(__tok0) if true => __Symbol::Variant1(__tok0),
                _ => unreachable!(),
            },
            9 => match __token {
                Token::Num(__tok0) if true => __Symbol::Variant2(__tok0),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    pub struct ProgramParser {
        _priv: (),
    }

    impl ProgramParser {
        pub fn new() -> ProgramParser {
            ProgramParser {
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<
            __TOKEN: __ToTriple<>,
            __TOKENS: IntoIterator<Item=__TOKEN>,
        >(
            &self,
            __tokens0: __TOKENS,
        ) -> Result<Vec<Stmt>, __lalrpop_util::ParseError<usize, token::Token, LexicalError>>
        {
            let __tokens = __tokens0.into_iter();
            let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
            __state_machine::Parser::drive(
                __StateMachine {
                    __phantom: core::marker::PhantomData::<()>,
                },
                __tokens,
            )
        }
    }
    pub(crate) fn __reduce<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut alloc::vec::Vec<i8>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> Option<Result<Vec<Stmt>,__lalrpop_util::ParseError<usize, token::Token, LexicalError>>>
    {
        let (__pop_states, __nonterminal) = match __action {
            0 => {
                __reduce0(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            1 => {
                __reduce1(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            2 => {
                __reduce2(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            3 => {
                __reduce3(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            4 => {
                __reduce4(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            5 => {
                __reduce5(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            6 => {
                __reduce6(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            7 => {
                __reduce7(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            8 => {
                __reduce8(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            9 => {
                __reduce9(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            10 => {
                __reduce10(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            11 => {
                __reduce11(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            12 => {
                __reduce12(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            13 => {
                __reduce13(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            14 => {
                __reduce14(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            15 => {
                __reduce15(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            16 => {
                // __Program = Program => ActionFn(0);
                let __sym0 = __pop_Variant5(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(__sym0);
                return Some(Ok(__nt));
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __states_len = __states.len();
        __states.truncate(__states_len - __pop_states);
        let __state = *__states.last().unwrap();
        let __next_state = __goto(__state, __nonterminal);
        __states.push(__next_state);
        None
    }
    #[inline(never)]
    fn __symbol_type_mismatch() -> ! {
        panic!("symbol type mismatch")
    }
    fn __pop_Variant4<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Expr, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant4(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant3<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Op, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant3(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant6<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Stmt, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant6(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant5<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Vec<Stmt>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant5(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant7<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, alloc::vec::Vec<Stmt>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant7(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant2<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, i64, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant2(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant0<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, token::Token, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant0(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant1<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, usize, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant1(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    pub(crate) fn __reduce0<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // BinaryOp = "+" => ActionFn(8);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action8::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 0)
    }
    pub(crate) fn __reduce1<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // BinaryOp = "-" => ActionFn(9);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action9::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 0)
    }
    pub(crate) fn __reduce2<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // BinaryOp = "*" => ActionFn(10);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action10::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 0)
    }
    pub(crate) fn __reduce3<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // BinaryOp = "/" => ActionFn(11);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action11::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 0)
    }
    pub(crate) fn __reduce4<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr = "(", BinaryOp, Expr, Expr, ")" => ActionFn(5);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant4(__symbols);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action5::<>(__sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (5, 1)
    }
    pub(crate) fn __reduce5<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr = Name => ActionFn(6);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action6::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 1)
    }
    pub(crate) fn __reduce6<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr = Num => ActionFn(7);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action7::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 1)
    }
    pub(crate) fn __reduce7<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Program =  => ActionFn(16);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action16::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (0, 2)
    }
    pub(crate) fn __reduce8<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Program = Stmt+ => ActionFn(17);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action17::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 2)
    }
    pub(crate) fn __reduce9<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Stmt = "(", Define, ")" => ActionFn(2);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action2::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 3)
    }
    pub(crate) fn __reduce10<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Stmt = "(", Assign, Expr, ")" => ActionFn(3);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action3::<>(__sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (4, 3)
    }
    pub(crate) fn __reduce11<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Stmt = "(", Print, Expr, ")" => ActionFn(4);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action4::<>(__sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (4, 3)
    }
    pub(crate) fn __reduce12<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Stmt* =  => ActionFn(12);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action12::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (0, 4)
    }
    pub(crate) fn __reduce13<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Stmt* = Stmt+ => ActionFn(13);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action13::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 4)
    }
    pub(crate) fn __reduce14<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Stmt+ = Stmt => ActionFn(14);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action14::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 5)
    }
    pub(crate) fn __reduce15<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Stmt+ = Stmt+, Stmt => ActionFn(15);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action15::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (2, 5)
    }
}
pub use self::__parse__Program::ProgramParser;

fn __action0<
>(
    (_, __0, _): (usize, Vec<Stmt>, usize),
) -> Vec<Stmt>
{
    __0
}

fn __action1<
>(
    (_, __0, _): (usize, alloc::vec::Vec<Stmt>, usize),
) -> Vec<Stmt>
{
    __0
}

fn __action2<
>(
    (_, _, _): (usize, token::Token, usize),
    (_, def, _): (usize, usize, usize),
    (_, _, _): (usize, token::Token, usize),
) -> Stmt
{
    Stmt::Define(def)
}

fn __action3<
>(
    (_, _, _): (usize, token::Token, usize),
    (_, target, _): (usize, usize, usize),
    (_, expr, _): (usize, Expr, usize),
    (_, _, _): (usize, token::Token, usize),
) -> Stmt
{
    Stmt::Assign(target, Box::new(expr))
}

fn __action4<
>(
    (_, _, _): (usize, token::Token, usize),
    (_, _, _): (usize, token::Token, usize),
    (_, expr, _): (usize, Expr, usize),
    (_, _, _): (usize, token::Token, usize),
) -> Stmt
{
    Stmt::PrintStmt(Box::new(expr))
}

fn __action5<
>(
    (_, _, _): (usize, token::Token, usize),
    (_, op, _): (usize, Op, usize),
    (_, first, _): (usize, Expr, usize),
    (_, second, _): (usize, Expr, usize),
    (_, _, _): (usize, token::Token, usize),
) -> Expr
{
    Expr::Binary(op, Box::new(first), Box::new(second))
}

fn __action6<
>(
    (_, __0, _): (usize, usize, usize),
) -> Expr
{
    Expr::Variable(__0)
}

fn __action7<
>(
    (_, __0, _): (usize, i64, usize),
) -> Expr
{
    Expr::Number(__0)
}

fn __action8<
>(
    (_, __0, _): (usize, token::Token, usize),
) -> Op
{
    Op::Add
}

fn __action9<
>(
    (_, __0, _): (usize, token::Token, usize),
) -> Op
{
    Op::Sub
}

fn __action10<
>(
    (_, __0, _): (usize, token::Token, usize),
) -> Op
{
    Op::Mul
}

fn __action11<
>(
    (_, __0, _): (usize, token::Token, usize),
) -> Op
{
    Op::Div
}

fn __action12<
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> alloc::vec::Vec<Stmt>
{
    alloc::vec![]
}

fn __action13<
>(
    (_, v, _): (usize, alloc::vec::Vec<Stmt>, usize),
) -> alloc::vec::Vec<Stmt>
{
    v
}

fn __action14<
>(
    (_, __0, _): (usize, Stmt, usize),
) -> alloc::vec::Vec<Stmt>
{
    alloc::vec![__0]
}

fn __action15<
>(
    (_, v, _): (usize, alloc::vec::Vec<Stmt>, usize),
    (_, e, _): (usize, Stmt, usize),
) -> alloc::vec::Vec<Stmt>
{
    { let mut v = v; v.push(e); v }
}

fn __action16<
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Vec<Stmt>
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action12(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action1(
        __temp0,
    )
}

fn __action17<
>(
    __0: (usize, alloc::vec::Vec<Stmt>, usize),
) -> Vec<Stmt>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action13(
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action1(
        __temp0,
    )
}

pub trait __ToTriple<> {
    fn to_triple(value: Self) -> Result<(usize,token::Token,usize), __lalrpop_util::ParseError<usize, token::Token, LexicalError>>;
}

impl<> __ToTriple<> for (usize, token::Token, usize) {
    fn to_triple(value: Self) -> Result<(usize,token::Token,usize), __lalrpop_util::ParseError<usize, token::Token, LexicalError>> {
        Ok(value)
    }
}
impl<> __ToTriple<> for Result<(usize, token::Token, usize), LexicalError> {
    fn to_triple(value: Self) -> Result<(usize,token::Token,usize), __lalrpop_util::ParseError<usize, token::Token, LexicalError>> {
        match value {
            Ok(v) => Ok(v),
            Err(error) => Err(__lalrpop_util::ParseError::User { error }),
        }
    }
}
