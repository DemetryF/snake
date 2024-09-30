use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

#[proc_macro_attribute]
pub fn id(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);

    assert!(
        matches!(ast.data, Data::Struct(_)),
        "only structs can be made id"
    );

    let struct_name = ast.ident;

    quote! {
        #[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct #struct_name(u32);

        impl #struct_name {
            pub(crate) fn new(id: u32) -> Self {
                Self(id)
            }
        }

        impl Into<u32> for #struct_name {
            fn into(self) -> u32 {
                self.0
            }
        }
    }
    .into()
}
