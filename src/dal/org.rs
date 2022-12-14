use diesel::prelude::*;
use diesel::{self, AsChangeset, Insertable, Queryable};
use serde_derive::{Deserialize, Serialize};

use crate::{db::establish_connection, schema::orgs};

#[derive(AsChangeset, Queryable, Serialize)]
pub struct Org {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub url: Option<String>,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Deserialize, Insertable)]
#[table_name = "orgs"]
pub struct NewOrg {
    pub name: String,
    pub description: String,
    pub url: String,
}

#[derive(Deserialize)]
pub struct UpdatedOrg {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub url: String,
}

impl Org {
    pub fn create(organization: &NewOrg) -> Org {
        let connection = establish_connection();
        let _created_successfully = diesel::insert_into(orgs::table)
            .values(organization)
            .execute(&connection)
            .is_ok();

        println!("hello");

        orgs::table
            .order(orgs::id.desc())
            .first(&connection)
            .unwrap()
    }

    pub fn read() -> Vec<Org> {
        let connection = establish_connection();
        orgs::table.load::<Org>(&connection).unwrap()
    }

    pub fn update(org: &UpdatedOrg) -> Org {
        let connection = establish_connection();
        let org_id = org.id;
        let name = &org.name;
        let description = &org.description;
        let url = &org.url;

        let _updated_successfully = diesel::update(orgs::table.filter(orgs::id.eq(org_id)))
            .set((
                orgs::name.eq(&name),
                orgs::description.eq(&description),
                orgs::url.eq(&url),
            ))
            .execute(&connection)
            .is_ok();

        Org::get_by_name(name).unwrap()
    }

    pub fn delete(record_id: i64) -> bool {
        let connection = establish_connection();
        diesel::delete(orgs::table.filter(orgs::id.eq(record_id)))
            .execute(&connection)
            .is_ok()
    }

    pub fn get_by_name(name: &str) -> Option<Org> {
        let connection = establish_connection();
        let results = orgs::table
            .filter(orgs::name.eq(name))
            .limit(1)
            .load::<Org>(&connection)
            .expect("Error reading orgs");

        results.into_iter().next()
    }
}
