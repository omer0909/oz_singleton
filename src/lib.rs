use proc_macro::TokenStream;
use quote::quote;
use syn::ItemStruct;

#[proc_macro_attribute]
pub fn singleton(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input: ItemStruct = syn::parse_macro_input!(item as ItemStruct);
    let name = &input.ident;
    let error_str = format!("Singleton {} not initialized!", name.to_string());

    let expanded = quote! {
        #input

        impl #name {
            fn instance_(instance: Option<Self>) -> &'static std::sync::RwLock<Self> {
                static INSTANCE: std::sync::OnceLock<std::sync::RwLock<#name>> = std::sync::OnceLock::new();
                if let Some(value) = instance {
                    let _ = INSTANCE.set(std::sync::RwLock::new(value));
                }
                INSTANCE.get().expect(#error_str)
            }

            pub fn initialize(instance: Self) {
                let _ = #name::instance_(Some(instance));
            }

            pub fn w() -> std::sync::RwLockWriteGuard<'static, Self> {
                #name::instance_(None).write().unwrap()
            }

            pub fn r() -> std::sync::RwLockReadGuard<'static, Self> {
                #name::instance_(None).read().unwrap()
            }
        }

    };
    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn singleton_unsafe(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input: ItemStruct = syn::parse_macro_input!(item as ItemStruct);
    let name = &input.ident;

    let expanded = quote! {
        #input

        impl #name {
            fn instance_() -> *mut Option<Self> {
                static mut INSTANCE: Option<#name> = None;
                unsafe { std::ptr::addr_of_mut!(INSTANCE) }
            }

            pub fn g() -> &'static mut Self {
                unsafe { (*Self::instance_()).as_mut().unwrap() }
            }

            pub fn initialize(instance: Self) {
                unsafe {
                    *Self::instance_() = Some(instance);
                }
            }

        }
    };
    TokenStream::from(expanded)
}
