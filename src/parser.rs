use std::collections::HashMap;
use crate::lexer::{EOF,Lexer,Token,TokenType};
use crate::VecPoly;

/// Defines the set of tokens which are considered to identify logical
/// connectives (e.g. `&&`, `||`, etc).
pub const LOGICAL_CONNECTIVES : &[TokenType] = &[
    TokenType::AmpersandAmpersand,
    TokenType::BarBar,
    TokenType::LongRightArrow
];

/// Defines the set of tokens which are considered to identify
/// arithmetic comparators (e.g. `<`, `<=`, `==`, etc).
pub const ARITHMETIC_COMPARATORS : &[TokenType] = &[
    TokenType::EqualsEquals,
    TokenType::ShreakEquals,
    TokenType::LeftAngle,
    TokenType::LeftAngleEquals,
    TokenType::RightAngle,
    TokenType::RightAngleEquals
];

/// Defines the set of tokens which are considered to identify
/// arithmetic operators (e.g. `+`, `-`, `*`, etc).
pub const ARITHMETIC_OPERATORS : &[TokenType] = &[
    TokenType::Minus,
    TokenType::Percent,
    TokenType::Plus,
    TokenType::RightSlash,
    TokenType::Star
];

pub const BINARY_CONNECTIVES : &[ &[TokenType] ] = &[
    ARITHMETIC_OPERATORS,
    ARITHMETIC_COMPARATORS,
    LOGICAL_CONNECTIVES
];

pub const TYPES : &[TokenType] = &[
    TokenType::Uint
];

// ===================================================================
// Parser
// ===================================================================

/// Simplest possible parser.  Its a combination lexer and parser!
pub struct Parser {
    /// Character sequence being parsed
    lexer: Lexer,
    /// Environment used for determining indices
    env: Environment,
    /// Bytecode program being constructed
    polys: Vec<VecPoly>
}

impl Parser {
    /// Construct a parser from a string slice.
    pub fn new(content: &str) -> Self {
        // Convert string slice into Vec<char>
        let lexer = Lexer::new(content);
        //
        let env = Environment::new();
        // Done
        Self{lexer, env, polys: Vec::new()}
    }

    /// Parse a line of text into a term.
    pub fn parse(mut self) -> Result<Vec<VecPoly>,()> {
        while self.lexer.lookahead(0) != EOF {
	    self.parse_declaration()?;
        }
        //
        Ok(self.polys)
    }

    fn parse_declaration(&mut self) -> Result<(),()> {
        let lookahead = self.lexer.lookahead(0);
        //
        match lookahead.kind {
            _ => {
                self.parse_decl_assert()
            }
        }
    }

    // ===============================================================
    // Declarations
    // ===============================================================

    fn parse_decl_assert(&mut self) -> Result<(),()> {
	let expr = self.parse_expr()?;
	// Parse asserted expression
	self.polys.push(expr);
	// Done
	Ok(())
    }
    
    // ===============================================================
    // Expressions
    // ===============================================================

    pub fn parse_expr(&mut self) -> Result<VecPoly,()> {
        self.parse_expr_binary(3)
    }

    /// Parse a binary expression at a given _level_.  Higher levels
    /// indicate expressions which bind _less tightly_.  Furthermore,
    /// level `0` corresponds simply to parsing a unary expression.
    fn parse_expr_binary(&mut self, level: usize) -> Result<VecPoly,()> {
        if level == 0 {
            self.parse_expr_unit()
        } else {
            let tokens = BINARY_CONNECTIVES[level-1];
            // Parse level below
	    let mut poly = self.parse_expr_binary(level-1)?;
            // Check whether binary connective follows
            match self.lexer.match_any(tokens) {
                Some(t) => {
		    let rhs = self.parse_expr_binary(level-1)?;
		    poly = Self::apply_binary_expr(t.kind,poly,rhs);
	            
                }
                None => {}
            };
	    // Done
	    Ok(poly)
        }
    }
    
    fn parse_expr_unit(&mut self) -> Result<VecPoly,()> {
        let lookahead = self.lexer.lookahead(0);
        //
        match lookahead.kind {
            TokenType::BoolLiteral(v) => self.parse_literal_bool(v),
            TokenType::LeftBrace => self.parse_expr_braced(),
            //TokenType::Identifier => self.parse_expr_varaccess(),
            TokenType::IntLiteral => self.parse_literal_int(),
            // TokenType::Shreak => self.parse_expr_not(),
            // TokenType::If => self.parse_expr_ifelse(),
            _ => {
                panic!("unexpected token {lookahead:?}");
            }
        }
    }

    fn parse_expr_braced(&mut self) -> Result<VecPoly,()> {
        self.lexer.expect(TokenType::LeftBrace);
	let p = self.parse_expr()?;
        self.lexer.expect(TokenType::RightBrace);
	Ok(p)
    }

    // ===============================================================
    // Literals
    // ===============================================================

    fn parse_literal_bool(&mut self, val: bool) -> Result<VecPoly,()> {
        self.lexer.expect(TokenType::BoolLiteral(val));
	// true => 0, false => 1 (loobean)
	let i = if val { 0 } else { 1 };
	Ok(VecPoly::from(i))	
    }

    fn parse_literal_int(&mut self) -> Result<VecPoly,()> {
        let tok = self.lexer.expect(TokenType::IntLiteral);
        let s = self.lexer.to_string(&tok);
        let i = s.parse::<usize>().unwrap();	
        Ok(VecPoly::from(i))
    }

    // ===============================================================
    // Misc
    // ===============================================================

    fn parse_identifier(&mut self) -> Result<String,()> {
        let ith = self.lexer.expect(TokenType::Identifier);
        Ok(self.lexer.to_string(&ith))
    }

    fn apply_binary_expr(t: TokenType, lhs: VecPoly, rhs: VecPoly) -> VecPoly {
	match t {
	    // Logical
	    TokenType::AmpersandAmpersand => lhs.add(&rhs),
	    TokenType::BarBar => lhs.mul(&rhs),
	    // Relational
	    // Arithmetic
	    TokenType::Minus => lhs.sub(&rhs),
	    TokenType::Plus => lhs.add(&rhs),
	    TokenType::Star => lhs.mul(&rhs),
	    _ => { panic!(); }
	}
    }
}

// ===================================================================
// Typing Environment
// ===================================================================

struct Environment {
    /// Maps functions to their indices
    fn_bindings: HashMap<String,(usize,usize)>,
    /// Maps variables to their indices
    var_bindings: HashMap<String,usize>
}

impl Environment {
    pub fn new() -> Self {
        let fn_bindings = HashMap::new();
        let var_bindings = HashMap::new();
        Self{fn_bindings,var_bindings}
    }

    pub fn alloc_fn(&mut self, name: &str, arity: usize) {
        // Determine index of this function
        let index = self.fn_bindings.len();
        //
        self.fn_bindings.insert(name.to_string(),(index,arity));
    }

    pub fn alloc_vars(&mut self, vars: &[String]) {
        self.var_bindings.clear();
        //
        for (i,n) in vars.iter().enumerate() {
            self.var_bindings.insert(n.to_string(),i);
        }
    }

    pub fn lookup_fn(&self, name: &str) -> Option<(usize,usize)> {
	self.fn_bindings.get(name).copied()
    }

    pub fn lookup_var(&self, name: &str) -> Option<usize> {
	self.var_bindings.get(name).copied()
    }
}
