use std::{
    collections::{BTreeMap, HashMap},
    str::FromStr,
};

use atmo_core::{
    nsid::{self, FullReference},
    Nsid,
};
use atmo_lexicon::{
    Blob, Boolean, Bytes, FieldSchema, Input, Integer, IoSchema, Lexicon, Object, Output, Ref,
    Schema, Union,
};
use heck::{ToPascalCase, ToSnakeCase};

use crate::{
    enum_::{RustStringEnumDef, RustUnionEnumDef, StringEnumVariant, UnionEnumVariant},
    module::{Item, ItemPath, ModulePath, ModuleTree},
    rpc::{RpcType, RustRpcDef},
    struct_::{RustStructDef, RustStructField},
    Type,
};

/// Abstract namespace tree representation.
///
/// This type holds the full tree of logical types extracted from the input schemae, prior to
/// generation of Rust type definitions.
#[derive(Default)]
pub struct NamespaceTree {
    /// Flattened namespace tree, keyed by NSID.
    namespaces: BTreeMap<Nsid, Namespace>,
}

impl NamespaceTree {
    pub fn add_lexicon(&mut self, lex: &Lexicon) {
        let namespace = Namespace::from_lexicon(lex);
        let nsid = Nsid::from_str(lex.id.as_str()).unwrap();
        self.namespaces.insert(nsid, namespace);
    }

    pub fn to_module_tree(&self) -> ModuleTree {
        let mut mod_tree = ModuleTree::default();

        for (nsid, namespace) in &self.namespaces {
            let mod_path = ModulePath::from(nsid);

            // Create type for primary definition.
            if let Some(main_def) = &namespace.main_def {
                let parent_mod_path = mod_path.parent().unwrap();

                match main_def {
                    MainDef::Object(obj) => {
                        let rs = self.create_rust_struct(nsid, "main", obj);
                        // TODO(dp): the name should probably not be stored in the struct def
                        let parent_mod = mod_tree.get_or_create_mut(&parent_mod_path);
                        parent_mod
                            .add_item(rs.name.to_string(), Item::Struct(rs))
                            .unwrap();
                    }

                    MainDef::Rpc(rpc) => {
                        let module = mod_tree.get_or_create_mut(&mod_path);

                        // Emit params, if any.
                        if let Some(params) = rpc
                            .params
                            .as_ref()
                            .map(|p| self.create_rust_struct(nsid, "params", p))
                        {
                            module.add_item("Params".into(), params.into()).unwrap();
                        }

                        let input_def = rpc.input.as_ref().and_then(|io| io.ty.as_ref()).and_then(
                            |i| match i {
                                RpcIoTy::Object(o) => {
                                    Some(self.create_rust_struct(nsid, "input", o).into())
                                }
                                RpcIoTy::Ref(_) => None,
                                RpcIoTy::Union(u) => {
                                    Some(self.create_rust_union_enum(nsid, None, "Input", u).into())
                                }
                            },
                        );

                        // Emit input type, if any.
                        if let Some(input) = input_def {
                            module.add_item("Input".into(), input).unwrap();
                        }

                        let output_def =
                            rpc.output.as_ref().and_then(|io| io.ty.as_ref()).and_then(
                                |o| match o {
                                    RpcIoTy::Object(o) => {
                                        Some(self.create_rust_struct(nsid, "output", o).into())
                                    }
                                    RpcIoTy::Ref(_) => None,
                                    RpcIoTy::Union(u) => Some(
                                        self.create_rust_union_enum(nsid, None, "Output", u).into(),
                                    ),
                                },
                            );

                        // Emit output type, if any.
                        if let Some(output) = output_def {
                            module.add_item("Output".into(), output).unwrap();
                        }

                        if let Some(error) = rpc
                            .error
                            .as_ref()
                            .map(|e| self.create_rust_string_enum("Error", e))
                        {
                            module.add_item("Error".into(), error.into()).unwrap();
                        }

                        let parent_mod = mod_tree.get_or_create_mut(&parent_mod_path);

                        let rpc = self.create_rust_rpc(nsid, rpc);
                        parent_mod
                            .add_item(nsid.name().to_pascal_case(), rpc.into())
                            .unwrap();
                    }
                }
            }

            let module = mod_tree.get_or_create_mut(&mod_path);

            // Create types for each other top-level def.
            for (def_name, other_def) in &namespace.other_defs {
                let name = def_name.to_pascal_case();

                let item = match &other_def.ty {
                    OtherDefTy::Object(obj) => {
                        Item::Struct(self.create_rust_struct(nsid, def_name, obj))
                    }
                    OtherDefTy::StringEnum(s) => {
                        Item::StringEnum(self.create_rust_string_enum(&name, s))
                    }
                    OtherDefTy::Union(u) => {
                        Item::UnionEnum(self.create_rust_union_enum(nsid, Some(def_name), &name, u))
                    }
                };

                module.add_item(name, item).unwrap();
            }

            // Create types for all anonymous types defined in object properties.
            for (prop_name, prop_def) in &namespace.prop_defs {
                let mut submod_path = mod_path.clone();
                submod_path.push(prop_name.def_name.to_snake_case());

                let submodule = mod_tree.get_or_create_mut(&submod_path);
                let name = prop_name.prop_name.to_pascal_case();

                let item = match prop_def {
                    PropDef::StringEnum(s) => {
                        Item::StringEnum(self.create_rust_string_enum(&name, s))
                    }
                    PropDef::Union(u) => Item::UnionEnum(self.create_rust_union_enum(
                        nsid,
                        Some(&prop_name.def_name),
                        &name,
                        u,
                    )),
                };

                submodule.add_item(name, item).unwrap();
            }
        }

        mod_tree
    }

