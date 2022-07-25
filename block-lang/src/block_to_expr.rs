use crate::{expr::Expr, block::WrappedExpr};

pub fn block_to_expr(wrapped: &WrappedExpr) -> Result<&Expr<'static>, Box<dyn std::error::Error>> {
    match wrapped {
        WrappedExpr::Variable { formed: (expr, _) }
        | WrappedExpr::Lambda { formed: Some((expr, _)), .. }
        | WrappedExpr::Application { formed: Some((expr, _)), .. } => Ok(expr),
        _ => Err(format!("unformed expression for {wrapped:?}"))?,
    }
}
