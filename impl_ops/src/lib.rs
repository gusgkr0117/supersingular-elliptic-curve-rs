extern crate proc_macro;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use quote::ToTokens;
use syn::FnArg::Typed;
use syn::ReturnType;
use syn::{GenericParam, LifetimeParam, Lifetime};

#[proc_macro_attribute]
pub fn impl_bin_ops(_attribute : proc_macro::TokenStream, item : proc_macro::TokenStream) -> proc_macro::TokenStream {
    let impl_trait : syn::ItemImpl = syn::parse(item.clone()).unwrap();
    
    let impl_generics = impl_trait.generics.to_token_stream();
    let impl_op_name = impl_trait.trait_.clone().unwrap().1.to_token_stream();
    let impl_self_ty = impl_trait.self_ty.to_token_stream();
    let impl_where_clause = match impl_trait.generics.clone().where_clause {
        Some(x) => x.to_token_stream(),
        None => TokenStream::new()
    };

    let func : syn::ImplItemFn = syn::parse(impl_trait.items[0].to_token_stream().into()).unwrap();

    let func_ident = func.sig.ident.to_token_stream();
    let mut func_arg_rhs : TokenStream = TokenStream::new();
    let mut func_arg_rhs_ty : TokenStream = TokenStream::new();
    let func_output = func.sig.output.to_token_stream();
    let func_output_return = match func.sig.output {
        ReturnType::Type(_, b) => b,
        _ => panic!("no output type"),
    };
    let func_output_type = func_output_return.to_token_stream();

    let func_block = func.block.to_token_stream();

    for arg in func.sig.inputs.iter() {
        match arg {
            Typed(t) => {
                func_arg_rhs = t.pat.to_token_stream().into();
                func_arg_rhs_ty = t.ty.to_token_stream().into();
            },
            _ => {},
        }
    }

    let result = quote! {
        impl #impl_generics std::ops::#impl_op_name <#func_arg_rhs_ty> for #impl_self_ty #impl_where_clause {
            type Output = #func_output_type;
            fn #func_ident(self,  #func_arg_rhs : #func_arg_rhs_ty) #func_output
                #func_block
        }

        impl #impl_generics std::ops::#impl_op_name <&#func_arg_rhs_ty> for #impl_self_ty #impl_where_clause {
            type Output = #func_output_type;
            fn #func_ident(self,  #func_arg_rhs : &#func_arg_rhs_ty) #func_output
                #func_block
        }

        impl #impl_generics std::ops::#impl_op_name <#func_arg_rhs_ty> for &#impl_self_ty #impl_where_clause {
            type Output = #func_output_type;
            fn #func_ident(self,  #func_arg_rhs : #func_arg_rhs_ty) #func_output
                #func_block
        }

        impl #impl_generics std::ops::#impl_op_name <&#func_arg_rhs_ty> for &#impl_self_ty #impl_where_clause {
            type Output = #func_output_type;
            fn #func_ident(self,  #func_arg_rhs : &#func_arg_rhs_ty) #func_output
                #func_block
        }
    };

    result.into()
}