-- 1. What is the total bonus amount for each employee?
select sum(amount), emp_id from bonus group by emp_id order by emp_id;
-- +-------------------------------------------------------------------------+
-- | QUERY PLAN                                                              |
-- |-------------------------------------------------------------------------|
-- | Sort  (cost=7940.17..8045.73 rows=42223 width=12)                       |
-- |   Sort Key: emp_id                                                      |
-- |   ->  HashAggregate  (cost=4274.00..4696.23 rows=42223 width=12)        |
-- |         Group Key: emp_id                                               |
-- |         ->  Seq Scan on bonus  (cost=0.00..3274.00 rows=200000 width=8) |
-- +-------------------------------------------------------------------------+

-- 2. Which employees received the highest total compensation last year, and how much was it?

-- we just need to get the salary, no one had bonus last year (2023)
select emp_id, salary from employee order by salary desc;
-- +---------------------------------------------------------------------+
-- | QUERY PLAN                                                          |
-- |---------------------------------------------------------------------|
-- | Sort  (cost=4918.41..5043.41 rows=50000 width=8)                    |
-- |   Sort Key: salary DESC                                             |
-- |   ->  Seq Scan on employee  (cost=0.00..1016.00 rows=50000 width=8) |
-- +---------------------------------------------------------------------+

-- results:
-- +--------+--------+
-- | emp_id | salary |
-- |--------+--------|
-- | 23993  | 80000  |
-- | 44995  | 80000  |
-- | 34257  | 80000  |
-- | 30778  | 80000  |
-- | 13980  | 79999  |
-- | 2565   | 79999  |
-- | 45915  | 79999  |
-- | 21340  | 79998  |
-- | 30354  | 79998  |
-- | 19105  | 79998  |
-- | 702    | 79998  |
-- | 1425   | 79998  |
-- +--------+--------+

-- but lets have some fun and join data with the bonus table (pretend we care about bonuses of this year)

-- this gets the sum of the bonuses for each employee in 2024
select sum(amount), emp_id from bonus where date_part('year', time) = 2024 group by emp_id;
-- +------------------------------------------------------------------------------------------------+
-- | QUERY PLAN                                                                                     |
-- |------------------------------------------------------------------------------------------------|
-- | Finalize GroupAggregate  (cost=4065.76..4155.05 rows=991 width=12)                             |
-- |   Group Key: emp_id                                                                            |
-- |   ->  Gather Merge  (cost=4065.76..4142.20 rows=588 width=12)                                  |
-- |         Workers Planned: 1                                                                     |
-- |         ->  Partial GroupAggregate  (cost=3065.75..3076.04 rows=588 width=12)                  |
-- |               Group Key: emp_id                                                                |
-- |               ->  Sort  (cost=3065.75..3067.22 rows=588 width=8)                               |
-- |                     Sort Key: emp_id                                                           |
-- |                     ->  Parallel Seq Scan on bonus  (cost=0.00..3038.71 rows=588 width=8)      |
-- |                           Filter: (date_part('year'::text, "time") = '2024'::double precision) |
-- +------------------------------------------------------------------------------------------------+

-- now we need to join both

-- regular join
select b.amount as bonus, e.emp_id, e.salary from employee e join bonus b on b.emp_id = e.emp_id;

-- solving the puzzle:
select
    e.emp_id,
    -- coalesce is needed because some
    -- employees might have no bonus (would give null result)
    coalesce(sum(b.amount), 0) + e.salary as total_compensation
from
    employee e
-- left join because regular INNER JOIN would not bring
-- employees without bonus
left join
    bonus b on b.emp_id = e.emp_id
    and date_part('year', b.time) = 2024
group by
    e.emp_id
order by
    total_compensation desc
limit 10;

-- to use dates dinamically:
--     AND b.time >= date_trunc('year', CURRENT_DATE - INTERVAL '1 year')
--     AND b.time < date_trunc('year', CURRENT_DATE)

-- results:
-- +--------+--------------------+
-- | emp_id | total_compensation |
-- |--------+--------------------|
-- | 37766  | 155136             |
-- | 7299   | 154983             |
-- | 19586  | 154430             |
-- | 18410  | 153260             |
-- | 44385  | 152787             |
-- | 23456  | 152702             |
-- | 12961  | 150949             |
-- | 13391  | 150895             |
-- | 37756  | 150649             |
-- | 43373  | 148990             |
-- +--------+--------------------+

