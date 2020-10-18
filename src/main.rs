use std::env;
use std::fs;
use c_lexer;
use c_lexer::token::Token::*;
use c_lexer::token::Token;

//TODO do while, if else without {},switch, bug fix
fn main() {
    let args: Vec<String> = env::args().collect();
    let path = args[1].to_string();
    let _output = args[1].to_string();
    let code = fs::read_to_string(path).unwrap();
    let l=c_lexer::Lexer::lex(&code);
    let ll=l.unwrap();

    //let mut scopes = Vec::new();
    let mut _depth = 0;
    let mut _depth: usize = 0;
    let mut _id = false;
    let mut _state=FuncName;
    let mut tk_itr = ll.clone();
    f_eq(&mut tk_itr.iter());

}
//if,brace,while,for or line
fn f_eq(tokens_iter: &mut std::slice::Iter<Token>){
    loop {
        match tokens_iter.next() {
            Some(tk) => {
                match tk {
                    IF => {
                        print!("\nif");
                        f_if(tokens_iter);
                        println!("\nendif");},
                    LBrace => {
                        println!(";");
                        f_eq(tokens_iter)},
                    WHILE => {
                        print!("\nwhile");
                        f_while(tokens_iter);
                        println!("\nendwhile");},  
                    FOR => {
                        //for==while in plantuml
                        print!("\nwhile");
                        f_while(tokens_iter);
                        println!("\nendwhile");}, 
                    RBrace=> {
                        f_else(tokens_iter);
                        break},
                    LineTerminator=>{}
                    _ => {
                        print!(":");
                        print_token(tk);
                        f_line(tokens_iter);},
                }
            },
            None => break,
        };
    }
}

fn f_line(tokens_iter: &mut std::slice::Iter<Token>){
    loop {
        match tokens_iter.next() {
            Some(tk) => {
                match tk {
                    LineTerminator => {
                                                //to do ""
                        println!(";");
                        break},
                    LBrace => {
                        println!(";");
                        f_eq(tokens_iter);break},
                    _ => {print_token(tk);},
                }
            },
            None => break,
        };
    }
}
fn f_if(tokens_iter: &mut std::slice::Iter<Token>){
    loop {
        match tokens_iter.next() {
            Some(tk) => {
                match tk {
                    // TODO
                    //~ LineTerminator=>{
                        //~ //no scope,"if" that is 1 line
                        //~ println!("then");
                        //~ break},
                    LBrace => {
                        println!("then");
                        f_eq(tokens_iter);break},
                    _ => {print_token(tk);},
                }
            },
            None => break,
        };
    }
}
fn f_elif(tokens_iter: &mut std::slice::Iter<Token>){
    loop {
        match tokens_iter.next() {
            Some(tk) => {
                match tk {
                    //else
                    LineTerminator=>break,
                    //elseif
                    IF=> {print!("if");
                        f_if(tokens_iter);break},
                    //else
                    LBrace => {
                        println!("");
                        f_eq(tokens_iter);break},
                    _ => {print_token(tk);},
                }
            },
            None => break,
        };
    }
}
fn f_else(tokens_iter: &mut std::slice::Iter<Token>){
    loop {
        match tokens_iter.next() {
            Some(tk) => {
                match tk {
                    //endif
                    LineTerminator=>break,
                    //else or elseif
                    ELSE => {
                        print!("\nelse");
                        f_elif(tokens_iter);
                        break},
                    LBrace => {
                        println!("");
                        f_eq(tokens_iter);break},
                    _ => {print_token(tk);},
                }
            },
            None => break,
        };
    }
}
fn f_for(){}
fn f_while(tokens_iter: &mut std::slice::Iter<Token>){
    loop {     
        match tokens_iter.next() {
            Some(tk) => {
                match tk {
                    LBrace => {
                        println!("");
                        f_eq(tokens_iter);break},
                    _ => {print_token(tk);},
                }
            },
            None => break,
        };
    }
}
fn f_switch(){}

fn print_token(a: &c_lexer::token::Token){

    let b=match a {
        LBrace       => "{",
            RBrace       => "}",
            LParen       => "(",
            RParen       => ")",
            LBracket     => "[",
            RBracket     => "]",
            Semicolon    => ";",
            Assign       => "=",
            Lt           => "<",
            Gt           => ">",
            Minus        => "-",
            Tilde        => "~",
            Exclamation  => "!",
            Plus         => "+",
            Multi        => "*",
            Slash       => "/",
            Colon        => ":",
            QuestionMark => "?",
            Comma        => ",",
            Dot          => ".",
            SingleAnd    => "&",
            InclusiveOr  => "|",
            ExclusiveOr  => "^",
            Mod          => "%",
            //Identifier(IStr) => //IStr.as_str(),
            //NumericLiteral(Number)
            //StringLiteral(String)
            //FuncName    => "__func__",
            SIZEOF      => "sizeof",
            PtrOp       => "->",
            IncOp       => "++",
            DecOp       => "--",
            LeftOp      => "<<",
            RightOp     => ">>",
            LeOp        => "<=",
            GeOp        => ">=",
            EqOp        => "==",
            NeOp        => "!=",
            AndOp       => "&&",
            OrOp        => "||",
            MulAssign   => "*=",
            DivAssign   => "/=",
            ModAssign   => "%=",
            AddAssign   => "+=",
            SubAssign   => "-=",
            LeftAssign  => "<<=",
            RightAssign => ">>=",
            AndAssign   => "&=",
            XorAssign   => "^=",
            OrAssign    => "|=",
            //ELLIPSIS                    => ...
            LineTerminator => "\n",
            BREAK => "beark",
            RETURN => "return",
            CONTINUE => "continue",
        _ =>{" "},
    };
    match a {
        Identifier(iii) => {print!("{}",iii.as_str());},
        NumericLiteral(n)=> {
            print!("{}",n.integer);},
    _ =>{
    print!("{}",b);
    },
    }
}
