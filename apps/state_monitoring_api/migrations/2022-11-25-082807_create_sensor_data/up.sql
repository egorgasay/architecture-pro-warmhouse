create table sensor_data
(
    id        serial primary key,
    value     varchar                 not null,
    status    varchar                 not null,
    ts        timestamp default now() not null,
    sensor_id integer                 not null
);