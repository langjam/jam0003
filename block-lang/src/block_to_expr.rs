use crate::{expr::Expr, block::WrappedExpr};


pub fn block_to_expr(wrapped: &WrappedExpr) -> Result<&Expr<'static>, String> {
    match wrapped {
        WrappedExpr::Variable { bound } => Ok(&Expr::Variable),
        WrappedExpr::Lambda { bind_entity, expr_entity, formed: Some(expr) } => Ok(expr),
        WrappedExpr::Application { func_entity, args_entity, formed: Some(expr) } => Ok(expr),
        _ => Err("malformed expression".into()),
    }
}
