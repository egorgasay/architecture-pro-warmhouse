create table sensor_data
(
    id        serial primary key,
    value     double precision        not null,
    unit      varchar                 not null,
    status    varchar                 not null,
    created_at timestamp default now() not null,
    sensor_id integer                 not null
);