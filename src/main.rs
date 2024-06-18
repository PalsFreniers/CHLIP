#[derive(Debug, PartialEq, Eq, Clone)]
struct Location {
    file: &'static str,
    line: u64,
    col: u64,
}

#[derive(Debug, PartialEq, Clone)]
enum Token {
    Identifier(Location, String),
    ParentesisClose(Location),
    ParentesisOpen(Location),
    ReturnOperator(Location),
    Integer(Location, u64),
    BracketClose(Location),
    CrossKeyword(Location),
    ChipKeyword(Location),
    BracketOpen(Location),
    Semicolon(Location),
    Colon(Location),
    Coma(Location),
}

/*
 * chip
 */

#[derive(Debug, Clone)]
struct Instruction {
    name: String,
    inputs: Vec<String>,
    output: Vec<String>,
}

#[derive(Debug, Clone)]
struct Cross {
    name: String,
    size: u64,
}

#[derive(Debug, Clone)]
struct Chip {
    name: String,
    inputs: Vec<Cross>,
    output: Vec<Cross>,
    crosss: Vec<Cross>,
    instr: Vec<Instruction>,
}

fn match_token(tok: &Token, _typ: Token) -> Result<(String, u64), String> {
    let t: Token = tok.clone();
    if matches!(t.clone(), _typ) {
        match t {
            Token::Integer(_, i) => { return Ok((String::from(""), i)); }
            Token::Identifier(_, s) => { return Ok((s, 0)); }
            _ => { return Ok((String::from(""), 0)); }
        }
    }
    let loc: Location;
    match t.clone() {
        Token::Integer(l, _) => { loc = l; }
        Token::Identifier(l, _) => { loc = l; }
        Token::ParentesisClose(l) => { loc = l; }
        Token::ParentesisOpen(l) => { loc = l; }
        Token::ReturnOperator(l) => { loc = l; }
        Token::BracketClose(l) => { loc = l; }
        Token::CrossKeyword(l) => { loc = l; }
        Token::ChipKeyword(l) => { loc = l; }
        Token::BracketOpen(l) => { loc = l; }
        Token::Semicolon(l) => { loc = l; }
        Token::Colon(l) => { loc = l; }
        Token::Coma(l) => { loc = l; }
    }
    return Err(format!("[ERROR] {}:{}:{} => Bad Token {:?}", loc.file, loc.line, loc.col, t));
}

const LOC_FILL: Location = Location {
    file: "",
    line: 0,
    col: 0,
};

fn parse_inst(tokens: &mut Vec<Token>) -> Result<Instruction, String> {
    let nme: String;
    let mut inp: Vec<String> = Vec::new();
    let mut outp: Vec<String> = Vec::new();
    
    let cur = tokens.pop().unwrap();
    match match_token(&cur, Token::Identifier(LOC_FILL, String::from(""))) {
        Ok(s) => { nme = s.0; }
        Err(e) => { return Err(e); }
    }
    let cur = tokens.pop().unwrap();
    match match_token(&cur, Token::ParentesisOpen(LOC_FILL)) {
        Ok(_) => {}
        Err(e) => { return Err(e); }
    }
    loop {
        let cur = tokens.pop().unwrap();
        match cur {
            Token::Identifier(_, ref s) => { inp.push(s.clone()); }
            _ => { match match_token(&cur, Token::CrossKeyword(LOC_FILL)) {Ok(_) => {} Err(e) => { return Err(e); }} }
        }
        let cur: Token = tokens.pop().unwrap();    
        match match_token(&cur, Token::Coma(LOC_FILL)) {
            Ok(_) => {}
            Err(_) => {
                match match_token(&cur, Token::ParentesisClose(LOC_FILL)) {
                    Ok(_) => {}
                    Err(e) => { return Err(e); }
                }
            }
        }
        if matches!(cur, Token::ParentesisClose(_)) {
            break;
        }
    }
    let cur: Token = tokens.pop().unwrap(); 
    match match_token(&cur, Token::ReturnOperator(LOC_FILL)) {
        Ok(_) => {}
        Err(e) => { return Err(e); }
    }
    loop {
        let cur = tokens.pop().unwrap();
        match cur {
            Token::Identifier(_, ref s) => { outp.push(s.clone()); }
            _ => { match match_token(&cur, Token::CrossKeyword(LOC_FILL)) {Ok(_) => {} Err(e) => { return Err(e); }} }
        }
        let cur: Token = tokens.pop().unwrap();    
        match match_token(&cur, Token::Coma(LOC_FILL)) {
            Ok(_) => {}
            Err(_) => {
                match match_token(&cur, Token::Semicolon(LOC_FILL)) {
                    Ok(_) => {}
                    Err(e) => { return Err(e); }
                }
            }
        }
        if matches!(cur, Token::Semicolon(_)) {
            break;
        }
    }
    return Ok(Instruction {
        name: nme,
        inputs: inp,
        output: outp,
    });
}

