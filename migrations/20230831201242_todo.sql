CREATE TABLE IF NOT EXISTS todo (
   todo_id serial PRIMARY KEY,
   description VARCHAR (255),
   created_on TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO todo (description)
VALUES ('do homework'), 
('clean flat');