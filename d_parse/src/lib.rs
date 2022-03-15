use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields, Type};


#[proc_macro_derive(MyMacro)]
pub fn my_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    eprintln!("Struct name is {}", name);
    let data = input.data;

    let strct = match(data) {
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
    let mut voids_vec = vec![];
    let mut i = 1;
    for field in fieldz.named {
        eprintln!("checking field {}", i);
        i += 1;
        voids_vec.push(match_void_p(field));
    }

    eprintln!("{:?}", voids_vec);
    let expanded = quote!{
        
    };
    TokenStream::from(expanded)
}

fn match_void_p(input: syn::Field) -> bool {

    let field_ty = input.ty;
    let found = match field_ty {
        Type::Ptr(ptr) => {
            eprintln!("found ptr");
            match *ptr.elem {
                Type::Path(path) => {
                    let actual_path = path.path;
                    let mut seg_ids = vec![];
                    for seg in actual_path.segments {
                        seg_ids.push(seg.ident);
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