fn parse_cross(tokens: &mut Vec<Token>) -> Result<Cross, String> {
    let nme: String;
    let sze: u64;

    let cur: Token = tokens.pop().unwrap();
    match match_token(&cur, Token::Identifier(LOC_FILL, String::from(""))) {
        Ok(s) => { nme = s.0; }
        Err(e) => { return Err(e); }
    }
    let cur: Token = tokens.pop().unwrap();    
    match match_token(&cur, Token::Colon(LOC_FILL)) {
        Ok(_) => {}
        Err(e) => { return Err(e); }
    }
    let cur: Token = tokens.pop().unwrap();    
    match match_token(&cur, Token::Integer(LOC_FILL, 0)) {
        Ok(s) => { sze = s.1; }
        Err(e) => { return Err(e); }
    }
    return Ok(Cross {
        name: nme,
        size: sze,
    });
}

fn parse_chip(tokens: &mut Vec<Token>) -> Result<Chip, String> {
    let nme: String;
    let mut inp: Vec<Cross> = Vec::new();
    let mut outp: Vec<Cross> = Vec::new();
    let mut cross: Vec<Cross> = Vec::new();
    let mut inst: Vec<Instruction> = Vec::new();

    let cur: Token = tokens.pop().unwrap();    
    match match_token(&cur, Token::Identifier(LOC_FILL, String::from(""))) {
        Ok(s) => { nme = s.0; }
        Err(e) => { return Err(e); }
    }
    let cur: Token = tokens.pop().unwrap();    
    match match_token(&cur, Token::ParentesisOpen(LOC_FILL)) {
        Ok(_) => {}
        Err(e) => { return Err(e); }
    }
    loop {
        match parse_cross(tokens) {
            Ok(cross) => { inp.push(cross); }
            Err(e) => { return Err(e); }
        }
        let cur: Token = tokens.pop().unwrap();    
        match match_token(&cur, Token::Coma(LOC_FILL)) {
            Ok(_) => {}
            Err(_) => {
                match match_token(&cur, Token::ParentesisClose(LOC_FILL)) {
                    Ok(_) => {}
                    Err(e) => { return Err(e); }
                }
            }
        }
        if matches!(cur, Token::ParentesisClose(_)) {
            break;
        }
    }
    let cur: Token = tokens.pop().unwrap(); 
    match match_token(&cur, Token::ReturnOperator(LOC_FILL)) {
        Ok(_) => {}
        Err(e) => { return Err(e); }
    }
    loop {
        match parse_cross(tokens) {
            Ok(cross) => { outp.push(cross); }
            Err(e) => { return Err(e); }
        }
        let cur: Token = tokens.pop().unwrap();    
        match match_token(&cur, Token::Coma(LOC_FILL)) {
            Ok(_) => {}
            Err(_) => {
                match match_token(&cur, Token::BracketOpen(LOC_FILL)) {
                    Ok(_) => {}
                    Err(e) => { return Err(e); }
                }
            }
        }
        if matches!(cur, Token::BracketOpen(_)) {
            break;
        }
    }
    match match_token(&cur, Token::BracketOpen(LOC_FILL)) {
        Ok(_) => {}
        Err(e) => { return Err(e); }
    }
    loop {
        let cur: Token = tokens.last().unwrap().to_owned();
        if matches!(cur, Token::BracketClose(_)) {
            break;
        }
        match cur {
            Token::CrossKeyword(_) => {
                tokens.pop();
                match parse_cross(tokens) {
                    Ok(c) => { cross.push(c); }
                    Err(e) => { return Err(e); }
                }
                let cur: Token = tokens.pop().unwrap();
                match match_token(&cur, Token::Semicolon(LOC_FILL)) {
                    Ok(_) => {}
                    Err(e) => { return Err(e); }
                }
            }
            Token::Identifier(_, _) => {
                match parse_inst(tokens) {
                    Ok(i) => { inst.push(i); }
                    Err(e) => { return Err(e); }
                }
            }
            _ => { match match_token(&cur, Token::CrossKeyword(LOC_FILL)) {Ok(_) => {} Err(e) => { return Err(e); }} }
        }
    }
    match match_token(&cur, Token::BracketClose(LOC_FILL)) {
        Ok(_) => {}
        Err(e) => { return Err(e); }
    }
    return Ok(Chip {
        name: nme,
        inputs: inp,
        output: outp,
        crosss: cross,
        instr: inst,
    });
}

