use std::collections::HashSet;

use substreams::errors::Error;
// use substreams::log;
use substreams_antelope::pb::{ActionTraces, DbOps};
use substreams_antelope::Block;

#[substreams::handlers::map]
fn map_events(params: String, block: Block) -> Result<ActionTraces, Error> {
    let mut allowed_actions = HashSet::new();

    // add additional actions to allowed_actions by using params
    // params "map_events=eosio::buyram,eosio::sellram"
    // split by ","
    // insert into allowed_actions
    for action in params.split(",") {
        allowed_actions.insert(action.to_string());
    }

    // BRAM ram.defi
    allowed_actions.insert("ram.defi::create".to_string());
    allowed_actions.insert("ram.defi::depositlog".to_string());
    allowed_actions.insert("ram.defi::depositram".to_string());
    allowed_actions.insert("ram.defi::transfer".to_string());
    allowed_actions.insert("ram.defi::transferlog".to_string());
    allowed_actions.insert("ram.defi::updateratio".to_string());
    allowed_actions.insert("ram.defi::updatestatus".to_string());
    allowed_actions.insert("ram.defi::withdrawlog".to_string());
    allowed_actions.insert("ram.defi::withdrawram".to_string());

    // WRAM eosio.wram
    allowed_actions.insert("eosio.wram::transfer".to_string());
    allowed_actions.insert("eosio.wram::unwrap".to_string());
    allowed_actions.insert("eosio.wram::retire".to_string());
    allowed_actions.insert("eosio.wram::create".to_string());
    allowed_actions.insert("eosio.wram::issue".to_string());

    // RAMS newrams.eos
    allowed_actions.insert("newrams.eos::transfer".to_string());
    allowed_actions.insert("newrams.eos::transferlog".to_string());
    allowed_actions.insert("newrams.eos::retire".to_string());
    allowed_actions.insert("newrams.eos::create".to_string());
    allowed_actions.insert("newrams.eos::issue".to_string());

    // eosio
    allowed_actions.insert("eosio::setram".to_string());
    allowed_actions.insert("eosio::setramrate".to_string());
    allowed_actions.insert("eosio::buyram".to_string());
    allowed_actions.insert("eosio::sellram".to_string());
    allowed_actions.insert("eosio::logbuyram".to_string());
    allowed_actions.insert("eosio::logsellram".to_string());
    allowed_actions.insert("eosio::logramchange".to_string());
    allowed_actions.insert("eosio::logsystemfee".to_string());
    allowed_actions.insert("eosio::ramburn".to_string());
    allowed_actions.insert("eosio::setramrate".to_string());
    allowed_actions.insert("eosio::setram".to_string());
    allowed_actions.insert("eosio::ramtransfer".to_string());
    allowed_actions.insert("eosio::buyramsefl".to_string());
    allowed_actions.insert("eosio::buyrambytes".to_string());
    allowed_actions.insert("eosio::newaccount".to_string());

    let action_traces = block.into_action_traces().filter_map(|action_trace| {
        let action = action_trace.clone().action.unwrap();
        let key = format!("{}::{}", action.account.clone().to_string(), action.name.to_string());
        if !allowed_actions.contains(&key) { return None; }
        if action_trace.receiver != action.account { return None; } // filter out extra inline actions
        // log::debug!("key: {:?}", key);
        return Some(action_trace)

    }).collect::<Vec<_>>();

    Ok( ActionTraces{action_traces} )
}


#[substreams::handlers::map]
fn map_changes(params: String, block: Block) -> Result<DbOps, Error> {
    let mut allowed_table_changes = HashSet::new();

    // add additional actions to allowed_table_changes by using params
    // params "map_changes=eosio::userres,eosio::global1"
    // split by ","
    // insert into allowed_table_changes
    for action in params.split(",") {
        allowed_table_changes.insert(action.to_string());
    }

    // BRAM ram.defi
    allowed_table_changes.insert("ram.defi::accounts".to_string());
    allowed_table_changes.insert("ram.defi::stat".to_string());
    allowed_table_changes.insert("ram.defi::config".to_string());

    // WRAM eosio.wram
    allowed_table_changes.insert("eosio.wram::accounts".to_string());
    allowed_table_changes.insert("eosio.wram::stat".to_string());

    // RAMS newrams.eos
    allowed_table_changes.insert("newrams.eos::accounts".to_string());
    allowed_table_changes.insert("newrams.eos::stat".to_string());

    // eosio
    allowed_table_changes.insert("eosio::userres".to_string());
    // allowed_table_changes.insert("eosio::global".to_string());
    // allowed_table_changes.insert("eosio::global2".to_string());

    let mut db_ops = Vec::new();
    for transaction_trace in block.into_transaction_traces() {
        for db_op in transaction_trace.db_ops {
            let key = format!("{}::{}", db_op.code.clone().to_string(), db_op.table_name.clone().to_string());
            if !allowed_table_changes.contains(&key) { continue; }
            db_ops.push(db_op);
        }
    }

    Ok( DbOps{db_ops} )
}
