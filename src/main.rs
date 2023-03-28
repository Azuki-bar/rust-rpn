use anyhow::Result;
use std::io::{self, Error, ErrorKind};
fn main() -> Result<()> {
    println!("welcome to rusty-rpn!");
    let mut buf = String::new();

    match io::stdin().read_line(&mut buf) {
        Ok(n) => println!("read {} bytes", n),
        Err(err) => println!("err {}", err),
    }

    let answer = tokenise(buf.to_string())
        .and_then(|node| {
            println!("{:?}", node);
            Ok(node)
        })
        .and_then(|node| execute(node));
    match answer {
        Ok(i) => {
            println!("answer is {}", i);
            Ok(())
        }
        Err(err) => {
            println!("err is {}", err);
            Err(err)
        }
    }
}
#[derive(Debug, PartialEq)]
pub enum Node {
    Val(i32),
    Plus,
    Minus,
    Mul,
    Div,
}
pub fn tokenise(input: String) -> Result<Vec<Node>> {
    input
        .trim()
        .split(" ")
        .map(|c| match c {
            "+" => Ok(Node::Plus),
            "-" => Ok(Node::Minus),
            "*" => Ok(Node::Mul),
            "/" => Ok(Node::Div),
            i => match i.parse() {
                Ok(i) => Ok(Node::Val(i)),
                Err(err) => Err(anyhow::anyhow!("parse number failed, err={:?}", err)),
            },
        })
        .collect::<Result<_>>()
}
pub fn execute(nodes: Vec<Node>) -> Result<i32> {
    let unmatch_err = Err("input error");
    let mut stack: Vec<Node> = vec![];
    let result = nodes.iter().try_for_each(|node| match node {
        Node::Plus => match (stack.pop(), stack.pop()) {
            (Some(Node::Val(i)), Some(Node::Val(j))) => Ok(stack.push(Node::Val(j + i))),
            (_, _) => unmatch_err,
        },
        Node::Minus => match (stack.pop(), stack.pop()) {
            (Some(Node::Val(i)), Some(Node::Val(j))) => Ok(stack.push(Node::Val(j - i))),
            (_, _) => unmatch_err,
        },
        Node::Mul => match (stack.pop(), stack.pop()) {
            (Some(Node::Val(i)), Some(Node::Val(j))) => Ok(stack.push(Node::Val(j * i))),
            (_, _) => unmatch_err,
        },
        Node::Div => match (stack.pop(), stack.pop()) {
            (Some(Node::Val(i)), Some(Node::Val(j))) if i != 0 => Ok(stack.push(Node::Val(j / i))),
            (Some(Node::Val(_)), Some(Node::Val(j))) if j == 0 => Err("divide by zero"),
            (_, _) => unmatch_err,
        },
        Node::Val(i) => Ok(stack.push(Node::Val(*i))),
    });
    match result {
        Ok(()) => {
            if stack.len() != 1 {
                Err(anyhow::anyhow!(Error::new(
                    ErrorKind::Other,
                    "stack length err"
                )))
            } else if let Node::Val(i) = stack[0] {
                Ok(i)
            } else {
                Err(anyhow::anyhow!(Error::new(ErrorKind::Other, "invalid")))
            }
        }
        Err(s) => Err(anyhow::anyhow!(Error::new(ErrorKind::Unsupported, s))),
    }
}

#[cfg(test)]
mod tests {
    use crate::{tokenise, Node};

    #[test]
    fn tokenize_test() {
        assert_eq!(tokenise("123".to_string()).unwrap(), vec![Node::Val(123)]);
        assert_eq!(tokenise(" 123".to_string()).unwrap(), vec![Node::Val(123)]);
        assert_eq!(
            tokenise("1++".to_string()).unwrap(),
            vec![Node::Val(1), Node::Plus, Node::Plus]
        );
        assert_eq!(
            tokenise("+-*/".to_string()).unwrap(),
            vec![Node::Plus, Node::Minus, Node::Mul, Node::Div]
        );
        assert_eq!(tokenise("".to_string()).unwrap(), vec![]);
        assert_eq!(tokenise("     ".to_string()).unwrap(), vec![]);
    }
    #[test]
    fn tokenise_err_test() {
        // assert_eq!(tokenise("..".to_string()).unwrap_err(),)
    }
}