fn parse(mut tokens: Vec<Token>) -> Result<Chip, String> {
    let cur: Token = tokens.pop().unwrap();
    match match_token(&cur, Token::ChipKeyword(LOC_FILL)) {
        Ok(_) => { return parse_chip(&mut tokens); }
        Err(e) => { return Err(e); }
    }
}

fn exec(name: String, chips: &Vec<Chip>, ins: Vec<u8>) -> Vec<u8> {
    if name == String::from("nand") {
        if ins.len() != 2 {
            panic!("[Error] => tried to call nand with bad number of parameters");
        }
        let a = ins[0];
        let b = ins[1];
        let mut out: u8 = a & b;
        out = (!out) & 0x1;
        return vec![out];
    }
    let mut out: Vec<u8> = Vec::new();
    let mut crs: Vec<u8> = Vec::new();
    let tmp: Option<&Chip> = chips.into_iter().find(|c| c.name == name);
    let me: Chip;
    match tmp {
        Some(c) => { me = (*c).clone(); }
        None => { panic!("[ERROR] => unable to get chip : '{}'", name); }
    }
    assert!(ins.len() == me.inputs.len());
    out.resize(me.output.len(), 0);
    crs.resize(me.crosss.len(), 0);
    for i in me.instr {
        let mut iins: Vec<u8> = Vec::new();
        for inp in i.inputs {
            let tmp: Option<(usize, Cross)> = me.inputs.clone().into_iter().enumerate().find(|c| c.1.name == inp);
            match tmp {
                Some(c) => { iins.push(ins[c.0]); }
                None => {
                    let tmp: Option<(usize, Cross)> = me.crosss.clone().into_iter().enumerate().find(|c| c.1.name == inp);
                    match tmp {
                        Some(c) => { iins.push(crs[c.0]); }
                        None => { panic!("[ERROR] => unable to get input {} for chip {}", inp, me.name); }
                    }
                }
            }
        }
        let t = exec(i.name.clone(), chips, iins);
        for oup in i.output.into_iter().enumerate() {
            let tmp: Option<(usize, Cross)> = me.output.clone().into_iter().enumerate().find(|c| c.1.name == oup.1);
            match tmp {
                Some(c) => { out[c.0] = t[oup.0]; }
                None => {
                    let tmp: Option<(usize, Cross)> = me.crosss.clone().into_iter().enumerate().find(|c| c.1.name == oup.1);
                    match tmp {
                        Some(c) => { crs[c.0] = t[oup.0]; }
                        None => { panic!("[ERROR] => unable to get input {} for chip {}", oup.1, me.name); }
                    }
                }
            }
        }
    }
    return out;
}

