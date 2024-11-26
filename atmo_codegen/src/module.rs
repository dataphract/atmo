use std::{
    collections::{BTreeMap, BTreeSet},
    fmt,
};

use atmo_core::nsid::{FullReference, Nsid};
use heck::{ToPascalCase, ToSnakeCase};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use crate::{
    enum_::{RustStringEnumDef, RustUnionEnumDef},
    rpc::RustRpcDef,
    struct_::RustStructDef,
};

pub struct Module {
    // Fully-qualified module path.
    path: ModulePath,
    // Set of submodules.
    submodules: BTreeSet<ModulePath>,
    // Map of non-submodule items, keyed by NSID.
    items: BTreeMap<String, Item>,
}

impl Module {
    pub fn add_item(&mut self, name: String, item: Item) -> Result<(), NameCollision> {
        if let Some(existing) = self.items.get(&name) {
            match (&existing.ty, &item.ty) {
                (ItemTy::StringEnum(s1), ItemTy::StringEnum(s2)) => {
                    if s1 != s2 {
                        return Err(NameCollision(name));
                    } else {
                        return Ok(());
                    }
                }

                (ItemTy::UnionEnum(u1), ItemTy::UnionEnum(u2)) => {
                    if u1 != u2 {
                        return Err(NameCollision(name));
                    } else {
                        return Ok(());
                    }
                }

                _ => return Err(NameCollision(name)),
            }
        }

        self.items.insert(name, item);

        Ok(())
    }
}

#[derive(Debug)]
pub struct NameCollision(String);

impl fmt::Display for NameCollision {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "multiple items named {:?}", self.0)
    }
}

#[derive(Debug)]
pub struct Item {
    pub ty: ItemTy,
}

impl Item {
    pub fn new(ty: ItemTy) -> Self {
        Item { ty }
    }
}

impl ToTokens for Item {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ty = &self.ty;

        quote! {
            #ty
        }
        .to_tokens(tokens)
    }
}

#[derive(Debug)]
pub enum ItemTy {
    Rpc(RustRpcDef),
    Struct(RustStructDef),
    StringEnum(RustStringEnumDef),
    UnionEnum(RustUnionEnumDef),
}

impl From<RustRpcDef> for ItemTy {
    #[inline]
    fn from(rpc: RustRpcDef) -> Self {
        ItemTy::Rpc(rpc)
    }
}

impl From<RustStructDef> for ItemTy {
    #[inline]
    fn from(s: RustStructDef) -> Self {
        ItemTy::Struct(s)
    }
}

impl From<RustStringEnumDef> for ItemTy {
    #[inline]
    fn from(s: RustStringEnumDef) -> Self {
        ItemTy::StringEnum(s)
    }
}

impl From<RustUnionEnumDef> for ItemTy {
    #[inline]
    fn from(u: RustUnionEnumDef) -> Self {
        ItemTy::UnionEnum(u)
    }
}

impl ToTokens for ItemTy {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match &self {
            ItemTy::Rpc(r) => r.to_tokens(tokens),
            ItemTy::Struct(s) => s.to_tokens(tokens),
            ItemTy::StringEnum(e) => e.to_tokens(tokens),
            ItemTy::UnionEnum(e) => e.to_tokens(tokens),
        }
    }
}

#[derive(Default)]
pub struct ModuleTree {
    // Flattened module tree, keyed by fully-qualified module path.
    modules: BTreeMap<ModulePath, Module>,
}

impl ModuleTree {
    pub fn get_or_create_mut(&mut self, path: &ModulePath) -> &mut Module {
        self.create_ancestors(path);
        self.create_only(path)
    }

    fn create_only(&mut self, path: &ModulePath) -> &mut Module {
        self.modules.entry(path.clone()).or_insert_with(|| Module {
            path: path.clone(),
            submodules: BTreeSet::new(),
            items: BTreeMap::new(),
        })
    }

    fn create_ancestors(&mut self, path: &ModulePath) {
        let mut child = path.clone();
        while let Some(parent_path) = child.parent() {
            let parent = self.create_only(&parent_path);
            parent.submodules.insert(child.clone());
            child = parent_path;
        }
    }

    // There's probably a way to do this in-place (i.e. without returning a new TokenStream for each
    // module) but it really doesn't matter.
    fn emit_module(&self, path: &ModulePath) -> TokenStream {
        let module = self.modules.get(path).unwrap();

        if module.submodules.is_empty() && module.items.is_empty() {
            return TokenStream::new();
        }

        let name = quote::format_ident!("{}", path.name());
        let items = module.items.values();
        let submodules = module.submodules.iter().map(|sm| self.emit_module(sm));

        quote! {
            pub mod #name {
                #(#items)*

                #(#submodules)*
            }
        }
    }
}

