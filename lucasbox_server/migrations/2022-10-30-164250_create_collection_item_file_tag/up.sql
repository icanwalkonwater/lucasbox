CREATE TABLE users
(
    id         UUID PRIMARY KEY     DEFAULT gen_random_uuid(),
    admin      BOOLEAN     NOT NULL DEFAULT FALSE,
    username   VARCHAR(20) NOT NULL,
    password   TEXT        NOT NULL,
    updated_at TIMESTAMP   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_at TIMESTAMP   NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE user_refresh_tokens
(
    user_id    UUID      NOT NULL REFERENCES users (id),
    token      TEXT PRIMARY KEY,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE collections
(
    id          SERIAL PRIMARY KEY,
    parent_id   INTEGER   NULL REFERENCES collections (id),
    inline      BOOLEAN   NOT NULL DEFAULT FALSE,
    level       SMALLINT  NOT NULL,
    name        TEXT      NOT NULL,
    description TEXT      NULL,
    created_by  UUID      NULL REFERENCES users (id), -- Default to root user
    updated_at  TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_at  TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE collection_items
(
    id            SERIAL PRIMARY KEY,
    collection_id INTEGER   NOT NULL REFERENCES collections (id),
    name          TEXT      NOT NULL,
    description   TEXT      NULL,
    created_by    UUID      NULL REFERENCES users (id), -- Default to root user
    updated_at    TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_at    TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE item_files
(
    id                 SERIAL PRIMARY KEY,
    collection_item_id INTEGER   NOT NULL REFERENCES collection_items (id),
    name               TEXT      NOT NULL,
    filepath           TEXT      NOT NULL,
    created_by         UUID      NULL REFERENCES users (id), -- Default to root user
    updated_at         TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at         TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE tags
(
    id         SERIAL PRIMARY KEY,
    label      TEXT      NOT NULL,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE tag_collections
(
    tag_id        INTEGER NOT NULL REFERENCES tags (id),
    collection_id INTEGER NOT NULL REFERENCES collections (id),
    PRIMARY KEY (tag_id, collection_id)
);

CREATE TABLE tag_collection_items
(
    tag_id             INTEGER NOT NULL REFERENCES tags (id),
    collection_item_id INTEGER NOT NULL REFERENCES collection_items (id),
    PRIMARY KEY (tag_id, collection_item_id)
);

CREATE TABLE tag_item_files
(
    tag_id       INTEGER NOT NULL REFERENCES tags (id),
    item_file_id INTEGER NOT NULL REFERENCES item_files (id),
    PRIMARY KEY (tag_id, item_file_id)
);

SELECT diesel_manage_updated_at('users');
SELECT diesel_manage_updated_at('collections');
SELECT diesel_manage_updated_at('collection_items');
SELECT diesel_manage_updated_at('item_files');
SELECT diesel_manage_updated_at('tags');
