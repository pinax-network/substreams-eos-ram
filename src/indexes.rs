use substreams::errors::Error;
use substreams::pb::sf::substreams::index::v1::Keys;
use substreams_antelope::pb::ActionTraces;

#[substreams::handlers::map]
fn index_transactions(action_traces: ActionTraces) -> Result<Keys, Error> {
    Ok(match action_traces.action_traces.is_empty() {
        true => Keys::default(),
        false => Keys {
            keys: vec!["eos-ram".to_string()],
        },
    })
}
