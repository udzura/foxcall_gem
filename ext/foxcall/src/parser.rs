use std::cell::Cell;

use crate::foxcall::Insn;

const RU: char = 'ル';
const BI: char = 'ビ';
const II: char = 'ー';
const KA: char = 'カ';
const I: char = 'イ';
const GI: char = 'ギ';

pub fn parse(src: &str) -> Result<Vec<Cell<Insn>>, String> {
    let mut tokens = tokenize(src)?;
    calculate_jumps(&mut tokens)?;
    Ok(tokens)
}

// ルー	ポインタをインクリメント （右にずらす）
// ルルー	ポインタをデクリメント (左にずらす）
// ルルルー	ポインタの値をインクリメント
// ルルルルー	ポインタの値をデクリメント
// ルルルルルー	ポインタの値を出力
// ビー	入力から1バイト読み込んで、ポインタが指す値に代入
// ルビー	ポインタの指す値が0なら、後の カイギ までジャンプ(要するにwhile)
// カイギ	ポインタの指す値が0でなければ、前の ルビー までジャンプ

fn tokenize(src: &str) -> Result<Vec<Cell<Insn>>, String> {
    let mut tokens: Vec<Cell<Insn>> = vec![];
    let mut src = src.chars();
    while let Some(rune) = src.next() {
        let mut letters: Vec<char> = vec![];
        match rune {
            RU => {
                letters.push(RU);
                while let Some(next) = src.next() {
                    match next {
                        RU | BI => letters.push(next),
                        II => {
                            match &letters[..] {
                                [RU] => {
                                    tokens.push(Cell::new(Insn::IncrPrt));
                                }
                                [RU, RU] => {
                                    tokens.push(Cell::new(Insn::DecrPrt));
                                }
                                [RU, RU, RU] => {
                                    tokens.push(Cell::new(Insn::IncrVal));
                                }
                                [RU, RU, RU, RU] => {
                                    tokens.push(Cell::new(Insn::DecrVal));
                                }
                                [RU, RU, RU, RU, RU] => {
                                    tokens.push(Cell::new(Insn::Print));
                                }
                                [RU, BI] => {
                                    tokens.push(Cell::new(Insn::JumpFwd(0)));
                                }
                                _ => Err("invalid pattern of letters".to_string())?,
                            }
                            break;
                        }
                        _ => Err("invalid pattern of letters".to_string())?,
                    }
                }
            }
            BI => match src.next() {
                Some(II) => {
                    tokens.push(Cell::new(Insn::Scan));
                }
                _ => Err("invalid pattern of letters".to_string())?,
            },
            II => Err("invalid pattern of letters".to_string())?,
            KA => match (src.next(), src.next()) {
                (Some(I), Some(GI)) => {
                    tokens.push(Cell::new(Insn::JumpBwd(0)));
                }
                _ => Err("invalid pattern of letters".to_string())?,
            },
            I | GI => Err("invalid pattern of letters".to_string())?,
            _ => continue,
        }
    }
    Ok(tokens)
}

fn calculate_jumps(tokens: &mut [Cell<Insn>]) -> Result<(), String> {
    let mut stack: Vec<usize> = vec![];
    for i in 0..tokens.len() {
        match tokens[i].get() {
            Insn::JumpFwd(_) => {
                stack.push(i);
            }
            Insn::JumpBwd(_) => {
                let fwd = stack
                    .pop()
                    .ok_or_else(|| format!("unmatched JumpBwd at position {}", i))?;
                tokens[fwd].set(Insn::JumpFwd(i));
                tokens[i].set(Insn::JumpBwd(fwd));
            }
            _ => {}
        }
    }
    if let Some(&pos) = stack.last() {
        return Err(format!("unmatched JumpFwd at position {}", pos));
    }
    Ok(())
}
