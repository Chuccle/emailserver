-- Your SQL goes here-- Table: public.Microbits

-- DROP TABLE IF EXISTS public."Microbits";

CREATE TABLE IF NOT EXISTS public."Microbits"
(
    "MicrobitID" integer NOT NULL DEFAULT nextval('"Microbits_MicrobitID_seq"'::regclass),
    "F_AccountID" integer,
    CONSTRAINT "Microbits_pkey" PRIMARY KEY ("MicrobitID"),
    CONSTRAINT "Owner" FOREIGN KEY ("F_AccountID")
        REFERENCES public."Users" ("AccountID") MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE CASCADE
        NOT VALID
)

TABLESPACE pg_default;

ALTER TABLE IF EXISTS public."Microbits"
    OWNER to postgres;
-- Index: fki_Owner

-- DROP INDEX IF EXISTS public."fki_Owner";

CREATE INDEX IF NOT EXISTS "fki_Owner"
    ON public."Microbits" USING btree
    ("F_AccountID" ASC NULLS LAST)
    TABLESPACE pg_default;

    -- Table: public.Users

-- DROP TABLE IF EXISTS public."Users";

CREATE TABLE IF NOT EXISTS public."Users"
(
    "AccountID" integer NOT NULL DEFAULT nextval('"Users_AccountID_seq"'::regclass),
    "Email" text COLLATE pg_catalog."default" NOT NULL,
    "Password" text COLLATE pg_catalog."default" NOT NULL,
    CONSTRAINT "Users_pkey" PRIMARY KEY ("AccountID")
)

TABLESPACE pg_default;

ALTER TABLE IF EXISTS public."Users"
    OWNER to postgres;