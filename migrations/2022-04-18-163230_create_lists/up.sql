CREATE TABLE lists
(
    id_list      TEXT       NOT NULL   PRIMARY KEY,
    display_name         TEXT       NOT NULL,
    is_owner     BOOLEAN    NOT NULL,
    count        INTEGER    NOT NULL,
    icon_name    TEXT       DEFAULT 'view-list-symbolic',
    provider     TEXT       NOT NULL
);