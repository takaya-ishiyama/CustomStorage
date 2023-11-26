create table if not exists users (
    id varchar(26) not null,
    name varchar(50) not null,
    constraint pk_user_id primary key (id)
);

insert into users (id, name) values ('01GE4ZQCSW8QHKSCA172Q5F358',  'テストデータ') on conflict do nothing;