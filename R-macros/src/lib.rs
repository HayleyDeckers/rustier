use proc_macro2::*;
use proc_macro::TokenStream;
//use syn::parse::{Parse, ParseStream};
//use syn::punctuated::Punctuated;
use syn::fold::Fold;
use syn::{parse_quote, ItemFn, FnArg, Ident, PatType};

#[derive(Debug)]
struct RustArgParsed{
    mutable : bool,
    reference : bool,
    is_slice : bool,
    typename : String
}

struct PrintIdent;
impl Fold for RustArgParsed{
    fn fold_ident(&mut self, expr: Ident) -> Ident {
        println!("got ident: {:?}", expr);
        expr
    }
    fn fold_pat_type(&mut self, expr: PatType) -> PatType {
        println!("got type {:?}", expr.ty);
        expr
    }
    fn fold_type_reference(&mut self, expr: syn::TypeReference) -> syn::TypeReference {
        self.reference = true;
        self.mutable = expr.mutability.is_some();
        expr
    }
}

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
                    // println!("input is {}", ty);
                    let expr : syn::ExprBlock = parse_quote!({unsafe{r_args = r_args
                        .as_ref()
                        .unwrap()
                        .sxp
                        .list
                        .cdr}; <#ty as FromR>::from_r(unsafe{r_args.as_ref().unwrap().sxp.list.car}).unwrap() });
                    expr
                }//this is the type, wrap it in statements of <Ty as Trait>::from_r(ptr)?;
                //exprTry{.expr = ExprCall{
                /// .func = Expr
                /// .args
                /// 
                /// 
                _ => unimplemented!()
            }
            

        // PrintIdent.fold_fn_arg(input.clone());
        }).collect::< syn::punctuated::Punctuated<_, syn::token::Comma>>();
        // inputs.iter().map(|arg : FnArg|{
        //     match arg{
        //         Receiver(_) => unimplemented!(),
        //         Typed(pat) => {
        //             pat.ty match Reference(TypeReference{copy mut}.elem match slice(Typeslice{}.elem match Path(Typepath{}.p .... ident))))
        // could use another fold to 'extract' the iden starting from the ty? PatIdent give the name in 
        //         }
        //     }
        // })
        // out.sig.ident = Ident::new(&new_name, out.sig.ident.span());
        // out.sig.abi = Some(syn::Abi)
        // let orig_code = out.block.clone();
        //need to grap args here.
        // the args passed from R can get sorted in the wrapper call. enough to do positional matching?
        // look up what call does with unhandled args.
        //
        // then loop over the types of args in the function, call in order some func on the sexpr, that extracts the n'th type, and then tries to cast it to the given type uing trait.
        // errors should trigger a stop in R, eventually.
        /*
        The R arguments start at let node = unsafe {
            self.root
                .as_ref()
                .unwrap()
                .sxp
                .list
                .cdr
                .as_ref()
                .unwrap()
        }
        where root is the arg, car is data, and cdr is the next element. names are somewhere in the tags.
        */

        /*
        List<'a>{
            raw : *mut Sexpr,
            _mark : PhantomData<&'a mut Sexpr>
        }

        impl<'a> List<'a>{
            get_item(&mut self) -> &'a mut Sexpr{
                self.raw.as_mut().unwrap(){}
            }
        }

        */
        let parsed : ItemFn = parse_quote!(
            #[no_mangle]
        pub extern "C" fn #new_name(mut r_args: *mut sexpr::Sexpr) -> *const sexpr::Sexpr {
            (#orig_name)(#new_args).return_to_r()
        }
        );
        // println!("{:#?}", parsed);
        parsed
        
    }
}

#[proc_macro_attribute]
pub fn R_export(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut tree: ItemFn = syn::parse(item.clone()).expect("has to e applied to func");
    let wrapped_fn = InsertWrapperR.fold_item_fn(tree.clone());
    //tree.sig.ident = FUNC name
    ////tree.sig.inputs[i].pat.pathIdent.ident.ident = arg name,
    //tree.sig.inputs[i].ty.elem.typePath.path.segments[0..n].ident.ident = type,
    // println!("tree: \"{:#?}\"", wrapped_fn);
    let expanded :proc_macro2::TokenStream = parse_quote! {
        #tree
        #wrapped_fn
    };
    TokenStream::from(expanded)
}