    fn create_rust_rpc(&self, nsid: &Nsid, rpc: &RpcDef) -> RustRpcDef {
        let mod_path = ModulePath::from(nsid);
        let params = rpc
            .params
            .is_some()
            .then(|| mod_path.item_path("Params".into()));
        let input = rpc
            .input
            .as_ref()
            .and_then(|i| i.ty.as_ref())
            .map(|ty| match ty {
                RpcIoTy::Ref(r) => r.into(),
                RpcIoTy::Object(_) | RpcIoTy::Union(_) => mod_path.item_path("Input".into()),
            });

        let output = rpc
            .output
            .as_ref()
            .and_then(|o| o.ty.as_ref())
            .map(|ty| match ty {
                RpcIoTy::Ref(r) => r.into(),
                RpcIoTy::Object(_) | RpcIoTy::Union(_) => mod_path.item_path("Output".into()),
            });

        let name = quote::format_ident!("{}", nsid.name().to_pascal_case());

        RustRpcDef {
            name,
            ty: rpc.ty,
            nsid: nsid.clone(),
            params,
            input,
            output,
            output_encoding: rpc
                .output
                .as_ref()
                .map(|o| o.encoding.clone())
                .unwrap_or("*/*".into()),
            error: None,
        }
    }

    fn create_rust_struct(&self, ns: &Nsid, def_name: &str, obj: &ObjectDef) -> RustStructDef {
        let mod_path = ModulePath::from(ns);

        let fields = obj
            .fields
            .iter()
            .map(|f| {
                let field_name: String = match f.name.as_str() {
                    "type" => "ty".into(),
                    "ref" => "ref_".into(),
                    other => other.to_snake_case(),
                };

                let field_ident = quote::format_ident!("{field_name}");
                let rename = f.name.clone();
                let inner_ty = match &f.ty {
                    ObjectFieldTy::Builtin(b) => Type::from(b),
                    ObjectFieldTy::Defined => {
                        let mut submod_path = mod_path.clone();
                        submod_path.push(def_name.to_snake_case());

                        submod_path.item_path(f.name.to_pascal_case()).into()
                    }
                    ObjectFieldTy::Ref(r) => {
                        let full =
                            self.resolve_ref(ns, &nsid::Reference::from_str(&r.ref_).unwrap());

                        let ref_ns = self.namespaces.get(&full.clone_nsid()).unwrap();

                        match full.fragment_name() {
                            Some(_) => {
                                let referent = ref_ns
                                    .other_defs
                                    .get(full.fragment_name().unwrap())
                                    .unwrap();

                                let ty = ItemPath::from(full).into();

                                if referent.is_array {
                                    Type::Vec(Box::new(ty))
                                } else {
                                    ty
                                }
                            }

                            None => ItemPath::from(full).into(),
                        }
                    }
                };

                RustStructField {
                    doc: None,
                    name: field_ident,
                    rename,
                    is_array: f.is_array,
                    is_optional: f.is_optional,
                    is_nullable: f.is_nullable,
                    inner_ty,
                }
            })
            .collect();

        let name_s = if def_name == "main" {
            ns.name().to_pascal_case()
        } else {
            def_name.to_pascal_case()
        };

        let name = quote::format_ident!("{name_s}");

        RustStructDef { name, fields }
    }

