use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub hubs: Vec<Hub>,
    pub accounts: Vec<Account>,
    pub schedules: Vec<Value>,
    #[serde(rename = "applied_filter")]
    pub applied_filter: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hub {
    #[serde(rename = "hub_type")]
    pub hub_type: String,
    pub id: String,
    #[serde(rename = "account_id")]
    pub account_id: i64,
    pub name: String,
    pub latitude: String,
    pub longitude: String,
    pub radius: i64,
    #[serde(rename = "maximum_capacity")]
    pub maximum_capacity: i64,
    #[serde(rename = "available_vehicles_count")]
    pub available_vehicles_count: i64,
    #[serde(rename = "vehicles_count")]
    pub vehicles_count: i64,
    #[serde(rename = "available_vehicles")]
    pub available_vehicles: AvailableVehicles,
    #[serde(rename = "allowed_vehicle_types")]
    pub allowed_vehicle_types: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AvailableVehicles {
    pub bike: i64,
    pub ebike: i64,
    pub escooter: i64,
    pub trailer: i64,
    pub cargo: i64,
    pub ecargo: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub id: i64,
    #[serde(rename = "supported_vehicle_types")]
    pub supported_vehicle_types: Vec<String>,
    #[serde(rename = "collect_consent")]
    pub collect_consent: bool,
    #[serde(rename = "day_deals")]
    pub day_deals: Vec<Value>,
    pub currency: String,
    #[serde(rename = "relocation_fees")]
    pub relocation_fees: RelocationFees,
    #[serde(rename = "lost_vehicle_fees")]
    pub lost_vehicle_fees: LostVehicleFees,
    #[serde(rename = "lost_insured_vehicle_fees")]
    pub lost_insured_vehicle_fees: LostInsuredVehicleFees,
    #[serde(rename = "other_fees")]
    pub other_fees: OtherFees,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RelocationFees {
    #[serde(rename = "500")]
    pub n500: String,
    #[serde(rename = "5000")]
    pub n5000: String,
    #[serde(rename = "50000")]
    pub n50000: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LostVehicleFees {
    pub bike: String,
    pub ebike: String,
    pub escooter: String,
    pub trailer: String,
    pub cargo: String,
    pub ecargo: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LostInsuredVehicleFees {
    pub bike: String,
    pub ebike: String,
    pub cargo: String,
    pub ecargo: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OtherFees {
    #[serde(rename = "bike_lost_1_months")]
    pub bike_lost_1_months: String,
    #[serde(rename = "bike_lost_2_months")]
    pub bike_lost_2_months: String,
    #[serde(rename = "debt_interest")]
    pub debt_interest: String,
}