-- 3. What is the total bonus amount awarded by department?
select
    e.dep_id as department,
    sum(b.amount) as total_bonus
from
    bonus b
join
    employee e
ON
    e.emp_id = b.emp_id
group by
    e.dep_id;

-- results
-- +------------+-------------+
-- | department | total_bonus |
-- |------------+-------------|
-- | 1          | 600876770   |
-- | 2          | 199996518   |
-- | 3          | 201135900   |
-- +------------+-------------+

-- explain
-- +------------------------------------------------------------------------------------------------------+
-- | QUERY PLAN                                                                                           |
-- |------------------------------------------------------------------------------------------------------|
-- | Finalize GroupAggregate  (cost=5988.61..5989.00 rows=3 width=12)                                     |
-- |   Group Key: e.dep_id                                                                                |
-- |   ->  Gather Merge  (cost=5988.61..5988.96 rows=3 width=12)                                          |
-- |         Workers Planned: 1                                                                           |
-- |         ->  Sort  (cost=4988.60..4988.61 rows=3 width=12)                                            |
-- |               Sort Key: e.dep_id                                                                     |
-- |               ->  Partial HashAggregate  (cost=4988.55..4988.58 rows=3 width=12)                     |
-- |                     Group Key: e.dep_id                                                              |
-- |                     ->  Hash Join  (cost=1641.00..4400.32 rows=117647 width=8)                       |
-- |                           Hash Cond: (b.emp_id = e.emp_id)                                           |
-- |                           ->  Parallel Seq Scan on bonus b  (cost=0.00..2450.47 rows=117647 width=8) |
-- |                           ->  Hash  (cost=1016.00..1016.00 rows=50000 width=8)                       |
-- |                                 ->  Seq Scan on employee e  (cost=0.00..1016.00 rows=50000 width=8)  |
-- +------------------------------------------------------------------------------------------------------+

-- now let's say we want the department names hehe :)
select
    e.dep_id as department,
    d.name,
    sum(b.amount) as total_bonus
from
    bonus b
join employee e ON e.emp_id = b.emp_id
join department d ON e.dep_id = d.dep_id
group by
    e.dep_id, d.name;

-- +------------+-------------+-------------+
-- | department | name        | total_bonus |
-- |------------+-------------+-------------|
-- | 1          | sales       | 600876770   |
-- | 2          | marketing   | 199996518   |
-- | 3          | engineering | 201135900   |
-- +------------+-------------+-------------+

-- +------------------------------------------------------------------------------------------------------------+
-- | QUERY PLAN                                                                                                 |
-- |------------------------------------------------------------------------------------------------------------|
-- | Finalize GroupAggregate  (cost=7029.08..7030.27 rows=9 width=21)                                           |
-- |   Group Key: e.dep_id, d.name                                                                              |
-- |   ->  Gather Merge  (cost=7029.08..7030.12 rows=9 width=21)                                                |
-- |         Workers Planned: 1                                                                                 |
-- |         ->  Sort  (cost=6029.07..6029.09 rows=9 width=21)                                                  |
-- |               Sort Key: e.dep_id, d.name                                                                   |
-- |               ->  Partial HashAggregate  (cost=6028.84..6028.93 rows=9 width=21)                           |
-- |                     Group Key: e.dep_id, d.name                                                            |
-- |                     ->  Hash Join  (cost=1642.07..5146.49 rows=117647 width=17)                            |
-- |                           Hash Cond: (e.dep_id = d.dep_id)                                                 |
-- |                           ->  Hash Join  (cost=1641.00..4400.32 rows=117647 width=8)                       |
-- |                                 Hash Cond: (b.emp_id = e.emp_id)                                           |
-- |                                 ->  Parallel Seq Scan on bonus b  (cost=0.00..2450.47 rows=117647 width=8) |
-- |                                 ->  Hash  (cost=1016.00..1016.00 rows=50000 width=8)                       |
-- |                                       ->  Seq Scan on employee e  (cost=0.00..1016.00 rows=50000 width=8)  |
-- |                           ->  Hash  (cost=1.03..1.03 rows=3 width=13)                                      |
-- |                                 ->  Seq Scan on department d  (cost=0.00..1.03 rows=3 width=13)            |
-- +------------------------------------------------------------------------------------------------------------+
