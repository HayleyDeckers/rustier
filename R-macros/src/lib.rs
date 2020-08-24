use proc_macro::TokenStream;
use syn::fold::Fold;
use syn::{parse_quote, ItemFn, Ident};

struct InsertWrapperR;
impl Fold for InsertWrapperR {
    fn fold_item_fn(&mut self, expr: ItemFn) -> ItemFn {
        let orig_name = expr.sig.ident.clone();
        let new_name = Ident::new(&format!("_RINTEROP_{}", &orig_name), orig_name.span()) ;
        let inputs = expr.sig.inputs.clone();
        let new_args = inputs.iter().map(|input|{
            match input {
                syn::FnArg::Typed(pat) => { 
                    let ty : &syn::Type = &pat.ty;
                    let expr : syn::ExprBlock = parse_quote!({unsafe{r_args = r_args
                        .as_ref()
                        .unwrap()
                        .sxp
                        .list
                        .cdr}; <#ty as FromR>::from_r(unsafe{r_args.as_ref().unwrap().sxp.list.car}).unwrap() });
                    expr
                }
                _ => unimplemented!()
            }
            
        }).collect::< syn::punctuated::Punctuated<_, syn::token::Comma>>();
        
        let parsed : ItemFn = parse_quote!(
            #[no_mangle]
        pub extern "C" fn #new_name(mut r_args: *mut sexpr::Sexpr) -> *const sexpr::Sexpr {
            (#orig_name)(#new_args).return_to_r()
        }
        );
        parsed
        
    }
}

#[proc_macro_attribute]
pub fn R_export(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut tree: ItemFn = syn::parse(item.clone()).expect("has to e applied to func");
    let wrapped_fn = InsertWrapperR.fold_item_fn(tree.clone());
    let expanded :proc_macro2::TokenStream = parse_quote! {
        #tree
        #wrapped_fn
    };
    TokenStream::from(expanded)
}
