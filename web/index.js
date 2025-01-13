fetch("/api/courses").then(courses => {
    console.log(courses.json().then(jason => console.log(jason)))
});