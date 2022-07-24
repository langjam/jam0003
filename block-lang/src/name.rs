use std::{cell::RefCell, fmt};

use bytecheck::CheckBytes;
use rkyv::{with::Map, Archive, Deserialize, Serialize};

use crate::expr::Expr;
use hashdb::{ArchiveDeserializer, ArchiveStore, HashType, LinkArena, TypeStore};

/// Object in disp that has a name
#[derive(Debug, Hash, Archive, Serialize, Deserialize)]
#[archive_attr(derive(bytecheck::CheckBytes))]
#[archive(bound(serialize = "__S: ArchiveStore", deserialize = "__D: ArchiveDeserializer<'e>"))]
pub enum NamedObject<'e> {
	Namespace(
		#[with(HashType)]
		#[omit_bounds]
		&'e Namespace<'e>,
	),
	Expr(
		#[with(HashType)]
		#[omit_bounds]
		&'e Expr<'e>,
	),
}

impl<'e> fmt::Display for NamedObject<'e> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
			Self::Namespace(namespace) => write!(f, "{namespace}"),
			Self::Expr(expr) => write!(f, "{expr}"),
		}
    }
}

/// Object that contains a named object and its name. Both name and expression are reverse-linked to this object.
#[derive(Debug, Hash, Archive, Serialize, Deserialize)]
#[archive_attr(derive(bytecheck::CheckBytes))]
#[archive(bound(serialize = "__S: ArchiveStore", deserialize = "__D: ArchiveDeserializer<'e>"))]
pub struct Name<'e> {
	#[with(HashType)]
	#[omit_bounds]
	pub string: &'e String,
	pub object: NamedObject<'e>,
}
impl<'e> Name<'e> {
	pub fn new(
		name: impl Into<String>,
		expr: &'e Expr<'e>,
		exprs: &'e LinkArena<'e>,
	) -> &'e Name<'e> {
		exprs.add(Self {
			string: exprs.add(name.into()),
			object: NamedObject::Expr(expr),
		})
	}
}
impl<'e> fmt::Display for Name<'e> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.string, self.object)
    }
}

#[derive(Default)]
pub struct NamespaceMut<'e> {
	namespace: RefCell<Namespace<'e>>,
}
impl<'e> NamespaceMut<'e> {
	pub fn new() -> Self { Self::default() }
	pub fn add(&self, name: impl Into<String>, expr: &'e Expr<'e>, exprs: &'e LinkArena<'e>) {
		self.namespace.borrow_mut().add(name, expr, exprs)
	}
	pub fn find<P: FnMut(&&&'e Name<'e>) -> bool>(&self, predicate: P) -> Option<&'e Name<'e>> {
		self.namespace.borrow().items.iter().find(predicate).as_deref().map(|n|*n)
	}
	pub fn for_each<F: FnMut(&&'e Name<'e>)>(&self, func: F) {
		self.namespace.borrow().items.iter().for_each(func)
	}
	pub fn store_inner(&self, exprs: &'e LinkArena<'e>) -> &'e Namespace<'e> {
		exprs.add(self.namespace.borrow().clone())
	}
	pub fn extend(&self, namespace: &Namespace<'e>) {
		self.namespace.borrow_mut().items.extend(&namespace.items);
	}
}
impl<'e> From<Namespace<'e>> for NamespaceMut<'e> {
    fn from(namespace: Namespace<'e>) -> Self {
        NamespaceMut { namespace: RefCell::new(namespace) }
    }
}

// A list of names
#[derive(Clone, Hash, Debug, Archive, Serialize, Deserialize, Default)]
#[archive_attr(derive(CheckBytes))]
#[archive(bound(serialize = "__S: ArchiveStore", deserialize = "__D: ArchiveDeserializer<'e>"))]
pub struct Namespace<'e> {
	#[with(Map<HashType>)]
	#[omit_bounds]
	pub items: Vec<&'e Name<'e>>,
}
impl<'e> Namespace<'e> {
	pub fn new() -> Self { Namespace::default() }
	pub fn add_name(&mut self, name: &'e Name<'e>) {
		self.items.push(name);
	}
	pub fn add(&mut self, name: impl Into<String>, expr: &'e Expr<'e>, exprs: &'e LinkArena<'e>) {
		self.items.push(Name::new(name, expr, exprs))
	}
}
impl<'e> fmt::Display for Namespace<'e> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for item in &self.items {
			match item.object {
				NamedObject::Expr(expr) => writeln!(f, "{expr}")?,
				_ => {},
			}
		}
		Ok(())
    }
}
