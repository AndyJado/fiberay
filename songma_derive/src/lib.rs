use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Fields, FieldsNamed};

#[proc_macro_derive(Vertex)]
pub fn derive(input: TokenStream) -> TokenStream {
    let stru = parse_macro_input!(input as DeriveInput);
    let (iden, data) = (stru.ident, stru.data);
    let vertex_iden = iden.to_string();
    let fids = named_fields(&data);
    let bulk_properties = fids.named.iter().map(|fid| {
        let name = &fid.ident;
        let ppt_iden = name.as_ref().unwrap().to_string();
        quote! { indradb::BulkInsertItem::VertexProperty(ver.id, indradb::Identifier::new(#ppt_iden).expect("ppt_iden"), serde_json::json!(self.#name)), }
    });
    let expanded = quote! {
        impl #iden {
            pub fn vertex(&self) -> indradb::Vertex {
                indradb::Vertex::new(indradb::Identifier::new(#vertex_iden).expect("vertex_iden"))
            }
            pub fn vertex_with_property(&self) -> Vec<indradb::BulkInsertItem> {
                let ver = self.vertex();
                vec![indradb::BulkInsertItem::Vertex(ver.clone()), #(#bulk_properties)*]
            }
        }
    };
    expanded.into()
}

/// DeriveInput.data -> {x: i32, y: u8}
fn named_fields(data: &Data) -> &FieldsNamed {
    if let Data::Struct(DataStruct {
        fields: Fields::Named(named_fields),
        ..
    }) = data
    {
        named_fields
    } else {
        unimplemented!()
    }
}
