use crate::conditions::ConditionData;
use mongodb::bson::doc;

pub struct MongoDBConditionsRepo {
    collection: mongodb::Collection<ConditionData>,
}

impl MongoDBConditionsRepo {
    pub fn new(
        client: mongodb::Client,
        db_name: String,
        collection_name: String,
    ) -> mongodb::error::Result<Self> {
        let db = client.database(&db_name);
        let collection = db.collection::<ConditionData>(&collection_name);
        Ok(MongoDBConditionsRepo { collection })
    }

    pub async fn insert_one(&self, condition: ConditionData) -> mongodb::error::Result<()> {
        self.collection.insert_one(condition).await?;
        Ok(())
    }

    pub async fn find_latest(
        &self,
        device_id: &str,
    ) -> mongodb::error::Result<Option<ConditionData>> {
        let filter = doc! { "device_id": device_id };

        let result = self
            .collection
            .find_one(filter)
            .sort(doc! { "timestamp": -1 })
            .await?;

        Ok(result)
    }
}

impl Clone for MongoDBConditionsRepo {
    fn clone(&self) -> Self {
        MongoDBConditionsRepo {
            collection: self.collection.clone(),
        }
    }
}

pub fn create_mongodb_conditions_repo(
    client: mongodb::Client,
    db_name: String,
    collection_name: String,
) -> mongodb::error::Result<MongoDBConditionsRepo> {
    MongoDBConditionsRepo::new(client, db_name, collection_name)
}