fn main() {
    let mut toks_not: Vec<Token> = vec![
        Token::ChipKeyword(Location {file: "main.chp", line: 0, col: 0}),
        Token::Identifier(Location {file: "main.chp", line: 1, col: 0}, String::from("not")),
        Token::ParentesisOpen(Location {file: "main.chp", line: 2, col: 0}),
        Token::Identifier(Location {file: "main.chp", line: 3, col: 0}, String::from("A")),
        Token::Colon(Location {file: "main.chp", line: 4, col: 0}),
        Token::Integer(Location {file: "main.chp", line: 5, col: 0}, 1),
        Token::ParentesisClose(Location {file: "main.chp", line: 10, col: 0}),
        Token::ReturnOperator(Location {file: "main.chp", line: 11, col: 0}),
        Token::Identifier(Location {file: "main.chp", line: 12, col: 0}, String::from("Z")),
        Token::Colon(Location {file: "main.chp", line: 13, col: 0}),
        Token::Integer(Location {file: "main.chp", line: 14, col: 0}, 1),
        Token::BracketOpen(Location {file: "main.chp", line: 15, col: 0}),
        Token::Identifier(Location {file: "main.chp", line: 16, col: 0}, String::from("nand")),
        Token::ParentesisOpen(Location {file: "main.chp", line: 17, col: 0}),
        Token::Identifier(Location {file: "main.chp", line: 18, col: 0}, String::from("A")),
        Token::Coma(Location {file: "main.chp", line: 19, col: 0}),
        Token::Identifier(Location {file: "main.chp", line: 20, col: 0}, String::from("A")),
        Token::ParentesisClose(Location {file: "main.chp", line: 21, col: 0}),
        Token::ReturnOperator(Location {file: "main.chp", line: 22, col: 0}),
        Token::Identifier(Location {file: "main.chp", line: 23, col: 0}, String::from("Z")),
        Token::Semicolon(Location {file: "main.chp", line: 24, col: 0}),
        Token::BracketClose(Location {file: "main.chp", line: 22, col: 0}),
    ];
    let mut toks_and: Vec<Token> = vec![
        Token::ChipKeyword(Location {file: "main.chp", line: 0, col: 0}),
        Token::Identifier(Location {file: "main.chp", line: 1, col: 0}, String::from("and")),
        Token::ParentesisOpen(Location {file: "main.chp", line: 2, col: 0}),
        Token::Identifier(Location {file: "main.chp", line: 3, col: 0}, String::from("A")),
        Token::Colon(Location {file: "main.chp", line: 4, col: 0}),
        Token::Integer(Location {file: "main.chp", line: 5, col: 0}, 1),
        Token::Coma(Location {file: "main.chp", line: 123, col: 0}),
        Token::Identifier(Location {file: "main.chp", line: 3, col: 0}, String::from("B")),
        Token::Colon(Location {file: "main.chp", line: 4, col: 0}),
        Token::Integer(Location {file: "main.chp", line: 5, col: 0}, 1),
        Token::ParentesisClose(Location {file: "main.chp", line: 10, col: 0}),
        Token::ReturnOperator(Location {file: "main.chp", line: 11, col: 0}),
        Token::Identifier(Location {file: "main.chp", line: 12, col: 0}, String::from("Z")),
        Token::Colon(Location {file: "main.chp", line: 13, col: 0}),
        Token::Integer(Location {file: "main.chp", line: 14, col: 0}, 1),
        Token::BracketOpen(Location {file: "main.chp", line: 15, col: 0}),
        Token::CrossKeyword(Location {file: "main.chp", line: 15, col: 0}),
        Token::Identifier(Location {file: "main.chp", line: 15, col: 0}, String::from("X")),
        Token::Colon(Location {file: "main.chp", line: 4, col: 0}),
        Token::Integer(Location {file: "main.chp", line: 5, col: 0}, 1),
        Token::Semicolon(Location {file: "main.chp", line: 4, col: 0}),
        Token::Identifier(Location {file: "main.chp", line: 16, col: 0}, String::from("nand")),
        Token::ParentesisOpen(Location {file: "main.chp", line: 17, col: 0}),
        Token::Identifier(Location {file: "main.chp", line: 18, col: 0}, String::from("A")),
        Token::Coma(Location {file: "main.chp", line: 19, col: 0}),
        Token::Identifier(Location {file: "main.chp", line: 20, col: 0}, String::from("B")),
        Token::ParentesisClose(Location {file: "main.chp", line: 21, col: 0}),
        Token::ReturnOperator(Location {file: "main.chp", line: 22, col: 0}),
        Token::Identifier(Location {file: "main.chp", line: 23, col: 0}, String::from("X")),
        Token::Semicolon(Location {file: "main.chp", line: 24, col: 0}),
        Token::Identifier(Location {file: "main.chp", line: 16, col: 0}, String::from("not")),
        Token::ParentesisOpen(Location {file: "main.chp", line: 17, col: 0}),
        Token::Identifier(Location {file: "main.chp", line: 18, col: 0}, String::from("X")),
        Token::ParentesisClose(Location {file: "main.chp", line: 21, col: 0}),
        Token::ReturnOperator(Location {file: "main.chp", line: 22, col: 0}),
        Token::Identifier(Location {file: "main.chp", line: 23, col: 0}, String::from("Z")),
        Token::Semicolon(Location {file: "main.chp", line: 24, col: 0}),
        Token::BracketClose(Location {file: "main.chp", line: 22, col: 0}),
    ];
    toks_not.reverse();
    toks_and.reverse();
    let mut chip: Vec<Chip> = Vec::new();
    match parse(toks_not) {
        Err(str) => {
            panic!("{}", str);
        }
        Ok(chp) => {
            chip.push(chp);
        }
    }
    match parse(toks_and) {
        Err(str) => {
            panic!("{}", str);
        }
        Ok(chp) => {
            chip.push(chp);
        }
    }
    let out: Vec<u8> = exec(String::from("and"), &chip, vec![0, 0]);
    println!("{:?}", out);
}
