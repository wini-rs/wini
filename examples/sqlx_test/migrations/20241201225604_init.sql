create table users (
    id serial primary key,
    name varchar(24) not null,
    age int
);

insert into users (name, age)
values 
    ('amy', 32),
    ('john', 39),
    ('mia', 27),
    ('liu', 45);
