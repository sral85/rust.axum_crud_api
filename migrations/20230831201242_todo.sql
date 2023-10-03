CREATE TABLE IF NOT EXISTS todo (
   id VARCHAR (255) NOT NULL PRIMARY KEY,
   status VARCHAR (255) NOT NULL DEFAULT 'open',
   description VARCHAR (255),
   created_on TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO todo (id, description)
VALUES ('homework', 'do math homework'), ('clean flat', 'vacuum carpets, clean bath');