    fn create_rust_string_enum(&self, type_name: &str, def: &StringEnumDef) -> RustStringEnumDef {
        let variants = def
            .values
            .iter()
            .map(|val| {
                let ident = quote::format_ident!("{}", val.to_pascal_case());

                StringEnumVariant {
                    doc: None,
                    string_value: val.into(),
                    ident,
                }
            })
            .collect();

        RustStringEnumDef {
            ident: quote::format_ident!("{type_name}"),
            variants,
            is_open: def.is_open,
        }
    }

    fn create_rust_union_enum(
        &self,
        nsid: &Nsid,
        parent_def: Option<&str>,
        type_name: &str,
        def: &UnionDef,
    ) -> RustUnionEnumDef {
        let parent_full = parent_def.map(|p| {
            let s = format!("{nsid}#{p}");
            FullReference::from_str(&s).unwrap()
        });

        let variants = def
            .variants
            .iter()
            .map(|r| {
                let full = self.resolve_ref(nsid, r);
                let path = ItemPath::from(&full);

                // If this union is an object property and has the parent def as a variant, it needs
                // an indirection to avoid infinite size.
                //
                // TODO(dp): this only handles self-loops -- a more complex dependency cycle won't
                // be caught here. Need a more sophisticated way to detect cycles, or to maintain a
                // list of properties that need boxing by hand.
                let needs_boxed = parent_full.as_ref().is_some_and(|p| p == &full);

                UnionEnumVariant {
                    nsid: full,
                    path,
                    needs_boxed,
                }
            })
            .collect();

        RustUnionEnumDef {
            doc: None,
            ident: quote::format_ident!("{type_name}"),
            variants,
            is_open: def.is_open,
        }
    }

    fn resolve_ref(&self, nsid: &Nsid, ref_: &nsid::Reference) -> FullReference {
        match ref_ {
            nsid::Reference::Full(f) => f.clone(),
            nsid::Reference::Relative(frag) => {
                nsid::FullReference::from_str(&format!("{nsid}{}", frag.as_str())).unwrap()
            }
        }
    }
}

pub struct Namespace {
    nsid: Nsid,

    main_def: Option<MainDef>,
    other_defs: HashMap<String, OtherDef>,
    prop_defs: HashMap<PropName, PropDef>,
}

impl Namespace {
    pub fn from_lexicon(lexicon: &Lexicon) -> Self {
        let nsid = Nsid::from_str(&lexicon.id).unwrap();

        let mut ns = Namespace {
            nsid,
            main_def: None,
            other_defs: HashMap::new(),
            prop_defs: HashMap::new(),
        };

        for (def_name, def) in &lexicon.defs {
            if def_name == "main" {
                ns.insert_main_def(def);
            } else {
                ns.insert_other_def(def_name.into(), def);
            }
        }

        ns
    }

    fn insert_main_def(&mut self, lex: &Schema) {
        if self.main_def.is_some() {
            panic!("main def already set");
        }

        let def = match lex {
            Schema::Object(o) => MainDef::Object(self.create_object_def("main", o)),

            Schema::Procedure(p) => {
                let params = p
                    .parameters
                    .as_ref()
                    .map(|p| self.create_object_def("params", p));

                let input = p.input.as_ref().map(|i| RpcIo {
                    encoding: i.encoding.clone(),
                    ty: self.create_input_def(i),
                });

                let output = p.output.as_ref().map(|o| RpcIo {
                    encoding: o.encoding.clone(),
                    ty: self.create_output_def(o),
                });

                let error = p.errors.as_ref().map(|e| self.create_error_def(e));

                MainDef::Rpc(RpcDef {
                    ty: RpcType::Procedure,
                    params,
                    input,
                    output,
                    error,
                })
            }

            Schema::Query(q) => {
                let params = q
                    .parameters
                    .as_ref()
                    .map(|p| self.create_object_def("params", p));

                let output = q.output.as_ref().map(|o| RpcIo {
                    encoding: o.encoding.clone(),
                    ty: self.create_output_def(o),
                });

                let error = q.errors.as_ref().map(|e| self.create_error_def(e));

                MainDef::Rpc(RpcDef {
                    ty: RpcType::Query,
                    params,
                    input: None,
                    output,
                    error,
                })
            }

            Schema::Record(r) => MainDef::Object(self.create_object_def("main", &r.record)),

            Schema::Subscription(_s) => {
                eprintln!("subscriptions not handled");
                return;
            }

            _ => unreachable!(),
        };

        self.main_def = Some(def);
    }

