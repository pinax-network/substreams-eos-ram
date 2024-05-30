-------------------------------------------------
-- Meta tables to store Substreams information --
-------------------------------------------------

CREATE TABLE IF NOT EXISTS cursors
(
    id        String,
    cursor    String,
    block_num Int64,
    block_id  String
)
    ENGINE = ReplacingMergeTree()
        PRIMARY KEY (id)
        ORDER BY (id);

-----------------------------------------------------------
-- Tables to store the raw events without any processing --
-----------------------------------------------------------

-- The table to store all events. This uses the trx_id as first primary key so we can use this table to do
-- event lookups based on a transaction id.
CREATE TABLE IF NOT EXISTS events
(
    -- block information --
    block_id        String,
    block_number    UInt64,
    timestamp       DateTime

    -- transaction information --
    transaction_id  String,
    action_ordinal  UInt32,
    account         String,
    name            String

    -- action information --
    json_data       String,
    raw_data        String
)
    ENGINE = ReplacingMergeTree()
        PRIMARY KEY (transaction_id, action_ordinal)
        ORDER BY (transaction_id, action_ordinal);