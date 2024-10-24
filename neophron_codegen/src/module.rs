use std::{
    collections::{BTreeMap, BTreeSet},
    fmt,
};

use quote::ToTokens;

pub struct Module {
    path: ModulePath,
    submodules: BTreeSet<ModulePath>,
    items: Vec<syn::Item>,
}

#[derive(Default)]
pub struct Modules {
    modules: BTreeMap<ModulePath, Module>,
}

impl Modules {
    pub fn ensure_exists(&mut self, path: ModulePath) {
        self.modules.entry(path.clone()).or_insert_with(|| Module {
            path,
            submodules: BTreeSet::new(),
            items: Vec::new(),
        });
    }
}

impl IntoIterator for Modules {
    type Item = (ModulePath, Module);

    type IntoIter = <BTreeMap<ModulePath, Module> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.modules.into_iter()
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
        let mut parent = self.clone();
        parent.segments.pop().is_some().then_some(parent)
    }

    pub fn push(&mut self, name: String) {
        self.segments.push(name);
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
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let mut it = self.segments.iter().peekable();

        while let Some(segment) = it.next() {
            quote::format_ident!("{segment}").to_tokens(tokens);

            if it.peek().is_some() {
                quote::quote! { :: }.to_tokens(tokens);
            }
        }
    }
}
