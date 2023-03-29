-- Your SQL goes here

create table account_pubg
(
    pubg_id varchar(40)  not null invisible,
    name    varchar(100) not null,
    constraint account_pubg_pk
        primary key (pubg_id)
);

