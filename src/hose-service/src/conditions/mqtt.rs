use crate::conditions::{ConditionData, LivingRoomConditionUpdated, MongoDBConditionsRepo};
use log;

pub async fn handle_living_room_conditions_update(
    payload: &str,
    conditions_repo: &MongoDBConditionsRepo,
) {
    let data = serde_json::from_str::<LivingRoomConditionUpdated>(payload).unwrap();
    log::info!("Living room conditions updated: {}", payload);

    let condition_data: ConditionData = ConditionData::from(data);

    conditions_repo
        .insert_one(condition_data)
        .await
        .expect("Failed to insert condition data into MongoDB");
}
