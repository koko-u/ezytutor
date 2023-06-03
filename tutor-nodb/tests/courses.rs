use std::str::FromStr;

use actix_web::http;
use actix_web::test;
use actix_web::web;
use actix_web::App;
use chrono::Local;
use tutor_nodb::model::courses::Course;
use tutor_nodb::model::courses::NewCourse;
use tutor_nodb::scopes::courses::courses_routes;
use tutor_nodb::states::courses::CoursesSate;
use uuid::Uuid;

use pretty_assertions::assert_eq;

#[actix_web::test]
async fn post_course() {
    let courses_state = web::Data::new(CoursesSate::default());
    let app = test::init_service(App::new().configure(courses_routes(courses_state.clone()))).await;

    let new_course = NewCourse {
        tutor_id: Uuid::from_str("512e0b53-d9d3-4ed7-b758-1f65b1c14a4f").unwrap(),
        course_name: "Course 1".to_string(),
        ..Default::default()
    };
    let req = test::TestRequest::post()
        .uri("/courses")
        .set_json(new_course)
        .to_request();
    let res = test::call_service(&app, req).await;

    assert_eq!(res.status(), http::StatusCode::CREATED);

    let courses = courses_state.lock().unwrap();
    assert!(courses
        .iter()
        .any(|course| course.course_name == "Course 1"));
}

#[actix_web::test]
async fn get_courses() {
    let test_tutor_id = Uuid::from_str("bf0cd75e-21e4-40d6-bb0c-bc48874463df").unwrap();
    let test_course1 = Course {
        tutor_id: Uuid::from_str("dada6b2a-7554-42d1-9bb7-63af7e3c4ccb").unwrap(),
        course_id: Uuid::from_str("fbc0ad72-1756-4810-884a-b25cc3fb9aaf").unwrap(),
        course_name: "Course 1".into(),
        posted_time: Local::now().naive_local(),
    };
    let test_course2 = Course {
        tutor_id: test_tutor_id,
        course_id: Uuid::from_str("83d7ccfb-1d65-40bb-b795-dcdf271efd7f").unwrap(),
        course_name: "Course 2".into(),
        posted_time: Local::now().naive_local(),
    };

    let courses_state = CoursesSate::default();
    {
        let mut courses = courses_state.lock().unwrap();
        courses.push(test_course1.clone());
        courses.push(test_course2.clone());
    }
    let courses_state = web::Data::new(courses_state);
    let app = test::init_service(App::new().configure(courses_routes(courses_state.clone()))).await;

    let req = test::TestRequest::get()
        .uri(&format!("/courses/{test_tutor_id}"))
        .to_request();
    let res: Vec<Course> = test::call_and_read_body_json(&app, req).await;

    assert_eq!(res, vec![test_course2]);
}

#[actix_web::test]
async fn get_single_course() {
    let test_tutor_id = Uuid::from_str("bf0cd75e-21e4-40d6-bb0c-bc48874463df").unwrap();
    let test_course_id = Uuid::from_str("83d7ccfb-1d65-40bb-b795-dcdf271efd7f").unwrap();
    let test_course1 = Course {
        tutor_id: Uuid::from_str("dada6b2a-7554-42d1-9bb7-63af7e3c4ccb").unwrap(),
        course_id: Uuid::from_str("fbc0ad72-1756-4810-884a-b25cc3fb9aaf").unwrap(),
        course_name: "Course 1".into(),
        posted_time: Local::now().naive_local(),
    };
    let test_course2 = Course {
        tutor_id: test_tutor_id,
        course_id: test_course_id,
        course_name: "Course 2".into(),
        posted_time: Local::now().naive_local(),
    };

    let courses_state = CoursesSate::default();
    {
        let mut courses = courses_state.lock().unwrap();
        courses.push(test_course1.clone());
        courses.push(test_course2.clone());
    }
    let courses_state = web::Data::new(courses_state);
    let app = test::init_service(App::new().configure(courses_routes(courses_state.clone()))).await;

    let req = test::TestRequest::get()
        .uri(&format!("/courses/{test_tutor_id}/{test_course_id}"))
        .to_request();
    let res: Course = test::call_and_read_body_json(&app, req).await;

    assert_eq!(res, test_course2);
}

#[actix_web::test]
async fn get_no_courses() {
    let test_tutor_id = Uuid::from_str("bf0cd75e-21e4-40d6-bb0c-bc48874463df").unwrap();
    let test_course_id = Uuid::from_str("fbc0ad72-1756-4810-884a-b25cc3fb9aaf").unwrap();
    let test_course1 = Course {
        tutor_id: Uuid::from_str("dada6b2a-7554-42d1-9bb7-63af7e3c4ccb").unwrap(),
        course_id: test_course_id,
        course_name: "Course 1".into(),
        posted_time: Local::now().naive_local(),
    };
    let test_course2 = Course {
        tutor_id: test_tutor_id,
        course_id: Uuid::from_str("83d7ccfb-1d65-40bb-b795-dcdf271efd7f").unwrap(),
        course_name: "Course 2".into(),
        posted_time: Local::now().naive_local(),
    };

    let courses_state = CoursesSate::default();
    {
        let mut courses = courses_state.lock().unwrap();
        courses.push(test_course1.clone());
        courses.push(test_course2.clone());
    }
    let courses_state = web::Data::new(courses_state);
    let app = test::init_service(App::new().configure(courses_routes(courses_state.clone()))).await;

    let req = test::TestRequest::get()
        .uri(&format!("/courses/{test_tutor_id}/{test_course_id}"))
        .to_request();
    let res = test::call_service(&app, req).await;

    assert_eq!(res.status(), http::StatusCode::NOT_FOUND);
}
