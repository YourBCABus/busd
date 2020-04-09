table! {
    accesstokens (token_hash) {
        token_hash -> Bytea,
        client_id -> Uuid,
        user_id -> Uuid,
        scopes -> Array<Text>,
        expires -> Timestamp,
    }
}

table! {
    authclientowners (client_id, owner_id) {
        client_id -> Uuid,
        owner_id -> Uuid,
    }
}

table! {
    authclients (client_id) {
        client_id -> Uuid,
        client_secret -> Bytea,
        meta -> Nullable<Jsonb>,
    }
}

table! {
    authorizedclients (client_id, user_id) {
        client_id -> Uuid,
        user_id -> Uuid,
        scopes -> Array<Text>,
    }
}

table! {
    authproviders (id) {
        id -> Uuid,
        user_id -> Uuid,
        provider -> Varchar,
        email -> Nullable<Text>,
        subject -> Nullable<Text>,
        meta -> Nullable<Jsonb>,
    }
}

table! {
    botowners (bot_id, owner_id) {
        bot_id -> Uuid,
        owner_id -> Uuid,
    }
}

table! {
    refreshtokens (token_hash) {
        token_hash -> Bytea,
        client_id -> Uuid,
        user_id -> Uuid,
        scopes -> Array<Text>,
        expires -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Uuid,
        is_bot -> Bool,
        bot_token_hash -> Nullable<Bytea>,
        user_token_hash -> Nullable<Bytea>,
        is_superadmin -> Bool,
        meta -> Nullable<Jsonb>,
    }
}

joinable!(accesstokens -> authclients (client_id));
joinable!(accesstokens -> users (user_id));
joinable!(authclientowners -> authclients (client_id));
joinable!(authclientowners -> users (owner_id));
joinable!(authorizedclients -> authclients (client_id));
joinable!(authorizedclients -> users (user_id));
joinable!(authproviders -> users (user_id));
joinable!(refreshtokens -> authclients (client_id));
joinable!(refreshtokens -> users (user_id));

allow_tables_to_appear_in_same_query!(
    accesstokens,
    authclientowners,
    authclients,
    authorizedclients,
    authproviders,
    botowners,
    refreshtokens,
    users,
);
