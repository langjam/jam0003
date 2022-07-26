///! Bindings allow for more easy manipulation of Lambda abstractions during beta reduction and creation
use std::fmt;
use thiserror::Error;

use rkyv::{Archive, Deserialize, Serialize};
use bytecheck::CheckBytes;

use hashdb::{ArchiveDeserializer, ArchiveStore, HashType, TypeStorable, TypeStore};

use super::{Expr, LambdaError};

/// PointerTree represents where the variables are in a Lambda abstraction.
#[derive(Clone, Hash, PartialEq, Eq, Debug, Archive, Serialize, Deserialize)]
#[archive_attr(derive(CheckBytes, Debug))]
#[archive(bound(serialize = "__S: ArchiveStore", deserialize = "__D: ArchiveDeserializer<'a>"))]
pub enum Binding<'a> {
	None,
	End,
	Branch(
		#[with(HashType)]
		#[omit_bounds]
		&'a Binding<'a>,
		#[with(HashType)]
		#[omit_bounds]
		&'a Binding<'a>,
	),
}
impl<'a> Binding<'a> {
	pub const NONE: &'static Binding<'static> = &Binding::None;
	pub const END: &'static Binding<'static> = &Binding::End;
	pub fn left(p: &'a Binding<'a>, arena: &'a impl TypeStore<'a>) -> &'a Binding<'a> {
		arena.add(Binding::Branch(p, Self::NONE))
	}
	pub fn right(p: &'a Binding<'a>, arena: &'a impl TypeStore<'a>) -> &'a Binding<'a> {
		arena.add(Binding::Branch(Self::NONE, p))
	}
	pub fn branch(l: &'a Binding<'a>, r: &'a Binding<'a>, arena: &'a impl TypeStore<'a>) -> &'a Binding<'a> {
		arena.add(Binding::Branch(l, r))
	}
	pub fn branch_reduce(l: &'a Binding<'a>, r: &'a Binding<'a>, arena: &'a impl TypeStore<'a>) -> &'a Binding<'a> {
		if l == Self::NONE && r == Self::NONE {
			Self::NONE
		} else {
			arena.add(Binding::Branch(l, r))
		}
	}
}
impl<'a> fmt::Display for Binding<'a> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match *self {
			Binding::Branch(left, right) => match (*right == Binding::None, *left == Binding::None) {
				(true, true) => write!(f, "BOTH(NONE, NONE)")?,
				(true, false) => write!(f, "<{}", left)?,
				(false, true) => write!(f, ">{}", right)?,
				(false, false) => write!(f, "({},{})", left, right)?,
			},
			Binding::End => write!(f, ".")?,
			Binding::None => {}
		}
		Ok(())
	}
}

/// Errors associated with various operations on `BindTree`s
#[derive(Debug, Error)]
pub enum BindTreeError {
	#[error("there was already a bound variable in the tree")]
	AlreadyBound,
	#[error("attempted to bind variable at branch in bind tree")]
	InvalidBindLocation,
	#[error("attempted to split bind tree on leaf")]
	InvalidSplit,
}

// Associates a type with 
#[derive(Clone, Hash, PartialEq, Debug)]
pub enum BindTree<'a, T: TypeStorable> {
	None,
	End(T),
	Branch(&'a BindTree<'a, T>, &'a BindTree<'a, T>),
}
impl<'a, 'e, T: TypeStorable + 'e> BindTree<'a, T> {
	pub const NONE: &'e BindTree<'e, T> = &BindTree::None;
	pub fn split(&'a self) -> Result<(&'a Self, &'a Self), BindTreeError> {
		Ok(match self {
			BindTree::Branch(l, r) => (l, r),
			BindTree::None => (self, self),
			BindTree::End(_) => Err(BindTreeError::InvalidSplit)?,
		})
	}
	fn branch_new(left: &'a Self, right: &'a Self) -> Self {
		if let (BindTree::None, BindTree::None) = (left, right) {
			BindTree::None
		} else {
			BindTree::Branch(left, right)
		}
	}
	pub fn branch(left: &'a Self, right: &'a Self, binds: &'a impl TypeStore<'a>) -> &'a Self {
		binds.add(Self::branch_new(left, right))
	}
	pub fn left(&'a self, binds: &'a impl TypeStore<'a>) -> &'a Self {
		Self::branch(self, BindTree::NONE, binds)
	}
	pub fn right(&'a self, binds: &'a impl TypeStore<'a>) -> &'a Self {
		Self::branch(BindTree::NONE, self, binds)
	}
	pub fn end(val: T, binds: &'a impl TypeStore<'a>) -> &'a Self {
		binds.add(BindTree::End(val))
	}
}

