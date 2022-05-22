-- Your SQL goes here
-- Table: public.Microbits

-- DROP TABLE IF EXISTS public."Microbits";

CREATE TABLE IF NOT EXISTS public."Microbits"
(
    id text COLLATE pg_catalog."default" NOT NULL,
    user_id integer,
    active_begin_hours integer NOT NULL DEFAULT 0,
    active_begin_minutes integer NOT NULL DEFAULT 0,
    active_end_hours integer NOT NULL DEFAULT 0,
    active_end_minutes integer NOT NULL DEFAULT 0,
    CONSTRAINT "Microbits_pkey" PRIMARY KEY (id),
    CONSTRAINT owner FOREIGN KEY (user_id)
        REFERENCES public."Users" (id) MATCH SIMPLE
        ON UPDATE RESTRICT
        ON DELETE RESTRICT
        NOT VALID
)

TABLESPACE pg_default;

ALTER TABLE IF EXISTS public."Microbits"
    OWNER to postgres;
-- Index: fki_owner

-- DROP INDEX IF EXISTS public.fki_owner;

CREATE INDEX IF NOT EXISTS fki_owner
    ON public."Microbits" USING btree
    (user_id ASC NULLS LAST)
    TABLESPACE pg_default;

    -- Table: public.Users

-- DROP TABLE IF EXISTS public."Users";

CREATE TABLE IF NOT EXISTS public."Users"
(
    id integer NOT NULL DEFAULT nextval('"Users_id_seq"'::regclass),
    email text COLLATE pg_catalog."default" NOT NULL,
    password text COLLATE pg_catalog."default" NOT NULL,
    CONSTRAINT "Users_pkey" PRIMARY KEY (id)
)

TABLESPACE pg_default;

ALTER TABLE IF EXISTS public."Users"
    OWNER to postgres;