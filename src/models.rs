/**
 * A note of Field Order
 * 
 * Using #[derive(Queryable)] assumes that the 
 * order of fields on the **Post** struct matches
 * the columns in the **posts** table, so make
 * sure to define them in the order seen in the
 * **schema.rs** file. 
 */
#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}
