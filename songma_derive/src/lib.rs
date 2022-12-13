use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Fields, FieldsNamed};

#[proc_macro_derive(Vertex)]
pub fn vertex_derive(input: TokenStream) -> TokenStream {
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
            pub fn iden() ->indradb::Identifier {
                indradb::Identifier::new(#vertex_iden).expect("vertex_iden")
            }

            pub fn vertex(&self) -> indradb::Vertex {
                indradb::Vertex::new(Self::iden())
            }

            pub fn vertex_with_property(&self) -> Vec<indradb::BulkInsertItem> {
                let ver = self.vertex();
                vec![indradb::BulkInsertItem::Vertex(ver.clone()), #(#bulk_properties)*]
            }
        }
    };
    expanded.into()
}

#[proc_macro_derive(EdgeKey)]
pub fn edge_derive(input: TokenStream) -> TokenStream {
    let stru = parse_macro_input!(input as DeriveInput);
    let (iden, data) = (stru.ident, stru.data);
    let edgekey_iden = iden.to_string();
    let fids_or_none = field_or_unit(&data);
    let edge_ppt_method = if let Some(fids) = fids_or_none {
        // edgeproperty
        let bulk_properties = fids.named.iter().map(|fid| {
            let field_name = &fid.ident;
            let ppt_iden = field_name.as_ref().unwrap().to_string();
            quote! {
                indradb::BulkInsertItem::EdgeProperty(key.clone(), indradb::Identifier::new(#ppt_iden).expect("ppt_iden"), serde_json::json!(self.#field_name)),
            }
        });
        // method
        quote! {
            pub fn edge_with_property(&self,out_id: uuid::Uuid, in_id: uuid::Uuid) -> Vec<indradb::BulkInsertItem> {
                let key = self.edgekey(out_id, in_id);
                vec![indradb::BulkInsertItem::Edge(key.clone()), #(#bulk_properties)*]
            }
        }
    } else {
        quote! {}
    };
    let expanded = quote! {
        impl #iden {
            pub fn edgekey(&self,out_id: uuid::Uuid, in_id: uuid::Uuid) -> indradb::EdgeKey {
                indradb::EdgeKey::new(out_id,indradb::Identifier::new(#edgekey_iden).expect("edge_iden failed!"),in_id)
            }
            #edge_ppt_method
        }
    };
    expanded.into()
}

/// FIXME: assume you don't even tuple struct
fn field_or_unit(data: &Data) -> Option<&FieldsNamed> {
    if let Data::Struct(DataStruct {
        fields: Fields::Named(named_fields),
        ..
    }) = data
    {
        Some(named_fields)
    } else {
        None
    }
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