impl<'a, T: fmt::Display + TypeStorable> fmt::Display for BindTree<'a, T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			BindTree::Branch(BindTree::None, right) => write!(f, ">{}", right)?,
			BindTree::Branch(left, BindTree::None) => write!(f, "<{}", left)?,
			BindTree::Branch(left, right) => write!(f, "({},{})", left, right)?,
			BindTree::End(val) => write!(f, "{}", val)?,
			BindTree::None => write!(f, "N")?,
		}
		Ok(())
	}
}

/// BindTree that can represent multiple lambda abstractions at once
pub type BindSubTree<'a> = BindTree<'a, usize>;
impl<'a, 'e> BindSubTree<'a> {
	/// Add PointerTree to ReplaceTree at certain abstraction level
	pub fn push_binding(self: &mut &'a Self, binds: &'a impl TypeStore<'a>, level: usize, pointer: &'e Binding<'e>) -> Result<(), BindTreeError> {
		*self = match (*self, pointer) {
			// If ReplaceTree is None, fill in pointer
			(tree, Binding::None) => tree,
			(BindTree::None, Binding::End) => Self::end(level, binds),
			(BindTree::None, Binding::Branch(l, r)) => {
				let (mut left, mut right) = (Self::NONE, Self::NONE);
				left.push_binding(binds, level, l)?;
				right.push_binding(binds, level, r)?;
				Self::branch(left, right, binds)
			}
			(BindTree::Branch(mut left, mut right), Binding::Branch(l, r)) => {
				left.push_binding(binds, level, l)?;
				right.push_binding(binds, level, r)?;
				Self::branch(left, right, binds)
			}
			(BindTree::End(_), _) => return Err(BindTreeError::AlreadyBound),
			(_, Binding::End) => return Err(BindTreeError::InvalidBindLocation),
		};
		Ok(())
	}
	/// Constructs PointerTree from ReplaceTree at certain abstraction level
	pub fn pop_binding(self: &mut &'a Self, binds: &'a impl TypeStore<'a>, level: usize, ptrs: &'e impl TypeStore<'e>) -> Result<&'e Binding<'e>, BindTreeError> {
		Ok(match self {
			BindTree::Branch(mut l, mut r) => {
				let left = l.pop_binding(binds, level, ptrs)?;
				let right = r.pop_binding(binds, level, ptrs)?;
				*self = Self::branch(l, r, binds);
				Binding::branch_reduce(left, right, ptrs)
			}
			BindTree::End(count) if level == *count => {
				*self = Self::NONE;
				Binding::END
			}
			_ => Binding::NONE,
		})
	}
}

/// BindTree that can associate an Expr with bound variables
pub type BindTypeTree<'a, 'e> = BindTree<'a, &'e Expr<'e>>;
impl<'a, 'e> BindTypeTree<'a, 'e> {
	// Push Binding and type Expr onto BindTypeTree
	pub fn push_binding(self: &mut &'a Self, bind: &'e Binding<'e>, bind_type: &'e Expr<'e>, binds: &'a impl TypeStore<'a>) -> Result<(), BindTreeError> {
		*self = match (*self, bind) {
			// If ReplaceTree is None, fill in pointer
			(tree, Binding::None) => tree,
			(BindTree::None, Binding::End) => Self::end(bind_type, binds),
			(BindTree::None, Binding::Branch(l, r)) => {
				let (mut left, mut right) = (Self::NONE, Self::NONE);
				left.push_binding(l, bind_type, binds)?;
				right.push_binding(r, bind_type, binds)?;
				Self::branch(left, right, binds)
			}
			(BindTree::Branch(mut left, mut right), Binding::Branch(l, r)) => {
				left.push_binding(l, bind_type, binds)?;
				right.push_binding(r, bind_type, binds)?;
				Self::branch(left, right, binds)
			}
			(BindTree::End(_), _) => return Err(BindTreeError::AlreadyBound),
			(_, Binding::End) => return Err(BindTreeError::InvalidBindLocation),
		};
		Ok(())
	}
}

// Index = 0 means no lambda history (tree should BindTree::None)
// Index > 0 means there is history
#[derive(PartialEq, Debug, Clone)]
pub struct BindIndex<'a> {
	pub index: usize,
	pub tree: &'a BindSubTree<'a>,
}

