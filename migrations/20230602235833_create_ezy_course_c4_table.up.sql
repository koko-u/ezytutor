CREATE TABLE IF NOT EXISTS ezy_course_c4 (
    course_id UUID NOT NULL,
    tutor_id UUID NOT NULL,
    course_name VARCHAR(256) NOT NULL,
    posted_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT pk_ezy_course_c4 PRIMARY KEY (course_id)
);