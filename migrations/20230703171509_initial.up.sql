DO $$ BEGIN
    CREATE TYPE LANGUAGE AS ENUM ('English', 'Spanish', 'German', 'French');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

CREATE TABLE IF NOT EXISTS users (
    user_id UUID PRIMARY KEY,
    created TIMESTAMP WITH TIME ZONE NOT NULL,
    updated TIMESTAMP WITH TIME ZONE NOT NULL,
    username TEXT NOT NULL UNIQUE,
    CHECK (octet_length(username) <= 24),
    CHECK (octet_length(username) >= 5),
    email TEXT NOT NULL UNIQUE,
    CHECK (octet_length(email) <= 64),
    phc_string TEXT NOT NULL,
    language LANGUAGE NOT NULL,
    totp_key BYTEA
);