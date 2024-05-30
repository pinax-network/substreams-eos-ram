use substreams_antelope::pb::ActionTrace;

pub fn to_key(action_trace: &ActionTrace) -> String {
    format!("{}-{}", action_trace.transaction_id, action_trace.action_ordinal)
}