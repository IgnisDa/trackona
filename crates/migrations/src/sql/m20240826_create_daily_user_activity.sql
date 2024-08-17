CREATE OR REPLACE FUNCTION get_time_of_day(time_input TIMESTAMPTZ)
RETURNS TEXT AS $$
BEGIN
    RETURN CASE
        WHEN EXTRACT(HOUR FROM time_input) BETWEEN 5 AND 11 THEN 'Morning'
        WHEN EXTRACT(HOUR FROM time_input) BETWEEN 12 AND 16 THEN 'Afternoon'
        WHEN EXTRACT(HOUR FROM time_input) BETWEEN 17 AND 20 THEN 'Evening'
        ELSE 'Night'
    END;
END;
$$ LANGUAGE plpgsql;

DROP MATERIALIZED VIEW IF EXISTS "daily_user_activity";

CREATE MATERIALIZED VIEW "daily_user_activity" AS
WITH counted_lots AS (
    SELECT
        CAST(s."finished_on" AS DATE) AS "date",
        s."user_id",
        get_time_of_day(s."last_updated_on") AS "time_of_day",
        m."lot",
        COUNT(DISTINCT s."metadata_id") AS "lot_count"
    FROM
        public."seen" s
    JOIN
        public."metadata" m ON s."metadata_id" = m."id"
    WHERE
        s."finished_on" IS NOT NULL
    GROUP BY
        CAST(s."finished_on" AS DATE), s."user_id", m."lot", "time_of_day"
),
reviews_count AS (
    SELECT
        CAST(r."posted_on" AS DATE) AS "review_day",
        r."user_id",
        get_time_of_day(r."posted_on") AS "time_of_day",
        COUNT(DISTINCT r."id") AS "review_counts"
    FROM
        public."review" r
    GROUP BY
        CAST(r."posted_on" AS DATE), r."user_id", "time_of_day"
),
measurements_count AS (
    SELECT
        CAST(m."timestamp" AS DATE) AS "measurement_day",
        m."user_id",
        get_time_of_day(m."timestamp") AS "time_of_day",
        COUNT(DISTINCT m."timestamp") AS "measurement_counts"
    FROM
        public."user_measurement" m
    GROUP BY
        CAST(m."timestamp" AS DATE), m."user_id", "time_of_day"
),
workouts_count AS (
    SELECT
        CAST(w."end_time" AS DATE) AS "workout_day",
        w."user_id",
        get_time_of_day(w."end_time") AS "time_of_day",
        COUNT(DISTINCT w."id") AS "workout_counts"
    FROM
        public."workout" w
    GROUP BY
        CAST(w."end_time" AS DATE), w."user_id", "time_of_day"
),
aggregated_times AS (
    SELECT
        coalesce(cl."date", rc."review_day", mc."measurement_day", wc."workout_day") AS "date",
        coalesce(cl."user_id", rc."user_id", mc."user_id", wc."user_id") AS "user_id",
        coalesce(cl."time_of_day", rc."time_of_day", mc."time_of_day", wc."time_of_day") AS "time_of_day",
        SUM(
            COALESCE(cl."lot_count", 0) +
            COALESCE(rc."review_counts", 0) +
            COALESCE(mc."measurement_counts", 0) +
            COALESCE(wc."workout_counts", 0)
        ) AS "time_of_day_count"
    FROM
        counted_lots cl
    FULL JOIN
        reviews_count rc ON cl."date" = rc."review_day" AND cl."user_id" = rc."user_id" AND cl."time_of_day" = rc."time_of_day"
    FULL JOIN
        measurements_count mc ON cl."date" = mc."measurement_day" AND cl."user_id" = mc."user_id" AND cl."time_of_day" = mc."time_of_day"
    FULL JOIN
        workouts_count wc ON cl."date" = wc."workout_day" AND cl."user_id" = wc."user_id" AND cl."time_of_day" = wc."time_of_day"
    GROUP BY
        coalesce(cl."date", rc."review_day", mc."measurement_day", wc."workout_day"),
        coalesce(cl."user_id", rc."user_id", mc."user_id", wc."user_id"),
        coalesce(cl."time_of_day", rc."time_of_day", mc."time_of_day", wc."time_of_day")
)
SELECT
    cl."date",
    cl."user_id",
    jsonb_agg(
        jsonb_build_object(
            'lot', cl."lot",
            'count', cl."lot_count"
        )
    ) AS "metadata_counts",
    jsonb_object_agg(at."time_of_day", at."time_of_day_count") AS "times_of_day",
    COALESCE(rc."review_counts", 0) AS "review_counts",
    COALESCE(mc."measurement_counts", 0) AS "measurement_counts",
    COALESCE(wc."workout_counts", 0) AS "workout_counts",
    CAST(
        (COALESCE(SUM(cl."lot_count"), 0) +
        COALESCE(rc."review_counts", 0) +
        COALESCE(mc."measurement_counts", 0) +
        COALESCE(wc."workout_counts", 0)) AS BIGINT
    ) AS "total_counts"
FROM
    counted_lots cl
LEFT JOIN
    reviews_count rc ON cl."date" = rc."review_day" AND cl."user_id" = rc."user_id"
LEFT JOIN
    measurements_count mc ON cl."date" = mc."measurement_day" AND cl."user_id" = mc."user_id"
LEFT JOIN
    workouts_count wc ON cl."date" = wc."workout_day" AND cl."user_id" = wc."user_id"
LEFT JOIN
    aggregated_times at ON cl."date" = at."date" AND cl."user_id" = at."user_id"
GROUP BY
    cl."date", cl."user_id", rc."review_counts", mc."measurement_counts", wc."workout_counts"
ORDER BY
    cl."date", cl."user_id";

DROP INDEX IF EXISTS "daily_user_activity_dates";
CREATE INDEX "daily_user_activity_dates" ON "daily_user_activity" ("date");
