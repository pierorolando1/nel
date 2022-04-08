use crate::token::Token;

pub trait AstNode {
    fn token_literal(&self) -> String;
    fn to_string(&self) -> String;
}

pub trait Expression: AstNode {}
pub trait Statement: AstNode {}

pub struct Program {
    pub statements: Vec<Box<dyn Statement>>,
}

impl Program {
    pub fn new(statements: Vec<Box<dyn Statement>>) -> Program {
        Program { statements }
    }
}

impl AstNode for Program {
    fn token_literal(&self) -> String {
        if self.statements.len() > 0 {
            self.statements
                .iter()
                .map(|s| s.token_literal())
                .collect::<Vec<String>>()
                .join("")
        } else {
            "".to_owned()
        }
    }

    fn to_string(&self) -> String {
        if self.statements.len() > 0 {
            self.statements
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
                .join("")
        } else {
            "".to_owned()
        }
    }
}

/* IDENTIFIER */
pub struct Identifier {
    pub token: Token,
    pub value: String,
}

impl Identifier {
    pub fn new(token: Token, value: String) -> Identifier {
        Identifier { token, value }
    }
}

impl AstNode for Identifier {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn to_string(&self) -> String {
        self.value.clone()
    }
}

impl Expression for Identifier {}
/* END IDENTIFIER */

/* LET STATEMENT */
pub struct LetStatement {
    pub token: Token,
    pub name: Option<Identifier>,
    pub value: Option<Box<dyn Expression>>,
}

impl AstNode for LetStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn to_string(&self) -> String {
        return format!(
            "{} {} = {}",
            self.token_literal(),
            self.name.as_ref().unwrap().value,
            self.value.as_ref().unwrap().to_string()
        );
    }
}

impl Statement for LetStatement {}
/* END LET STATEMENT */

/* RETURN STATEMENT */
pub struct ReturnStatement {
    pub token: Token,
    pub return_value: Option<Box<dyn Expression>>,
}

impl AstNode for ReturnStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn to_string(&self) -> String {
        return format!(
            "{} {}",
            self.token_literal(),
            self.return_value.as_ref().unwrap().to_string()
        );
    }
}

impl Statement for ReturnStatement {}
/* END RETURN STATEMENT */

/* EXPRESSION STATEMENT */
pub struct ExpressionStatement {
    pub token: Token,
    pub expression: Option<Box<dyn Expression>>,
}

impl AstNode for ExpressionStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn to_string(&self) -> String {
        self.expression.as_ref().unwrap().to_string()
    }
}

impl Statement for ExpressionStatement {}
/* END EXPRESSION STATEMENT */

/* INTEGER */
pub struct Integer {
    pub token: Token,
    pub value: Option<isize>,
}

impl AstNode for Integer {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn to_string(&self) -> String {
        return self.value.as_ref().unwrap().to_string();
    }
}

impl Expression for Integer {}
/* END INTEGER */

/* PREFIX */
pub struct Prefix {
    pub token: Token,
    pub operator: String,
    pub right: Option<Box<dyn Expression>>,
}

impl AstNode for Prefix {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn to_string(&self) -> String {
        return format!(
            "{}{}",
            self.operator,
            self.right.as_ref().unwrap().to_string()
        );
    }
}
/* END PREFIX */

/* INFIX */
pub struct Infix {
    pub token: Token,
    pub left: Box<dyn Expression>,
    pub operator: String,
    pub right: Option<Box<dyn Expression>>,
}

impl AstNode for Infix {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn to_string(&self) -> String {
        return format!(
            "{} {} {}",
            self.left.to_string(),
            self.operator,
            self.right.as_ref().unwrap().to_string()
        );
    }
}

impl Expression for Infix {}
/* END INFIX */

/* BOOLEAN */
pub struct Boolean {
    pub token: Token,
    pub value: Option<bool>,
}

impl AstNode for Boolean {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn to_string(&self) -> String {
        return self.value.as_ref().unwrap().to_string();
    }
}

impl Expression for Boolean {}
/* END BOOLEAN */

/* BLOCK STATEMENT */
pub struct Block {
    pub token: Token,
    pub statements: Vec<Box<dyn Statement>>,
}

impl AstNode for Block {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
    fn to_string(&self) -> String {
        return format!(
            "{}",
            self.statements
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
                .join("")
        );
    }
}

impl Statement for Block {}
/* END BLOCK STATEMENT */

/* IF EXPRESSION */
pub struct If {
    pub token: Token,
    pub condition: Option<Box<dyn Expression>>,
    pub consequence: Option<Box<Block>>,
    pub alternative: Option<Box<Block>>,
}

impl AstNode for If {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
    fn to_string(&self) -> String {
        let mut out = format!(
            "si {} {}",
            self.condition.as_ref().unwrap().to_string(),
            self.consequence.as_ref().unwrap().to_string()
        );
        if self.alternative.is_some() {
            out = format!(
                "{} sino {}",
                out,
                self.alternative.as_ref().unwrap().to_string()
            );
        }

        return out;
    }
}

impl Expression for If {}
/* END IF EXPRESSION */

/* FUNCTION EXPRESSION */
pub struct Function {
    pub token: Token,
    pub parameters: Vec<Identifier>,
    pub body: Option<Block>,
}

impl AstNode for Function {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn to_string(&self) -> String {
        let param_list = self
            .parameters
            .iter()
            .map(|p| p.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        return format!(
            "{}({}) {}",
            self.token_literal(),
            param_list,
            self.body.as_ref().unwrap().to_string()
        );
    }
}

impl Expression for Function {}
/* END FUNCTION EXPRESSION */

/* CALL EXPRESSION */
pub struct Call {
    pub token: Token,
    pub function: Box<dyn Expression>,
    pub arguments: Option<Vec<Box<dyn Expression>>>,
}

impl AstNode for Call {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn to_string(&self) -> String {
        let args = self
            .arguments
            .as_ref()
            .unwrap()
            .iter()
            .map(|a| a.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        return format!("{}({})", self.function.to_string(), args);
    }
}

impl Expression for Call {}
/* END CALL EXPRESSION */

/* STRING LITERAL */
pub struct StringLiteral {
    pub token: Token,
    pub value: String,
}

impl AstNode for StringLiteral {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn to_string(&self) -> String {
        return self.value.clone();
    }
}

impl Expression for StringLiteral {}
/* END STRING LITERAL */
