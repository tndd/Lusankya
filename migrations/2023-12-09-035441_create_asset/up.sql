-- Your SQL goes here

CREATE TABLE asset (
    id UUID NOT NULL,
    version TEXT NOT NULL,
    class TEXT NOT NULL,
    exchange TEXT NOT NULL,
    symbol TEXT NOT NULL,
    status TEXT NOT NULL,
    tradable BOOLEAN NOT NULL,
    marginable BOOLEAN NOT NULL,
    shortable BOOLEAN NOT NULL,
    easy_to_borrow BOOLEAN NOT NULL,
    fractionable BOOLEAN NOT NULL,
    PRIMARY KEY (id, version)
);