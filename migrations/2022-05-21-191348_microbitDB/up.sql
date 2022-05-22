-- Your SQL goes here
CREATE TABLE public."Users"
(
    id serial NOT NULL,
    email text NOT NULL,
    password text NOT NULL,
    PRIMARY KEY (id)
);


    CREATE TABLE public."Microbits"
(
    id text NOT NULL,
    user_id integer,
    active_begin_hours integer NOT NULL DEFAULT 0,
    active_begin_minutes integer NOT NULL DEFAULT 0,
    active_end_hours integer NOT NULL DEFAULT 0,
    active_end_minutes integer NOT NULL DEFAULT 0,
    PRIMARY KEY (id),
    CONSTRAINT "Owner" FOREIGN KEY (user_id)
        REFERENCES public."Users" (id) MATCH SIMPLE
        ON UPDATE RESTRICT
        ON DELETE RESTRICT
        NOT VALID
);
