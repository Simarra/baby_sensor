use super::schema::statements;

#[derive(Queryable)]
pub struct Statement {
    pub id: i32,
    pub temperature: f32,
    pub timestamp: i32,
}

#[derive(Insertable)]
#[table_name = "statements"]
pub struct NewStatement<'a> {
    pub id: &'a i32,
    pub temperature: &'a f32,
    pub timestamp: &'a i32,
}