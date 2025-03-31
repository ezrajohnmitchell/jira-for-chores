CREATE TABLE
    IF NOT EXISTS CATALOGUE_TASK (
        id uuid PRIMARY KEY,
        organization uuid NOT NULL,
        created_by uuid NOT NULL,
        title varchar(80) NOT NULL,
        description text NOT NULL
    )