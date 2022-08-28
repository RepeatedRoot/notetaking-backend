use mongodb::{
    bson::{doc, oid::ObjectId},
    {Client, Collection}
};
use serde::{Serialize, de::DeserializeOwned};
use std::error::Error;
use bson::{Bson, document::Document};

/* A struct to store database management variables */
#[derive(Debug, Clone)]
pub struct DatabaseMgr {
    pub client: Client,
    pub coll: Collection<Document>
}

/* Functions for database management */
impl DatabaseMgr {
    /* Intialise the datbase manager struct with a connection reference */
    pub async fn new(db_uri: &str, db_name: &str, coll_name: &str) -> Result<Self, Box<dyn Error>> {
        let client: Client = Client::with_uri_str(db_uri).await?;   //create a new connection, return any errors if encountered
        let coll: Collection<Document> = client.database(db_name).collection(coll_name);

        Ok(Self { client, coll } )  //Everything worked as expected, return the struct
    }

    /* Serialise a struct and insert the resulting document into the database */
    pub async fn db_insert_document<T: Serialize>(&self, structure: &T) -> Result<ObjectId, Box<dyn Error>> {
        let serialized_client: Bson = bson::to_bson(&structure)?;
        let document: &Document = serialized_client.as_document().unwrap();
        let insert_result = self.coll.insert_one(document, None).await?;
        let client_id: ObjectId = insert_result.inserted_id
            .as_object_id()
            .expect("Retrieved _id should have been of type ObjectId");
        
        Ok(client_id)
    }   

    pub async fn db_find_document<T: DeserializeOwned>(&self, document_id: &ObjectId) -> Result<T, Box<dyn Error>> {
        let loaded_document = self.coll.find_one(Some(doc! {"_id": document_id }), None)
            .await?
            .expect("Document not found");
        
        let loaded_document_struct = bson::from_bson(Bson::Document(loaded_document))?;
        println!("Document loaded from collection");
        Ok(loaded_document_struct)
    }

    /* Remove a document by it's _id */
    pub async fn db_delete_document(&self, id: &ObjectId) -> Result<(), Box<dyn Error>> {
        self.coll.delete_one(doc! {"_id": id}, None).await?;
        Ok(())
    }
}
