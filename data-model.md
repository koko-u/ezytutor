## data model

```plantuml
@startuml
entity course {
    +course_id
    +tutor_id
    --
    course_name
    posted_time
}
@enduml
```