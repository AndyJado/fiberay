use indradb::{Datastore, Identifier, RocksdbDatastore, SpecificVertexQuery, Vertex};
use serde_json::json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = &"./image.rdb";
    // Create an in-memory datastore
    let db = RocksdbDatastore::new(db_path, None)?;
    // default create vertex bulk
    let v_iden = Identifier::new("sample")?;
    let ppt = Identifier::new("idsdad")?;
    let vv = Vertex::new(v_iden);
    db.create_vertex(&vv)?;
    let v_q = SpecificVertexQuery::single(vv.id);
    db.set_vertex_properties(
        indradb::VertexPropertyQuery {
            inner: v_q.into(),
            name: ppt,
        },
        serde_json::Value::Bool(true),
    )?;
    // derive insert

    println!("{:#?}", db);
    db.sync()?;
    Ok(())
}
