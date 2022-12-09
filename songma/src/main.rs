use indradb::{Datastore, MemoryDatastore};
use songma::vertexes::ShearModule;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create an in-memory datastore
    let db = MemoryDatastore::default();
    let wa = ShearModule { value: 3.2 };
    let ver_p = wa.vertex_with_property();
    db.bulk_insert(ver_p)?;
    dbg!(db);
    Ok(())
}
