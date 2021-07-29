CREATE TABLE public.users
(
    id bigint NOT NULL,
    name character varying(512) NOT NULL,
    allow_post_vacancies boolean NOT NULL DEFAULT FALSE,
    PRIMARY KEY (id)
);

CREATE TABLE public.vacancies
(
    id bigint GENERATED ALWAYS AS IDENTITY,
    owner_id bigint NOT NULL,
    title character varying(512) NOT NULL,
    description text,
    PRIMARY KEY (id)
);
