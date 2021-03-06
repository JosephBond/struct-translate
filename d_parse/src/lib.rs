use proc_macro::TokenStream;
use quote::{quote, __private::Span, ToTokens};
use syn::{parse_macro_input, DeriveInput, Data, Fields, Type, Path, TypePath, punctuated::Punctuated, PathSegment, PathArguments, FnArg};
use core::slice::Iter;
use std::fmt::format;

#[proc_macro_derive(MyMacro)]
pub fn my_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let new_struct_name = syn::Ident::new(&format!("New{}", name), Span::call_site());
    eprintln!("Struct name is {}", name);
    let data = input.data;

    let strct = match data {
        Data::Struct(conts) => conts,
        _ => panic!("Error, encountered something other than a struct!"),
    };

    eprintln!("Constructing fields!");
    let fieldz = match strct.fields {
        Fields::Named(fields) => {
            fields
        }
        _ => panic!("found the wrong type of fields!"),
    };

    eprintln!("Constructed fields!");
    let mut generator = FreshNameGen::new();
    let mut new_names = Vec::new();
    let mut new_types = Vec::new();

    let mut i = 1;
    for field in fieldz.named {
        eprintln!("checking field {}", i);
        eprintln!("{:?}", ty_enum_constructor_to_string(field.ty.clone()));
        i += 1;
        let (new_n, new_t) = update_field(field, &mut generator);
        new_names.push(new_n);
        new_types.push(new_t);

    }

    let new_types_2 = generator.output_all_names();
    eprintln!("{:?}", new_types);
    let expanded = quote!{
        struct #new_struct_name  < #(#new_types_2),* > {
            #(#new_names: #new_types),*
        }
    };
    eprintln!("{:?}", expanded.to_string());
    TokenStream::from(expanded)
}
fn update_field(old: syn::Field, generator: &mut FreshNameGen) -> (String, String) {
    if match_void_p_field(&old) {
        let new_type = format!("Box<Option<{}>>", generator.next_name());
        (old.ident.unwrap().to_string(), new_type)
    } else {
        (old.ident.unwrap().to_string(), type_to_string(&old.ty))
    }
} 
fn type_to_string(input_type: &syn::Type) -> String {
    input_type.to_token_stream().to_string()
}
fn _get_idents(input: Iter<syn::Field>) -> Vec<String> {
    let mut new_ids = Vec::new();
    for field in input {
        new_ids.push(field.ident.as_ref().unwrap().to_string());
    }
    new_ids
}
fn match_void_p_field(input: &syn::Field) -> bool {
    match_void_p(&input.ty)
}

fn match_void_p(input: &syn::Type) -> bool {
    let found = match input {
        Type::Ptr(ptr) => {
            eprintln!("found ptr");
            match &*ptr.elem {
                Type::Path(path) => {
                    let actual_path = &path.path;
                    let mut seg_ids = vec![];
                    for seg in &actual_path.segments {
                        seg_ids.push(seg.ident.clone());
                    }
                    if seg_ids == ["libc", "c_void" ] {
                        true
                    } else {
                        false
                    }
                }
                _ => false,
            }
        }
        _ => { eprintln!("Did not find pointer type"); false},
    };
    found
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

struct FreshNameGen {
    count: usize,
    used_names: Vec<String>,
}

impl FreshNameGen {
    pub fn new() -> Self {
        FreshNameGen {
            count: 0,
            used_names: Vec::new(),
        }
    }

    pub fn next_name(&mut self) -> String {
        let out = format!("X{}", self.count);
        self.count = self.count + 1;
        self.used_names.push(out.clone());
        out
    }

    pub fn output_all_names(&self) -> Vec<String> {
        self.used_names.clone()
    }
}

fn func_def_translate(item: syn::ItemFn) -> String {
    let mut out = String::new();
    let sig = item.sig;

    for arg in sig.inputs {
        match arg {
            FnArg::Receiver(_arg) => {
                out.push_str("self, ")
            },
            FnArg::Typed(arg) => {
                if let syn::Pat::Ident(narg) = *arg.pat {
                    let mut ty_str = String::new();
                    ty_str.push_str(&arg.ty.to_token_stream().to_string());
                    out.push_str(&format!("{} : {} ", narg.ident.to_string(), ty_str)
                    );
                } else {
                    panic!();
                }
            },
        }
    }

    out
}

fn process_fn_arg_ty(arg: syn::FnArg, mut type_gen: FreshNameGen) -> String {
    let mut out = String::new();
    out.push_str("(");
    if let FnArg::Typed(pat_ty) = arg {
        let arg_id = pat_ty.to_token_stream().to_string();

        out.push_str(&arg_id);
        out.push_str(", ");
        
        if match_void_p(&pat_ty.ty) {
            out.push_str(&type_gen.next_name());
        }  
        else {
    
        }
        
        out.push(')');
        out
    }
    else {
        String::from("new")
    }
}

// fn new_boxed_ty(namer: FreshNameGen) -> syn::Type {
//     syn::Type:: {

//     }
// }

fn ty_enum_constructor_to_string(input: syn::Type) -> String {
    let mut out = String::new();

    match input {
        Type::Array(_) => out.push_str("arr"),
        Type::BareFn(_) => out.push_str("barefn"),
        Type::Group(_) => out.push_str("group"),
        Type::ImplTrait(_) => out.push_str("implT"),
        Type::Infer(_) => out.push_str("inf"),
        Type::Macro(_) => out.push_str("macro"),
        Type::Never(_) => out.push_str("never"),
        Type::Paren(_) => out.push_str("paren"),
        Type::Path(_) => out.push_str("path"),
        Type::Ptr(_) => out.push_str("ptr"),
        Type::Reference(_) => out.push_str("ref"),
        Type::Slice(_) => out.push_str("slice"),
        Type::TraitObject(_) => out.push_str("traitobj"),
        Type::Tuple(_) => out.push_str("tuple"),
        Type::Verbatim(_) => out.push_str("verb"),
        _ => out.push_str("wtaf"),
    }
    out
}