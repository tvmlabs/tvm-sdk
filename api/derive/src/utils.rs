use api_info::NumberType;
use proc_macro2::TokenStream;
use quote::quote;
use syn::AngleBracketedGenericArguments;
use syn::Attribute;
use syn::Fields;
use syn::GenericArgument;
use syn::Lit;
use syn::Meta;
use syn::MetaNameValue;
use syn::NestedMeta;
use syn::Path;
use syn::PathArguments;
use syn::Type;
use syn::TypeArray;

pub(crate) fn field_from(
    name: Option<&syn::Ident>,
    attrs: &Vec<Attribute>,
    value: api_info::Type,
) -> api_info::Field {
    let (summary, description) = doc_from(attrs);
    let value = if has_attr_value("serde", "default", attrs) {
        api_info::Type::Optional { inner: Box::new(value) }
    } else {
        value
    };
    let name = if let Some(name) = find_attr_value("serde", "rename", attrs) {
        name
    } else {
        name.map(|x| x.to_string()).unwrap_or("".into())
    };
    api_info::Field { name, summary, description, value }
}

pub(crate) fn module_from(name: Option<&syn::Ident>, attrs: &Vec<Attribute>) -> api_info::Module {
    let (summary, description) = doc_from(attrs);
    let name = if let Some(name) = find_attr_value("api_module", "name", attrs) {
        name
    } else {
        name.map(|x| x.to_string()).unwrap_or("".into())
    };
    api_info::Module { name, summary, description, types: vec![], functions: vec![] }
}

