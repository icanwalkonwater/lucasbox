-- This file was automatically created by Diesel to setup helper functions
-- and other internal bookkeeping. This file is safe to edit, any future
-- changes will be added to existing projects as new migrations.


-- Sets up a trigger for the given table to automatically set a column called
-- `updated_at` whenever the row is modified (unless `updated_at` was included
-- in the modified columns)
--
-- # Example
--
-- ```sql
-- CREATE TABLE users (id SERIAL PRIMARY KEY, updated_at TIMESTAMP NOT NULL DEFAULT NOW());
--
-- SELECT diesel_manage_updated_at('users');
-- ```
CREATE OR REPLACE FUNCTION diesel_manage_updated_at(_tbl REGCLASS) RETURNS VOID AS
$$
BEGIN
    EXECUTE FORMAT('CREATE TRIGGER set_updated_at BEFORE UPDATE ON %s
                    FOR EACH ROW EXECUTE PROCEDURE diesel_set_updated_at()', _tbl);
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION diesel_set_updated_at() RETURNS TRIGGER AS
$$
BEGIN
    IF (
            new IS DISTINCT FROM old AND
            new.updated_at IS NOT DISTINCT FROM old.updated_at
        ) THEN
        new.updated_at := CURRENT_TIMESTAMP;
    END IF;
    RETURN new;
END;
$$ LANGUAGE plpgsql;