    /// Creates a non-primary definition and inserts it into the namespace tree.
    fn insert_other_def(&mut self, name: String, lex: &Schema) {
        if self.other_defs.contains_key(&name) {
            panic!("name collision: {name}");
        }

        let (is_array, ty) = match lex {
            Schema::Array(a) => match &*a.items {
                FieldSchema::Union(u) => (true, OtherDefTy::Union(UnionDef::from_lexicon(u))),

                a => unreachable!("unhandled lexicon array item: {a:?}"),
            },

            Schema::Object(o) => (false, OtherDefTy::Object(self.create_object_def(&name, o))),

            Schema::String(s) => (
                false,
                match StringEnumDef::from_lexicon(s) {
                    Some(def) => OtherDefTy::StringEnum(def),
                    None => return,
                },
            ),

            // Tokens don't produce a type definition.
            Schema::Token(_) => return,

            _ => panic!("unhandled lexicon: {lex:?}"),
        };

        self.other_defs.insert(name, OtherDef { is_array, ty });
    }

    /// Creates a definition for an RPC input type.
    fn create_input_def(&mut self, input: &Input) -> Option<RpcIoTy> {
        let schema = input.schema.as_ref()?;

        match schema {
            IoSchema::Object(o) => Some(RpcIoTy::Object(self.create_object_def("input", o))),
            IoSchema::Ref(r) => {
                let r = nsid::Reference::from_str(&r.ref_).unwrap();
                Some(RpcIoTy::Ref(self.resolve_ref(&r)))
            }
            IoSchema::Union(u) => Some(RpcIoTy::Union(UnionDef::from_lexicon(u))),
        }
    }

    /// Creates a definition for an RPC output type.
    fn create_output_def(&mut self, output: &Output) -> Option<RpcIoTy> {
        let schema = output.schema.as_ref()?;

        match schema {
            IoSchema::Object(o) => Some(RpcIoTy::Object(self.create_object_def("output", o))),
            IoSchema::Ref(r) => Some(RpcIoTy::Ref(
                self.resolve_ref(&nsid::Reference::from_str(&r.ref_).unwrap()),
            )),
            IoSchema::Union(u) => Some(RpcIoTy::Union(UnionDef::from_lexicon(u))),
        }
    }

    /// Creates a definition for an RPC error enum.
    fn create_error_def(&self, errors: &[atmo_lexicon::Error]) -> StringEnumDef {
        StringEnumDef {
            values: errors.iter().map(|e| e.name.clone()).collect(),
            is_open: true,
        }
    }

    /// Creates a definition for an object.
    fn create_object_def(&mut self, def_name: &str, obj: &Object) -> ObjectDef {
        let mut fields = Vec::new();

        for (prop_name, prop) in &obj.properties {
            let (ty, is_array) = match prop {
                FieldSchema::Array(a) => (ObjectFieldTy::from_lexicon(&a.items), true),

                p => (ObjectFieldTy::from_lexicon(p), false),
            };

            // Slow linear lookup, but objects have few properties.
            let is_optional = !obj.required.contains(prop_name);
            let is_nullable = obj.nullable.contains(prop_name);

            fields.push(ObjectField {
                name: prop_name.clone(),
                is_array,
                is_nullable,
                is_optional,
                ty,
            });

            if let Some(p) = PropDef::from_lexicon(prop) {
                let prop_name = PropName {
                    def_name: def_name.into(),
                    prop_name: prop_name.clone(),
                };

                self.prop_defs.insert(prop_name, p);
            }
        }

        ObjectDef { fields }
    }

    fn resolve_ref(&self, ref_: &nsid::Reference) -> FullReference {
        match ref_ {
            nsid::Reference::Full(f) => f.clone(),
            nsid::Reference::Relative(frag) => {
                nsid::FullReference::from_str(&format!("{}{}", self.nsid, frag.as_str())).unwrap()
            }
        }
    }
}

/// A primary item definition.
pub enum MainDef {
    Object(ObjectDef),
    Rpc(RpcDef),
}

pub struct OtherDef {
    pub is_array: bool,
    pub ty: OtherDefTy,
}

/// A non-primary item definition.
pub enum OtherDefTy {
    Object(ObjectDef),
    StringEnum(StringEnumDef),
    Union(UnionDef),
}

/// An RPC I/O item definition.
pub enum RpcIoTy {
    Object(ObjectDef),
    Ref(nsid::FullReference),
    Union(UnionDef),
}

