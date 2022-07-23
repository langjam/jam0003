pub type UnOp {
  Neg
  Not
  Paren
}

pub type BinOp {
  Add
  Sub
  Mul
  Div
  Mod
  And
  Or
  Gt
  Lt
  Ge
  Le
  Eq
  Ne
}

pub type Expr(annotation_type) {
  Expr(annotation: annotation_type, data: ExprData(annotation_type))
}

pub type ExprData(annotation_type) {
  IntegerLiteral(val: Int)
  BooleanLiteral(val: Bool)
  StringLiteral(val: String)
  UnOpExpr(op: UnOp, child: Expr(annotation_type))
  BinOpExpr(op: BinOp, lhs: Expr(annotation_type), rhs: Expr(annotation_type))
}
