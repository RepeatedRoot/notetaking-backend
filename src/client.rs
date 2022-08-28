use std::error::Error;

use mongodb::{
    bson::{doc, oid::ObjectId},
    {Client, Collection},
    Database
};
use serde::{Serialize, de::DeserializeOwned};
use bson::{Bson, document::Document};

/* A struct to store database management variables */
#[derive(Debug, Clone)]
pub struct DatabaseMgr {
    pub client: Client,
    pub db: Database,
}

/* Functions for database management (CRUD) */
impl DatabaseMgr {
    /* Intialise the datbase manager struct with a connection reference */
    pub async fn new(db_uri: &str, db_name: &str) -> Result<Self, Box<dyn Error>> {
        let client: Client = Client::with_uri_str(db_uri).await?;   //create a new connection, return any errors if encountered
        let db: Database = client.database(db_name);

        Ok(Self { client, db} )  //Everything worked as expected, return the struct
    }

    /* Serialise a struct and insert the resulting document into the database */
    pub async fn db_insert_document<T: Serialize>(&self, structure: &T, coll_name: &str) -> Result<ObjectId, Box<dyn Error>> {
        let collection: Collection<Document> = self.db.collection(coll_name);
        
        let bson_document: Bson = bson::to_bson(&structure)?;
        let document: &Document = bson_document.as_document().unwrap();
        let insert_result = collection.insert_one(document, None).await?;
        let client_id: ObjectId = insert_result.inserted_id
            .as_object_id()
            .expect("Retrieved _id should have been of type ObjectId");
        
        Ok(client_id)
    }   

    /* Find a document in the database given it's _id, deserialize to a struct */
    pub async fn db_find_document<T: DeserializeOwned>(&self, document_id: &ObjectId, coll_name: &str) -> Result<T, Box<dyn Error>> {
        let collection: Collection<Document> = self.db.collection(coll_name);
        
        let loaded_document = collection.find_one(Some(doc! {"_id": document_id }), None)
            .await?
            .expect("Document not found");
        
        let loaded_document_struct = bson::from_bson(Bson::Document(loaded_document))?;
        println!("Document loaded from collection");
        Ok(loaded_document_struct)
    }

    pub async fn db_update_document<T: Serialize + DeserializeOwned>(&self, document_id: &ObjectId, structure: T, coll_name: &str) -> Result<(), Box<dyn Error>> {
        let collection: Collection<Document> = self.db.collection(coll_name);

        let bson_document: Bson = bson::to_bson(&structure)?;
        let document: &Document = bson_document.as_document().unwrap();

        println!("Document to update with: {:?}", document);

        let update_result = collection.update_one(
            doc ! { "_id": document_id},
            doc! { "$set": document.to_owned() },
            None
        ).await?;

        let upserted_id = update_result.upserted_id;

        println!("Upserted id: {:?}", upserted_id);

        Ok(())
    }

    /* Remove a document by it's _id */
    pub async fn db_delete_document(&self, id: &ObjectId, coll_name: &str) -> Result<(), Box<dyn Error>> {
        let collection: Collection<Document> = self.db.collection(coll_name);
        
        collection.delete_one(doc! {"_id": id}, None).await?;
        Ok(())
    }
}