/// An object property item definition.
pub enum PropDef {
    StringEnum(StringEnumDef),
    Union(UnionDef),
}

impl PropDef {
    fn from_lexicon(lex: &FieldSchema) -> Option<Self> {
        match lex {
            FieldSchema::Array(a) => match &*a.items {
                FieldSchema::Array(_) => panic!("nested arrays not supported"),
                FieldSchema::String(s) => StringEnumDef::from_lexicon(s).map(PropDef::StringEnum),
                FieldSchema::Union(u) => Some(PropDef::Union(UnionDef::from_lexicon(u))),
                _ => None,
            },
            FieldSchema::String(s) => StringEnumDef::from_lexicon(s).map(PropDef::StringEnum),
            FieldSchema::Union(u) => Some(PropDef::Union(UnionDef::from_lexicon(u))),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PropName {
    pub def_name: String,
    pub prop_name: String,
}

pub struct ObjectDef {
    pub fields: Vec<ObjectField>,
}

pub struct ObjectField {
    pub name: String,
    pub ty: ObjectFieldTy,
    pub is_array: bool,
    pub is_optional: bool,
    pub is_nullable: bool,
}

pub enum ObjectFieldTy {
    Builtin(BuiltinTy),
    /// The field is a newly defined type.
    Defined,
    /// The field type is a reference to a type defined elsewhere.
    Ref(Ref),
}

impl ObjectFieldTy {
    fn from_lexicon(lex: &FieldSchema) -> Self {
        match lex {
            FieldSchema::Array(_) => panic!("nested arrays not allowed"),
            FieldSchema::Blob(b) => BuiltinTy::Blob(b.clone()).into(),
            FieldSchema::Boolean(b) => BuiltinTy::Boolean(b.clone()).into(),
            FieldSchema::Bytes(b) => BuiltinTy::Bytes(b.clone()).into(),
            FieldSchema::CidLink => BuiltinTy::CidLink.into(),
            FieldSchema::Integer(i) => BuiltinTy::Integer(i.clone()).into(),
            FieldSchema::Ref(r) => ObjectFieldTy::Ref(r.clone()),
            FieldSchema::String(s) => {
                if !s.known_values.is_empty() || s.enum_values.is_some() {
                    ObjectFieldTy::Defined
                } else {
                    BuiltinTy::String(s.clone()).into()
                }
            }
            FieldSchema::Union(_) => ObjectFieldTy::Defined,
            FieldSchema::Unknown => BuiltinTy::Unknown.into(),
        }
    }
}

impl From<BuiltinTy> for ObjectFieldTy {
    #[inline]
    fn from(ty: BuiltinTy) -> Self {
        ObjectFieldTy::Builtin(ty)
    }
}

pub enum BuiltinTy {
    Blob(Blob),
    Boolean(Boolean),
    Bytes(Bytes),
    CidLink,
    Integer(Integer),
    // TODO: new type
    String(atmo_lexicon::String),
    Unknown,
}

pub struct RpcDef {
    pub ty: RpcType,
    pub params: Option<ObjectDef>,
    pub input: Option<RpcIo>,
    pub output: Option<RpcIo>,
    pub error: Option<StringEnumDef>,
}

pub struct RpcIo {
    pub encoding: String,
    pub ty: Option<RpcIoTy>,
}

pub struct StringEnumDef {
    pub values: Vec<String>,
    pub is_open: bool,
}

impl StringEnumDef {
    fn from_lexicon(lex: &atmo_lexicon::String) -> Option<Self> {
        if !lex.known_values.is_empty() {
            assert!(lex.enum_values.is_none());
            assert!(lex.const_value.is_none());

            let values = lex.known_values.iter().map(String::from).collect();

            return Some(StringEnumDef {
                values,
                is_open: true,
            });
        };

        if let Some(ev) = &lex.enum_values {
            assert!(lex.const_value.is_none());

            let values = ev.iter().map(String::from).collect();

            return Some(StringEnumDef {
                values,
                is_open: false,
            });
        }

        None
    }
}

pub struct UnionDef {
    pub variants: Vec<nsid::Reference>,
    pub is_open: bool,
}

impl UnionDef {
    fn from_lexicon(lex: &Union) -> Self {
        let variants = lex
            .refs
            .iter()
            .map(|s| nsid::Reference::from_str(s).unwrap())
            .collect();

        UnionDef {
            variants,
            is_open: !lex.closed,
        }
    }
}
