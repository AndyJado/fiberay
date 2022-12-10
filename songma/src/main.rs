use indradb::{BulkInsertItem, Datastore, MemoryDatastore, Vertex};
use songma::{
    edges::Calculate,
    vertexes::{ShearModule, YoungsModule},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create an in-memory datastore
    let db = MemoryDatastore::default();
    // vertex
    let shear_ver = ShearModule { value: 3.2 };
    let young_ver = YoungsModule {
        value: 9821391.12,
        degree: 270,
    };
    let ver_p = shear_ver.vertex_with_property();
    let ver_young = young_ver.vertex_with_property();
    let BulkInsertItem::Vertex(ref ver) = ver_p[0] else {panic!("not vertex!")};
    // edge
    let eg = Calculate;
    let ed_to = eg.edgekey(ver.id.clone(), ver.id.clone());
    dbg!(eg);
    // database
    db.bulk_insert(ver_p)?;
    dbg!(db);
    Ok(())
}
