use crate::data::{
    source::{Source, SourceError},
    token::TokenData,
};

use super::data::{PartialLexInput, PartialLexResult};

pub fn lex(input: &Source) -> Result<Vec<TokenData>, SourceError> {
    let success = token(PartialLexInput {
        offset: 0,
        remaining: input.text.clone(),
    })?;
    Ok(vec![success.token])
}

fn token(input: PartialLexInput) -> PartialLexResult {
    Err(SourceError {
        offset: input.offset,
        message: "Lexer unimplemented".to_string(),
    })
}