impl<'a, 'e> BindIndex<'a> {
	pub const DEFAULT: BindIndex<'a> = BindIndex::new(0, BindTree::NONE);
	pub const fn new(index: usize, tree: &'a BindSubTree<'a>) -> Self {
		Self { index, tree }
	}
	/// Split on branch of BindTree
	pub fn split(&self) -> Result<(Self, Self), BindTreeError> {
		let (left, right) = self.tree.split()?;
		Ok((BindIndex::new(self.index, left), BindIndex::new(self.index, right)))
	}
	// Join two BindIndexs of same index
	pub fn join(left: BindIndex<'a>, right: BindIndex<'a>, binds: &'a impl TypeStore<'a>) -> BindIndex<'a> {
		debug_assert_eq!(left.index, right.index);
		BindIndex::new(left.index, BindTree::branch(left.tree, right.tree, binds))
	}
	/// Push Binding onto BindIndex
	pub fn push_binding(&mut self, pointer: &'e Binding<'e>, binds: &'a impl TypeStore<'a>) -> Result<(), BindTreeError> {
		let BindIndex { index, tree } = self;
		*index += 1;
		tree.push_binding(binds, *index, pointer)?;
		Ok(())
	}
	/// Pop Binding from BindIndex
	pub fn pop_binding(&mut self, binds: &'a impl TypeStore<'a>, ptrs: &'e impl TypeStore<'e>) -> Result<&'e Binding<'e>, LambdaError> {
		let BindIndex { index, tree } = self;
		if *index == 0 {
			return Err(LambdaError::BindingLevelMismatch);
		}
		let ret = tree.pop_binding(binds, *index, ptrs)?;
		*index -= 1;
		Ok(ret)
	}
	/// Build BindIndex from nested Lambda expressions
	pub fn push_lambda(&mut self, expr: &'a Expr<'a>, binds: &'a impl TypeStore<'a>) -> Result<&'a Expr<'a>, BindTreeError> {
		Ok(if let Expr::Lambda { bind: pointer_tree, expr } = expr {
			let pushed_expr = self.push_lambda(expr, binds)?;
			self.push_binding(pointer_tree, binds)?;
			pushed_expr
		} else {
			&expr
		})
	}
	/// Creates nested Lambda expressions from BindIndex
	#[allow(dead_code)]
	pub fn pop_lambda(&mut self, expr: &'e Expr<'e>, binds: &'a impl TypeStore<'a>, exprs: &'e impl TypeStore<'e>) -> Result<&'e Expr<'e>, LambdaError> {
		let pointer_tree = self.pop_binding(binds, exprs)?;
		let popped_expr = if self.index == 0 { &expr } else { self.pop_lambda(expr, binds, exprs)? };
		Ok(Expr::lambda(pointer_tree, popped_expr, exprs))
	}
}

impl<'a> fmt::Display for BindIndex<'a> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}/{}", self.tree, self.index)
	}
}

#[test]
fn test_replace_tree() {
	use crate::name::NamespaceMut;
	use hashdb::LinkArena;

	use Binding as B;
	let binds = &LinkArena::new();
	let exprs = &LinkArena::new();
	let mut r = BindIndex::DEFAULT;
	println!("start: [{}]", r);

	let lambda = crate::parse::parse("[x y z w] x (y z) w", &mut NamespaceMut::new(), exprs).unwrap();
	println!("lambda: {}", lambda);
	let expr = r.push_lambda(&lambda, binds).unwrap();
	println!("after push: {} : {}", expr, r);
	let lambda_2 = r.pop_lambda(expr, binds, exprs).unwrap();
	println!("after pop: {}", lambda_2);
	assert_eq!(lambda, lambda_2);

	// Test Split & Join
	let r = BindIndex::DEFAULT;
	test_split(binds, r).unwrap();

	let _pts = &LinkArena::new();
	let mut r = BindIndex::DEFAULT;
	r.push_binding(&B::END, binds).unwrap();
	test_split(binds, r).unwrap_err();

	// let r = &mut ReduceArena(, 0);
	// test_split(r, db).unwrap(); // This will error
	let mut r = BindIndex::DEFAULT;
	r.push_binding(&B::left(B::END, exprs), binds).unwrap();
	test_split(binds, r).unwrap();

	let mut r = BindIndex::DEFAULT;
	r.push_binding(&B::branch(B::right(B::END, exprs), B::END, exprs), binds).unwrap();
	test_split(binds, r).unwrap();
}
#[allow(dead_code)]
fn test_split<'a>(binds: &'a impl TypeStore<'a>, r: BindIndex<'a>) -> Result<(), BindTreeError> {
	print!("split [{}] ", r);
	let (left, right) = r.split().map_err(|e| {
		println!("split err: {}", e);
		e
	})?;
	print!(" - ([{}] [{}])", left, right);
	let r_after = BindIndex::join(left, right, binds);
	println!(" = [{}]", r);
	assert_eq!(r, r_after);
	Ok(())
}
