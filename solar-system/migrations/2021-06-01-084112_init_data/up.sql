insert into planets(name, type, mean_radius, mass) values ('Mercury', 'TERRESTRIAL_PLANET', 2439.7, 0.33 * power(10, 24));
insert into planets(name, type, mean_radius, mass) values ('Venus', 'TERRESTRIAL_PLANET', 6051.8, 4.87 * power(10, 24));
insert into planets(name, type, mean_radius, mass) values ('Earth', 'TERRESTRIAL_PLANET', 6371.0, 5.97 * power(10, 24));
insert into planets(name, type, mean_radius, mass) values ('Mars', 'TERRESTRIAL_PLANET', 3389.5, 0.642 * power(10, 24));
insert into planets(name, type, mean_radius, mass) values ('Jupiter', 'GAS_GIANT', 69911.0, 1898.0 * power(10, 24));
insert into planets(name, type, mean_radius, mass) values ('Saturn', 'GAS_GIANT', 58232.0, 568.0 * power(10, 24));
insert into planets(name, type, mean_radius, mass) values ('Uranus', 'ICE_GIANT', 25362.0, 86.8 * power(10, 24));
insert into planets(name, type, mean_radius, mass) values ('Neptune', 'ICE_GIANT', 24622.0, 102.0 * power(10, 24));

insert into satellites(name, first_spacecraft_landing_date, planet_id) values ('Moon', '1959-09-13', (select id from planets where name = 'Earth'));
insert into satellites(name, planet_id) values ('Phobos', (select id from planets where name = 'Mars'));
insert into satellites(name, planet_id) values ('Deimos', (select id from planets where name = 'Mars'));
insert into satellites(name, planet_id) values ('Io', (select id from planets where name = 'Jupiter'));
insert into satellites(name, planet_id) values ('Europa', (select id from planets where name = 'Jupiter'));
insert into satellites(name, planet_id) values ('Ganymede', (select id from planets where name = 'Jupiter'));
insert into satellites(name, planet_id) values ('Callisto', (select id from planets where name = 'Jupiter'));
insert into satellites(name, planet_id) values ('Titan', (select id from planets where name = 'Saturn'));
insert into satellites(name, planet_id) values ('Ariel', (select id from planets where name = 'Uranus'));
insert into satellites(name, planet_id) values ('Umbriel', (select id from planets where name = 'Uranus'));
insert into satellites(name, planet_id) values ('Titania', (select id from planets where name = 'Uranus'));
insert into satellites(name, planet_id) values ('Oberon', (select id from planets where name = 'Uranus'));
insert into satellites(name, planet_id) values ('Miranda', (select id from planets where name = 'Uranus'));
insert into satellites(name, planet_id) values ('Triton', (select id from planets where name = 'Neptune'));