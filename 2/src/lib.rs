#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Operator {
    // `+`
    Add,
    // `-`
    Sub,
    // `*`
    Mul,
    // `/`
    Div,
}

#[derive(Debug, PartialEq)]
pub enum InfixToken {
    Operator(Operator),
    Operand(isize),
    LeftParen,
    RightParen,
}

#[derive(Debug, PartialEq)]
pub enum PostfixToken {
    Operator(Operator),
    Operand(isize),
}

pub fn is_valid(tokens: &[InfixToken]) -> bool {
    let mut count = 0;
    //invalid case 1: Operator first
    if let InfixToken::Operator(_) = (*tokens)[0] {return false;}
    //invalid case 2: Right Paren first
    if let InfixToken::RightParen = (*tokens)[0] {return false;}
    if let InfixToken::LeftParen = (*tokens)[0] {count += 1;}
    //invalid case 3: Operand or left parenthesis are preceded by an operand or right parenthesis
    //invalid case 4: Operator or right Parenthesis are preceeded by an operator or left parenthesis
    for x in 1..tokens.len()
    {
        if count < 0
        {return false;}

        if let InfixToken::Operand(_) = (*tokens)[x]
        {
            if let InfixToken::Operand(_) = (*tokens)[x-1]
            {return false;}
            if let InfixToken::RightParen = (*tokens)[x-1]
            {return false;}
        }

        if let InfixToken::Operator(_) = (*tokens)[x]
        {
            if let InfixToken::Operator(_) = (tokens)[x-1]
            {return false;}
            if let InfixToken::LeftParen = (tokens)[x-1]
            {return false;}
        }

        if let InfixToken::LeftParen = (*tokens)[x]
        {
            count += 1;
            if let InfixToken::Operand(_) = (*tokens)[x-1]
            {return false;}
            if let InfixToken::RightParen = (*tokens)[x-1]
            {return false;}
        }

        if let InfixToken::RightParen = (*tokens)[x]
        {
            count -=1;
            if let InfixToken::Operator(_) = (tokens)[x-1]
            {return false;}
            if let InfixToken::LeftParen = (tokens)[x-1]
            {return false;}
        }
    }
    //invalid case 5: Operator at last
    if let InfixToken::Operator(_) = (*tokens)[tokens.len()-1] {return false;}
    //invalid case 6: LeftParen at last
    if let InfixToken::LeftParen = (*tokens)[tokens.len()-1] {return false;}
    //invalid case 7: Left Right Parenthesis do not match
    if count != 0
    {return false;}
    true
}
/// Transforms an infix expression to a postfix expression.
///
/// If the infix expression is valid, outputs `Some(_)`;
/// otherwise, outputs `None`.
pub fn is_higher(token1: &InfixToken, token2: InfixToken) -> bool {
    if let InfixToken::Operator(Operator::Mul) = *token1
    {return true;}
    else if let InfixToken::Operator(Operator::Div) = *token1
    {return true;}
    else if let InfixToken::LeftParen = *token1
    {return false;}
    else if let InfixToken::Operator(Operator::Add) = token2
    {return true;}
    else if let InfixToken::Operator(Operator::Sub) = token2
    {return true;}
    false
}

pub fn in_to_post(token: InfixToken) -> PostfixToken {
    match token
    {
        InfixToken::Operator(Operator::Add) => return PostfixToken::Operator(Operator::Add),
        InfixToken::Operator(Operator::Sub) => return PostfixToken::Operator(Operator::Sub),
        InfixToken::Operator(Operator::Mul) => return PostfixToken::Operator(Operator::Mul),
        InfixToken::Operator(Operator::Div) => return PostfixToken::Operator(Operator::Div),
        _ => (),
    }
}
pub fn infix_to_postfix(tokens: &[InfixToken]) -> Option<Vec<PostfixToken>> {
    let mut index;
    let mut result: Vec<PostfixToken> = Vec::new();
    let mut temp: Vec<InfixToken> = Vec::new();

    if tokens.len() == 0
    {return None;}
    if tokens.len() == 1
    {
        if let InfixToken::Operand(a) = (*tokens)[0]
        {
            result.push(PostfixToken::Operand(a));
        }
        else
        {return None;}
        return Some(result);
    }
    //see if this array is valid
    if !is_valid(tokens)
    {return None;}
    //start coding

    for x in 0..tokens.len()
    {
        match (*tokens)[x]
        {
            InfixToken::Operand(a) => result.push(PostfixToken::Operand(a)),
            InfixToken::LeftParen => temp.push(InfixToken::LeftParen),
            InfixToken::RightParen =>
            {
                index = temp.len();
                while index > 0
                {
                    if let InfixToken::LeftParen = temp[index-1]
                    {
                        temp.pop();
                        break;
                    }
                    else
                    {
                        result.push(in_to_post(temp.pop().unwrap()));
                        index -= 1;
                    }
                }
            }
            InfixToken::Operator(token) =>
            {
                if temp.len() != 0{
                index = temp.len();
                    while index > 0
                    {
                        if is_higher(&temp[index-1],InfixToken::Operator(token))
                        {
                            result.push(in_to_post(temp.pop().unwrap()));
                            index -= 1;
                        }
                        else
                        {break;}
                    }
                }
                temp.push(InfixToken::Operator(token));
            }
        }
    }

    while temp.len() != 0
    {
        result.push(in_to_post(temp.pop().unwrap()));
    }

    Some(result)

}
