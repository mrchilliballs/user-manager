use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Data, Fields, ItemStruct};

#[proc_macro_attribute]
pub fn optionalize(_input: TokenStream, annotated_item: TokenStream) -> TokenStream {
    //
    let input_clone: proc_macro2::TokenStream = annotated_item.clone().into();
    //
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(annotated_item as ItemStruct);

    // Extract the name of the original struct
    let original_struct_name = &input.ident;

    // Create a new identifier for the OptionalStruct
    let optional_struct_name = syn::Ident::new(
        &format!("Optional{}", original_struct_name),
        original_struct_name.span(),
    );

    // Generate code to define the OptionalStruct
    let optionalized_struct = quote! {
        struct #optional_struct_name {
            #input
        }
    };

    // Generate code to implement conversion to OptionalStruct
    let convert_to_optional = match input.fields {
        Fields::Named(named_fields) => {
            let fields = named_fields.named.iter().map(|field| {
                let field_name = &field.ident;
                quote! {
                    #field_name: Some(#original_struct_name.#field_name)
                }
            });

            quote! {
                impl From<#original_struct_name> for #optional_struct_name {
                    fn from(orig: #original_struct_name) -> Self {
                        #optional_struct_name {
                            #(#fields,)*
                        }
                    }
                }
            }
        }
        Fields::Unnamed(_) => {
            quote! {
                compile_error!("Unsupported unnamed fields in the struct");
            }
        }
        Fields::Unit => {
            quote! {
                impl From<#original_struct_name> for #optional_struct_name {
                    fn from(_: #original_struct_name) -> Self {
                        #optional_struct_name
                    }
                }
            }
        }
    };

    // Combine the generated code
    let output = quote! {
        #input_clone
        #optionalized_struct
        #convert_to_optional
    };

    // Return the generated code as a TokenStream
    output.into()
}
