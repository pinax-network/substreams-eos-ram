use substreams_antelope::pb::{ActionTrace, DbOp};

pub fn action_trace_to_key(action_trace: &ActionTrace) -> String {
    format!("{}-{}", action_trace.transaction_id, action_trace.action_ordinal)
}

pub fn db_op_to_key(db_op: &DbOp) -> String {
    format!("{}-{}-{}-{}", db_op.code, db_op.table_name, db_op.scope, db_op.action_index)
}