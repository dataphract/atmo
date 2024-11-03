use std::{
    collections::{BTreeMap, BTreeSet},
    fmt,
};

use atmo_core::nsid::Nsid;
use heck::{ToPascalCase, ToSnakeCase};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use crate::{
    enum_::{StringEnumDef, UnionEnumDef},
    rpc::RpcDef,
    struct_::StructDef,
};

pub struct Module {
    // Fully-qualified module path.
    path: ModulePath,
    // Set of submodules.
    submodules: BTreeSet<ModulePath>,
    // Map of non-submodule items, keyed by name.
    items: BTreeMap<String, Item>,
}

impl Module {
    pub fn item_exists(&self, name: &str) -> bool {
        self.items.contains_key(name)
    }

    pub fn add_item(&mut self, name: String, item: Item) -> Result<(), NameCollision> {
        if let Some(existing) = self.items.get(&name) {
            match (existing, &item) {
                (Item::StringEnum(s1), Item::StringEnum(s2)) => {
                    if s1 != s2 {
                        return Err(NameCollision);
                    } else {
                        return Ok(());
                    }
                }

                (Item::UnionEnum(u1), Item::UnionEnum(u2)) => {
                    if u1 != u2 {
                        return Err(NameCollision);
                    } else {
                        return Ok(());
                    }
                }
                _ => panic!("item collision: {name}"),
            }
        }

        self.items.insert(name, item);

        Ok(())
    }
}

#[derive(Debug)]
pub struct NameCollision;

#[derive(Debug)]
pub enum Item {
    Rpc(RpcDef),
    Struct(StructDef),
    StringEnum(StringEnumDef),
    UnionEnum(UnionEnumDef),
}

impl ToTokens for Item {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Item::Rpc(r) => r.to_tokens(tokens),
            Item::Struct(s) => s.to_tokens(tokens),
            Item::StringEnum(e) => e.to_tokens(tokens),
            Item::UnionEnum(e) => e.to_tokens(tokens),
        }
    }
}

#[derive(Default)]
pub struct Generated {
    // Flattened module tree, keyed by fully-qualified module path.
    modules: BTreeMap<ModulePath, Module>,
}

impl Generated {
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

    pub fn item(&self, path: &ItemPath) -> Option<&Item> {
        let module = self.modules.get(&path.module_path)?;
        module.items.get(&path.item_name)
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

impl IntoIterator for Generated {
    type Item = (ModulePath, Module);

    type IntoIter = <BTreeMap<ModulePath, Module> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.modules.into_iter()
    }
}

impl ToTokens for Generated {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        for (path, module) in self.modules.iter() {
            if module.path.parent().is_some() {
                continue;
            }

            self.emit_module(path).to_tokens(tokens);
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ModulePath {
    segments: Vec<String>,
}

impl ModulePath {
    pub fn new() -> Self {
        Self {
            segments: Vec::new(),
        }
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
