CREATE TABLE jsonb_example (
    id          integer NOT NULL,
    stringy     varchar NOT NULL,
    some_json   jsonb NOT NULL
);

INSERT INTO jsonb_example
(id,stringy,some_json)
VALUES
(1, 'hi mom', '{"hello": "there", "contract":[{"id":32, "name": "Agreement"}]}'), (2, 'hi dad', '{"hello": "to all the people",  "contract":null}');