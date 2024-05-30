use substreams::{errors::Error, pb::substreams::Clock, Hex};
use substreams_antelope::pb::{ActionTraces, DbOps};
use substreams_database_change::{change::AsString, pb::database::{table_change, DatabaseChanges}};

use crate::keys::{action_trace_to_key, db_op_to_key};

#[substreams::handlers::map]
pub fn db_out(clock: Clock, map_events: ActionTraces, map_changes: DbOps) -> Result<DatabaseChanges, Error> {
    let mut tables = DatabaseChanges::default();
    let timestamp = clock.timestamp.unwrap().seconds.to_string();

    for action_trace in map_events.action_traces {
        let action = action_trace.clone().action.unwrap();
        let key = action_trace_to_key(&action_trace);

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

    for db_op in map_changes.db_ops {
        let key = db_op_to_key(&db_op);

        tables
            .push_change("changes", key, 0, table_change::Operation::Create)
            // block information
            .change("block_id", ("", clock.id.as_str()))
            .change("block_number", ("", clock.number.as_string().as_str()))
            .change("timestamp", ("", timestamp.as_str()))

            // transaction information
            .change("operation", ("", db_op.operation.to_string().as_str()))
            .change("action_index", ("", db_op.action_index.to_string().as_str()))
            .change("code", ("", db_op.code.to_string().as_str()))
            .change("scope", ("", db_op.scope.to_string().as_str()))
            .change("table_name", ("", db_op.table_name.to_string().as_str()))
            .change("primary_key", ("", db_op.primary_key.to_string().as_str()))

            // change information
            .change("new_data", ("", Hex::encode(db_op.new_data).as_str()))
            .change("new_data_json", ("", db_op.new_data_json.to_string().as_str()))
            .change("new_payer", ("", Hex::encode(db_op.new_payer).as_str()))
            .change("old_data", ("", Hex::encode(db_op.old_data).as_str()))
            .change("old_data_json", ("", db_op.old_data_json.to_string().as_str()))
            .change("old_payer", ("", Hex::encode(db_op.old_payer).as_str()))
            ;
    }
    Ok(tables)
}
