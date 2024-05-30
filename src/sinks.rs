use substreams::{errors::Error, pb::substreams::Clock, Hex};
use substreams_antelope::pb::ActionTraces;
use substreams_database_change::{change::AsString, pb::database::{table_change, DatabaseChanges}};

use crate::keys::to_key;

#[substreams::handlers::map]
pub fn db_out(clock: Clock, action_traces: ActionTraces) -> Result<DatabaseChanges, Error> {
    let mut tables = DatabaseChanges::default();
    let timestamp = clock.timestamp.unwrap().seconds.to_string();

    for action_trace in action_traces.action_traces {
        let action = action_trace.clone().action.unwrap();
        let key = to_key(&action_trace);

        tables
            .push_change("events", key, 0, table_change::Operation::Create)
            // block information
            .change("block_id", ("", clock.id.as_str()))
            .change("block_number", ("", clock.number.as_string().as_str()))
            .change("timestamp", ("", timestamp.as_str()))

            // transaction information
            .change("transaction_id", ("", action_trace.transaction_id.as_str()))
            .change("action_ordinal", ("", action_trace.action_ordinal.to_string().as_str()))

            // action information
            .change("account", ("", action.account.to_string().as_str()))
            .change("name", ("", action.name.to_string().as_str()))
            .change("json_data", ("", action.json_data.to_string().as_str()))
            .change("raw_data", ("", Hex::encode(action.raw_data).as_str()));

    }
    Ok(tables)
}
