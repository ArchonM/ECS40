pub enum Operator {
    // `+`
    Add,
    // `-`
    Sub,
    // `*`
    Mul,
    // '/'
    Div,
}

pub enum Token {
    Operator(Operator),
    Operand(isize),
}

/// Evaluates the postix expression.
///
/// Input: a postfix expression, where each element contains an operator or operand.
/// Returns: if the postfix expression is valid, returns `Some(value)`;
///     otherwise, returns `None`.

pub fn eval(tokens: &[Token]) -> Option<isize> {
    // TODO
    let length = (*tokens).len();
    let mut digits = 0;
    let mut operators = 0;
    let mut current;
    let mut new_array: Vec<isize> = Vec::new();
    let mut count = 0;

    for x in 0..length
    {
        match (*tokens)[x]
        {
            Token::Operand(_) => digits += 1,
            Token::Operator(_) => operators += 1,
        }
    }
    if operators != digits - 1
    {
        return None;
    }

    if length > 2
    {
        for x in 0..2
        {
            match (*tokens)[x]
            {
                Token::Operator(_) => return None,
                _ => {},
            }
        }
    }

    for x in 0..length
    {
        match (*tokens)[x]
        {
            Token::Operand(a) =>
            {
                new_array.push(a);
                count += 1;
            },
            Token::Operator(Operator::Add) =>
            {
                current = new_array[count-2] + new_array[count-1];
                new_array.pop();
                new_array.pop();
                new_array.push(current);
                count -= 1;
            },
            Token::Operator(Operator::Sub) =>
            {
                current = new_array[count-2] - new_array[count-1];
                new_array.pop();
                new_array.pop();
                new_array.push(current);
                count -= 1;
            },
            Token::Operator(Operator::Mul) =>
            {
                current = new_array[count-2] * new_array[count-1];
                new_array.pop();
                new_array.pop();
                new_array.push(current);
                count -= 1;
            },
            Token::Operator(Operator::Div) =>
            {
                current = new_array[count-2] / new_array[count-1];
                new_array.pop();
                new_array.pop();
                new_array.push(current);
                count -= 1;
            },
        }
    }


    Some(new_array[0])
}
