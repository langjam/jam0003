import gleam/option.{None, Option}
import lang/ast

pub type Type {
  InvalidTypeMarker
  IntegerType
  BooleanType
  StringType
}

pub type TypeAdapter(old_annotation_type, new_annotation_type) {
  TypeAdapter(
    encode_type: fn(old_annotation_type, Type) -> new_annotation_type,
    decode_type: fn(new_annotation_type) -> Type,
  )
}

pub fn typecheck_expr(
  adapter: TypeAdapter(old_annotation_type, new_annotation_type),
  expr: ast.Expr(old_annotation_type),
  constraint: Option(Type),
) -> ast.Expr(new_annotation_type) {
  let ast.Expr(old_annotation, old_data) = expr
  let augment_with_type = fn(
    new_data: ast.ExprData(new_annotation_type),
    inferred_type: Type,
  ) {
    ast.Expr(adapter.encode_type(old_annotation, inferred_type), new_data)
  }
  case old_data {
    ast.IntegerLiteral(val) ->
      augment_with_type(ast.IntegerLiteral(val), IntegerType)

    ast.BooleanLiteral(val) ->
      augment_with_type(ast.BooleanLiteral(val), BooleanType)

    ast.StringLiteral(val) ->
      augment_with_type(ast.StringLiteral(val), StringType)

    ast.UnOpExpr(op, child) -> {
      let new_child = typecheck_expr(adapter, child, constraint)
      let child_type =
        new_child.annotation
        |> adapter.decode_type()
      augment_with_type(
        ast.UnOpExpr(op, new_child),
        case op {
          ast.Neg ->
            case child_type {
              IntegerType -> IntegerType
              _other -> InvalidTypeMarker
            }
          ast.Not ->
            case child_type {
              BooleanType -> BooleanType
              _other -> InvalidTypeMarker
            }
          ast.Paren -> child_type
        },
      )
    }

    ast.BinOpExpr(op, lhs, rhs) -> {
      let new_lhs = typecheck_expr(adapter, lhs, None)
      let new_rhs = typecheck_expr(adapter, rhs, None)
      let lhs_type =
        new_lhs.annotation
        |> adapter.decode_type()
      let rhs_type =
        new_rhs.annotation
        |> adapter.decode_type()
      augment_with_type(
        ast.BinOpExpr(op, new_lhs, new_rhs),
        case op {
          ast.Add | ast.Sub | ast.Mul | ast.Div | ast.Mod ->
            case #(lhs_type, rhs_type) {
              #(IntegerType, IntegerType) -> IntegerType
              _other -> InvalidTypeMarker
            }
          ast.And | ast.Or ->
            case #(lhs_type, rhs_type) {
              #(BooleanType, BooleanType) -> BooleanType
              _other -> InvalidTypeMarker
            }
          ast.Gt | ast.Lt | ast.Ge | ast.Le ->
            case #(lhs_type, rhs_type) {
              #(IntegerType, IntegerType) -> BooleanType
              _other -> InvalidTypeMarker
            }
          ast.Eq | ast.Ne ->
            case lhs_type == rhs_type {
              True -> BooleanType
              _other -> InvalidTypeMarker
            }
        },
      )
    }
  }
}
