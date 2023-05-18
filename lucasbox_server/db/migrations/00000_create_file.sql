CREATE TABLE IF NOT EXISTS file
(
    id         UUID PRIMARY KEY,
    name       TEXT NOT NULL,
    filepath   TEXT NOT NULL,
    edited_at  DATE NOT NULL DEFAULT CURRENT_DATE,
    created_at DATE NOT NULL DEFAULT CURRENT_DATE
);