//! HeapSize derive macro.
//!
//! https://github.com/dtolnay/syn/blob/master/examples/heapsize/heapsize_derive/src/lib.rs

use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{
    parse_macro_input, parse_quote, Data, DeriveInput, Fields, GenericParam, Generics, Index,
};

#[proc_macro_derive(HeapSize)]
pub fn derive_heap_size(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // 将输入标记解析为语法树
    let input = parse_macro_input!(input as DeriveInput);

    // 在下面的准引语中用作 '#name '
    let name = input.ident;

    // 为所有的类型参数 T 添加限定 `T: HeapSize`
    let generics = add_trait_bounds(input.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    // 生成表达式用于所有字段堆大小的求和
    let sum = heap_size_sum(&input.data);

    let expanded = quote! {
        // 生成的 impl
        impl #impl_generics heapsize::HeapSize for #name #ty_generics #where_clause {
            fn heap_size_of_children(&self) -> usize {
                #sum
            }
        }
    };

    // 将输出标记交还给编译器
    proc_macro::TokenStream::from(expanded)
}

// 为所有的类型参数 T 添加限定 `T: HeapSize`
fn add_trait_bounds(mut generics: Generics) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param.bounds.push(parse_quote!(heapsize::HeapSize));
        }
    }
    generics
}

// 生成表达式用于所有字段堆大小的求和
fn heap_size_sum(data: &Data) -> TokenStream {
    match *data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                // 展开的表达式类似于
                //
                //      0 + self.x.heap_seize() + self.y.heap_size() + self.z.heap_size()
                //
                // 但是使用完全限定的函数调用语法。
                //
                // 这里我们需要注意每个 `syn::Field` 的展开都是符合 `heap_size_of_children` 调用的。
                // 这样可以使得任何没有实现 `HeapSize` 的字段类型会出现编译器的错误信息。
                let recurse = fields.named.iter().map(|f| {
                    let name = &f.ident;
                    quote_spanned! {f.span()=>
                        heapsize::HeapSize::heap_size_of_children(&self.#name)
                    }
                });
                quote! {
                    0 #(+ #recurse)*
                }
            }
            Fields::Unnamed(ref fields) => {
                // 展开的表达式类似于
                //
                //     0 + self.0.heap_size() + self.1.heap_size() + self.2.heap_size()
                let recurse = fields.unnamed.iter().enumerate().map(|(i, f)| {
                    let index = Index::from(i);
                    quote_spanned! {f.span() =>
                        heapsize::HeapSize::heap_size_of_children(&self.#index)
                    }
                });
                quote! {
                    0 #(+ #recurse)*
                }
            }
            Fields::Unit => {
                // 单元结构体的堆大小为 0
                quote!(0)
            }
        },
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    }
}
