use proc_macro::TokenStream;
use syn::fold::Fold;
use syn::{parse_quote, ItemFn, Ident};

struct InsertWrapperR;
impl Fold for InsertWrapperR {
    //TODO: fix this mess. Need to cleanup code and fix all dependencies in crates.
    fn fold_item_fn(&mut self, expr: ItemFn) -> ItemFn {
        let orig_name = expr.sig.ident.clone();
        let new_name = Ident::new(&format!("_RINTEROP_{}", &orig_name), orig_name.span()) ;
        let inputs = expr.sig.inputs.clone();
        let new_args = inputs.iter().enumerate().map(|(i,input)|{
            match input {
                syn::FnArg::Typed(pat) => { 
                    let ty : &syn::Type = &pat.ty;
                    let expr : syn::ExprBlock = parse_quote!({unsafe{r_args = r_args
                        .as_ref()
                        .unwrap()
                        .sxp
                        .list
                        .cdr}; <#ty as TryFromR>::try_from_r(unsafe{r_args.as_ref().unwrap().sxp.list.car}).map_err(|e| format!("Rustier encountered an error parsing argument {} : {}",#i +1,  e))? });
                    expr
                }
                _ => unimplemented!()
            }
            
        }).collect::< syn::punctuated::Punctuated<_, syn::token::Comma>>();
        
        let parsed : ItemFn = parse_quote!(
            #[no_mangle]
        pub extern "C" fn #new_name(mut r_args: *mut sexpr::Sexpr) -> *const sexpr::Sexpr {
                let call = {|| -> Result<_, ::std::string::String>{ let result = (#orig_name)(#new_args); Ok(result) }}();
                match call {
                    Ok(x) => match x.try_into_r().map_err(|e|{format!("Rustier encountered an error returning to R: {}", e)}){
                        Ok(y) => y,
                        Err(e) => {
                            unsafe{
                                Rf_error(::std::ffi::CString::new(e).as_ref().map(|s| s.as_c_str()).unwrap_or(::std::ffi::CStr::from_bytes_with_nul_unchecked(b"Rustier error when returning to R: [error could not be formatted]")).as_ptr());
                                R_NilValue
                            }
                        }
                    },
                    Err(e) => {
                        unsafe{
                            Rf_error(::std::ffi::CString::new(e).as_ref().map(|s| s.as_c_str()).unwrap_or(::std::ffi::CStr::from_bytes_with_nul_unchecked(b"Rustier error when parsing R arguments: [error could not be formatted]")).as_ptr());
                            R_NilValue
                        }
                        
                    }
                
                }
            }
        );
        parsed
        
    }
}
#[allow(non_snake_case)]
#[proc_macro_attribute]
pub fn R_export(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let tree: ItemFn = syn::parse(item.clone()).expect("has to e applied to func");
    let wrapped_fn = InsertWrapperR.fold_item_fn(tree.clone());
    let expanded :proc_macro2::TokenStream = parse_quote! {
        #tree
        #wrapped_fn
    };
    TokenStream::from(expanded)
}