impl IntoIterator for ModuleTree {
    type Item = (ModulePath, Module);

    type IntoIter = <BTreeMap<ModulePath, Module> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.modules.into_iter()
    }
}

impl ToTokens for ModuleTree {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        for (path, module) in self.modules.iter() {
            if module.path.parent().is_some() {
                continue;
            }

            self.emit_module(path).to_tokens(tokens);
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct ModulePath {
    segments: Vec<String>,
}

impl ModulePath {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn name(&self) -> &str {
        self.segments.last().unwrap().as_str()
    }

    pub fn parent(&self) -> Option<Self> {
        if self.segments.len() < 2 {
            return None;
        }

        let mut parent = self.clone();
        parent.segments.pop();
        Some(parent)
    }

    pub fn push(&mut self, name: String) {
        self.segments.push(name);
    }

    pub fn item_path(&self, item_name: String) -> ItemPath {
        ItemPath {
            module_path: self.clone(),
            item_name,
        }
    }
}

impl fmt::Display for ModulePath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut it = self.segments.iter().peekable();

        while let Some(segment) = it.next() {
            f.write_str(segment)?;

            if it.peek().is_some() {
                f.write_str("::")?;
            }
        }

        Ok(())
    }
}

impl ToTokens for ModulePath {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mut it = self.segments.iter().peekable();

        quote! { crate :: }.to_tokens(tokens);

        while let Some(segment) = it.next() {
            quote::format_ident!("{segment}").to_tokens(tokens);

            if it.peek().is_some() {
                quote! { :: }.to_tokens(tokens);
            }
        }
    }
}

impl From<&Nsid> for ModulePath {
    fn from(nsid: &Nsid) -> Self {
        let segments = nsid.segments().map(|s| s.to_snake_case()).collect();

        ModulePath { segments }
    }
}

impl From<Nsid> for ModulePath {
    fn from(nsid: Nsid) -> Self {
        Self::from(&nsid)
    }
}

impl FromIterator<String> for ModulePath {
    #[inline]
    fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {
        ModulePath {
            segments: Vec::from_iter(iter),
        }
    }
}

impl<'a> FromIterator<&'a str> for ModulePath {
    fn from_iter<T: IntoIterator<Item = &'a str>>(iter: T) -> Self {
        ModulePath {
            segments: Vec::from_iter(iter.into_iter().map(String::from)),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ItemPath {
    module_path: ModulePath,
    item_name: String,
}

impl ItemPath {
    #[inline]
    pub fn name(&self) -> &str {
        self.item_name.as_str()
    }

    #[inline]
    pub fn module_path(&self) -> &ModulePath {
        &self.module_path
    }

    /// Generates a name for a variant based on the names of the item and its ancestor modules.
    pub fn variant_name(&self, num_ancestors: usize) -> String {
        assert!(num_ancestors < self.module_path.segments.len());

        let mut parts = Vec::new();

        parts.push(self.name());
        parts.extend(
            self.module_path
                .segments
                .iter()
                .rev()
                .take(num_ancestors)
                .map(String::as_str),
        );

        let mut name = String::new();

        for part in parts.into_iter().rev() {
            use std::fmt::Write;

            write!(&mut name, "{part}_").unwrap();
        }

        name.to_pascal_case()
    }
}

impl From<&FullReference> for ItemPath {
    fn from(r: &FullReference) -> Self {
        match r.fragment_name() {
            Some("main") | None => {
                let nsid = r.clone_nsid();
                ModulePath::from(&nsid)
                    .parent()
                    .unwrap()
                    .item_path(nsid.name().to_pascal_case())
            }
            Some(name) => ModulePath::from(r.clone_nsid()).item_path(name.to_pascal_case()),
        }
    }
}

impl From<FullReference> for ItemPath {
    #[inline]
    fn from(r: FullReference) -> Self {
        Self::from(&r)
    }
}

impl ToTokens for ItemPath {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let module_path = &self.module_path;
        let item_name = quote::format_ident!("{}", &self.item_name);

        quote! {
            #module_path :: #item_name
        }
        .to_tokens(tokens)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn module_path_from_nsid() {
        let nsid = Nsid::from_str("com.example.foo").unwrap();
        let mod_path = ModulePath::from(nsid);

        assert_eq!(
            ModulePath::from_iter(vec!["com", "example", "foo"]),
            mod_path
        );
    }
}