pub(crate) fn doc_to_tokens(
    summary: &Option<String>,
    description: &Option<String>,
) -> (TokenStream, TokenStream) {
    let summary = match summary {
        Some(s) => quote! { Some(#s.into()) },
        None => quote! { None },
    };
    let description = match description {
        Some(s) => quote! { Some(#s.into()) },
        None => quote! { None },
    };
    (summary, description)
}

pub(crate) fn field_to_tokens(f: &api_info::Field) -> TokenStream {
    let name = &f.name;
    let value = type_to_tokens(&f.value);
    let (summary, description) = doc_to_tokens(&f.summary, &f.description);
    quote! {
        api_info::Field {
            name: #name.into(),
            summary: #summary,
            description: #description,
            value: #value,
        }
    }
}

pub(crate) fn module_to_tokens(m: &api_info::Module) -> TokenStream {
    let name = &m.name;
    let (summary, description) = doc_to_tokens(&m.summary, &m.description);
    quote! {
        api_info::Module {
            name: #name.into(),
            summary: #summary,
            description: #description,
            types: Vec::new(),
            functions: Vec::new(),
        }
    }
}

pub(crate) fn const_value_to_tokens(v: &api_info::ConstValue) -> TokenStream {
    match v {
        api_info::ConstValue::None {} => quote! { api_info::ConstValue::None {} },
        api_info::ConstValue::Bool(repr) => quote! { api_info::ConstValue::Bool(#repr.into()) },
        api_info::ConstValue::String(repr) => {
            quote! { api_info::ConstValue::String(#repr.into()) }
        }
        api_info::ConstValue::Number(repr) => {
            quote! { api_info::ConstValue::Number(#repr.into()) }
        }
    }
}

pub(crate) fn const_to_tokens(c: &api_info::Const) -> TokenStream {
    let name = &c.name;
    let value = const_value_to_tokens(&c.value);
    let (summary, description) = doc_to_tokens(&c.summary, &c.description);
    quote! {
        api_info::Const {
            name: #name.into(),
            summary: #summary,
            description: #description,
            value: #value,
        }
    }
}

pub(crate) fn function_to_tokens(m: &api_info::Function) -> TokenStream {
    let name = &m.name;
    let params = m.params.iter().map(field_to_tokens);
    let result = type_to_tokens(&m.result);
    let (summary, description) = doc_to_tokens(&m.summary, &m.description);
    quote! {
        api_info::Function {
            name: #name.into(),
            summary: #summary,
            description: #description,
            params: vec![#(#params), *],
            result: #result,
            errors: None,
        }
    }
}

fn number_type_to_tokens(number_type: &NumberType) -> TokenStream {
    match number_type {
        NumberType::UInt => quote! { api_info::NumberType::UInt },
        NumberType::Int => quote! { api_info::NumberType::Int },
        NumberType::Float => quote! { api_info::NumberType::Float },
    }
}

fn type_to_tokens(t: &api_info::Type) -> TokenStream {
    match t {
        api_info::Type::None {} => quote! { api_info::Type::None {} },
        api_info::Type::Any {} => quote! { api_info::Type::Any {} },
        api_info::Type::Boolean {} => quote! { api_info::Type::Boolean {} },
        api_info::Type::Number { number_type, number_size: size } => {
            let number_type_tokens = number_type_to_tokens(number_type);
            quote! { api_info::Type::Number { number_type: #number_type_tokens, number_size: #size } }
        }
        api_info::Type::BigInt { number_type, number_size: size } => {
            let number_type_tokens = number_type_to_tokens(number_type);
            quote! { api_info::Type::BigInt { number_type: #number_type_tokens, number_size: #size } }
        }
        api_info::Type::String {} => quote! { api_info::Type::String {} },
        api_info::Type::Ref { name } => {
            quote! { api_info::Type::Ref { name: #name.into() } }
        }
        api_info::Type::Optional { inner } => {
            let inner_type = type_to_tokens(inner);
            quote! { api_info::Type::Optional { inner: #inner_type.into() } }
        }
        api_info::Type::Array { item } => {
            let item_type = type_to_tokens(item);
            quote! { api_info::Type::Array { item: #item_type.into() } }
        }
        api_info::Type::Struct { fields } => {
            let field_types = fields.iter().map(field_to_tokens);
            quote! { api_info::Type::Struct { fields: vec![#(#field_types),*] } }
        }
        api_info::Type::EnumOfConsts { consts } => {
            let consts = consts.iter().map(const_to_tokens);
            quote! { api_info::Type::EnumOfConsts { consts: vec![#(#consts),*] } }
        }
        api_info::Type::EnumOfTypes { types } => {
            let types = types.iter().map(field_to_tokens);
            quote! { api_info::Type::EnumOfTypes { types: vec![#(#types),*] } }
        }
        api_info::Type::Generic { name, args } => {
            let types = args.iter().map(type_to_tokens);
            quote! { api_info::Type::Generic { name: #name.into(), args: vec![#(#types),*] } }
        }
    }
}

pub(crate) fn type_from(ty: &Type) -> api_info::Type {
    match ty {
        Type::Array(a) => array_type_from(a),
        Type::BareFn(_f) => panic!("function is unsupported"),
        Type::Group(_g) => panic!("group is unsupported"),
        Type::ImplTrait(_t) => panic!("impl_trait is unsupported"),
        Type::Infer(_t) => panic!("infer is unsupported"),
        Type::Macro(_t) => panic!("macro is unsupported"),
        Type::Never(_n) => panic!("never is unsupported"),
        Type::Paren(_p) => panic!("paren is unsupported"),
        Type::Path(p) => type_from_path(&p.path),
        Type::Ptr(_p) => panic!("ptr is unsupported"),
        Type::Reference(_r) => panic!("reference is unsupported"),
        Type::Slice(_s) => panic!("slice is unsupported"),
        Type::TraitObject(_t) => panic!("trait_object is unsupported"),
        Type::Tuple(t) => {
            if t.elems.is_empty() {
                api_info::Type::None {}
            } else {
                panic!("None empty tuples is unsupported")
            }
        }
        Type::Verbatim(_t) => panic!("verbatim is unsupported"),
        _ => panic!("Unsupported type"),
    }
}

fn array_type_from(ty: &TypeArray) -> api_info::Type {
    api_info::Type::Array { item: Box::new(type_from(ty.elem.as_ref())) }
}

fn type_from_path(path: &Path) -> api_info::Type {
    if let Some(segment) = path.segments.last() {
        let name = unqualified_type_name(segment.ident.to_string());
        if let Some(result) = match &segment.arguments {
            PathArguments::None => Some(resolve_type_name(name)),
            PathArguments::AngleBracketed(args) => generic_type_from(name, args),
            _ => None,
        } {
            return result;
        }
    }
    panic!(
        "Unsupported type {:?}",
        path.segments.last().map(|x| x.ident.to_string()).unwrap_or_default()
    )
}

fn resolve_type_name(name: String) -> api_info::Type {
    match name.as_ref() {
        "String" => api_info::Type::String {},
        "bool" => api_info::Type::Boolean {},
        "u8" => api_info::Type::u(8),
        "u16" => api_info::Type::u(16),
        "u32" => api_info::Type::u(32),
        "u64" => api_info::Type::u(64),
        "u128" => api_info::Type::u(128),
        "i8" => api_info::Type::i(8),
        "i16" => api_info::Type::i(16),
        "i32" => api_info::Type::i(32),
        "i64" => api_info::Type::i(64),
        "i128" => api_info::Type::i(128),
        "f32" => api_info::Type::f(32),
        "usize" | "isize" => panic!("usize and isize can't be used in API"),
        _ => api_info::Type::Ref { name },
    }
}

fn generic_type_from(
    name: String,
    args: &AngleBracketedGenericArguments,
) -> Option<api_info::Type> {
    let get_inner_type = || match (args.args.len(), args.args.first()) {
        (1, Some(GenericArgument::Type(t))) => Some(type_from(t)),
        _ => None,
    };
    match name.as_ref() {
        "Option" => get_inner_type().map(|x| api_info::Type::Optional { inner: x.into() }),
        "Vec" => get_inner_type().map(|x| api_info::Type::Array { item: x.into() }),
        _ => {
            let args = args
                .args
                .iter()
                .map(|arg| {
                    if let GenericArgument::Type(t) = arg {
                        type_from(t)
                    } else {
                        panic!("Generic argument must be the type.")
                    }
                })
                .collect();
            Some(api_info::Type::Generic { name, args })
        }
    }
}

fn replace_tabs(s: &str) -> String {
    s.split('\t').collect::<Vec<&str>>().join("    ").trim_end().into()
}

fn get_leading_spaces(s: &str) -> usize {
    let mut count = 0;
    while count < s.len() && &s[count..=count] == " " {
        count += 1;
    }
    count
}

fn reduce_lines(lines: Vec<String>) -> Vec<String> {
    if lines.is_empty() {
        return lines;
    }
    let mut min_leading_spaces = None;
    let mut reduced = Vec::new();
    for line in &lines {
        let line = replace_tabs(line);
        if !line.is_empty() {
            let leading_spaces = get_leading_spaces(&line);
            if min_leading_spaces.is_none() || leading_spaces < min_leading_spaces.unwrap() {
                min_leading_spaces = Some(leading_spaces);
            }
        }
        reduced.push(line);
    }
    if min_leading_spaces.is_some() && min_leading_spaces.unwrap() > 0 {
        for line in &mut reduced {
            if !line.is_empty() {
                *line = line[min_leading_spaces.unwrap()..].into();
            }
        }
    }
    reduced
}

fn get_doc(element_summary: String, element_description: String) -> (String, String) {
    if element_description.trim().is_empty() {
        return (element_summary, String::new());
    }
    let lines = reduce_lines(element_description.split('\n').map(|s| s.into()).collect());
    let mut summary = String::new();
    let mut summary_complete = false;
    let mut description = String::new();
    for line in &lines {
        if summary_complete {
            if !line.is_empty() || !description.is_empty() {
                description.push_str(line);
                description.push('\n');
            }
        } else if !line.is_empty() {
            if !summary.is_empty() {
                summary.push(' ');
            }
            if let Some(dot_pos) = line.find(". ").or(line.find(".\n")) {
                summary.push_str(&line[0..(dot_pos + 1)]);
                summary_complete = true;
                description.push_str(line[(dot_pos + 1)..].trim_start());
                description.push(' ');
            } else {
                summary.push_str(line);
            }
        } else if !summary.is_empty() {
            summary_complete = true;
        }
    }
    (summary, description.trim().into())
}

pub(crate) fn doc_from(attrs: &[Attribute]) -> (Option<String>, Option<String>) {
    let mut summary = String::new();
    let mut description = String::new();

    fn try_add(doc: &mut String, s: &str) {
        if !doc.is_empty() {
            doc.push('\n');
        }
        doc.push_str(s);
    }

    for attr in attrs.iter() {
        match DocAttr::from(attr) {
            DocAttr::Doc(text) => try_add(&mut description, &text),
            DocAttr::Summary(text) => try_add(&mut summary, &text),
            _ => (),
        }
    }

    if summary.is_empty() && !description.is_empty() {
        if let Some(line) = description.lines().next() {
            summary.push_str(line);
        }
    }
    fn non_empty(s: String) -> Option<String> {
        if s.is_empty() { None } else { Some(s) }
    }
    let (summary, description) = get_doc(summary, description);
    (non_empty(summary), non_empty(description))
}

enum DocAttr {
    None,
    Summary(String),
    Doc(String),
}

impl DocAttr {
    fn from(attr: &Attribute) -> DocAttr {
        match attr.parse_meta() {
            Ok(Meta::NameValue(ref meta)) => {
                return get_value_of("doc", meta).map(DocAttr::Doc).unwrap_or(DocAttr::None);
            }
            Ok(Meta::List(ref list)) => {
                if path_is(&list.path, "doc") {
                    if let Some(NestedMeta::Meta(Meta::NameValue(meta))) = list.nested.first() {
                        return get_value_of("summary", meta)
                            .map(DocAttr::Summary)
                            .unwrap_or(DocAttr::None);
                    }
                }
            }
            _ => (),
        };
        DocAttr::None
    }
}

fn find_attr_value_meta(
    attr_name: &'static str,
    value_name: &'static str,
    attrs: &Vec<Attribute>,
) -> Option<Meta> {
    for attr in attrs {
        if let Ok(Meta::List(list)) = attr.parse_meta() {
            if path_is(&list.path, attr_name) {
                for item in list.nested {
                    if let NestedMeta::Meta(meta) = item {
                        let value_path = match &meta {
                            Meta::NameValue(name_value) => Some(&name_value.path),
                            Meta::Path(path) => Some(path),
                            _ => None,
                        };
                        if let Some(path) = value_path {
                            if path_is(path, value_name) {
                                return Some(meta);
                            }
                        }
                    }
                }
            }
        }
    }
    None
}

pub(crate) fn find_attr_value(
    attr_name: &'static str,
    value_name: &'static str,
    attrs: &Vec<Attribute>,
) -> Option<String> {
    if let Some(Meta::NameValue(name_value)) = find_attr_value_meta(attr_name, value_name, attrs) {
        if let Lit::Str(lit) = &name_value.lit {
            return Some(lit.value());
        }
    }
    None
}

pub(crate) fn has_attr_value(
    attr_name: &'static str,
    value_name: &'static str,
    attrs: &Vec<Attribute>,
) -> bool {
    find_attr_value_meta(attr_name, value_name, attrs).is_some()
}

pub(crate) fn get_value_of(name: &'static str, meta: &MetaNameValue) -> Option<String> {
    if path_is(&meta.path, name) {
        if let Lit::Str(lit) = &meta.lit {
            return Some(lit.value());
        }
    }
    None
}

fn unqualified_type_name(qualified_name: String) -> String {
    match qualified_name.rfind("::") {
        Some(pos) => qualified_name[(pos + 2)..].into(),
        None => qualified_name,
    }
}

pub(crate) fn path_is(path: &Path, expected: &str) -> bool {
    if let Some(ident) = path.get_ident() { *ident == expected } else { false }
}

pub fn fields_from(fields: &Fields) -> Vec<api_info::Field> {
    fields.iter().map(|f| field_from(f.ident.as_ref(), &f.attrs, type_from(&f.ty))).collect()
}
