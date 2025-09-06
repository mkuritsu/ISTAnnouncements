const coursesList = document.querySelector(".courses-list");
const degreeSelect = document.getElementById("degree-select");
const courseSelect = document.getElementById("course-select");
const addCourseButton = document.getElementById("add-course-button");
const confirmAddCourseButton = document.getElementById("submit-add-course-button");
const cancelAddCourseButton = document.getElementById("cancel-add-course-button");
const popupContainer = document.querySelector(".add-course-popup-container");
const ignorePrevious = document.getElementById("ignore-previous");
const courseColor = document.getElementById("course-color");
const courseRole = document.getElementById("role-id")

async function deleteCourse(course) {
    await fetch("/api/courses/" + course.id, { method: "DELETE" });
}

function createCourseCard(course) {
    let div = document.createElement("div");
    div.classList.add("course-card");
    let header = document.createElement("div");
    header.classList.add("course-card-header");
    let footer = document.createElement("footer");
    footer.classList.add("course-card-footer");
    let title = document.createElement("h1");
    title.textContent = course.name;
    let button = document.createElement("button");
    button.textContent = "Remove";
    button.classList.add("remove-course-button");
    button.onclick = async () => {
        await deleteCourse(course);
        div.remove();
    };
    footer.appendChild(button);
    header.appendChild(title);
    div.append(header);
    div.append(footer);
    return div;
}

function getAcademicYear() {
    const now = new Date();
    const year = now.getFullYear();
    if (now.getMonth() >= 7) {
        return `${year}/${year + 1}`;
    } else {
        return `${year - 1}/${year}`;
    }
}

async function selectDegree(degreeId) {
    courseSelect.replaceChildren();
    const opt = document.createElement("option");
    opt.setAttribute("selected", true);
    opt.setAttribute("value", -1);
    opt.textContent = "None"
    courseSelect.appendChild(opt);
    if (degreeId != -1) {
        const academicYear = getAcademicYear();
        const response = await fetch(`/cors?url=https://fenix.tecnico.ulisboa.pt/api/fenix/v1/degrees/${degreeId}/courses?academicTerm=${academicYear}`);
        const courses = await response.json();
        courses.sort((a, b) => a.name <= b.name ? -1 : 1);
        for (const course of courses) {
            const courseOpt = document.createElement("option");
            courseOpt.setAttribute("value", course.id);
            courseOpt.textContent = course.name;
            courseSelect.appendChild(courseOpt);
        }
    }
}

async function addCourse(courseId) {
    const response = await fetch(`/cors?url=https://fenix.tecnico.ulisboa.pt/api/fenix/v1/courses/${courseId}`);
    const courseInfo = await response.json();
    const courseDto = {
        id: parseInt(courseId),
        name: courseInfo.name,
        rss_url: courseInfo.announcementLink,
        role_id: courseRole.value,
        ignore_previous: ignorePrevious.checked,
        color: parseInt(courseColor.value.substring(1), 16)
    };
    console.log(courseDto);
    const addResponse = await fetch("/api/courses", { method: "POST", headers: { 'Content-Type': 'application/json' }, body: JSON.stringify(courseDto) });
    const course = await addResponse.json();
    const card = createCourseCard(course);
    coursesList.appendChild(card);
}

fetch("/api/courses").then(response => response.json().then(courses => {
    for (let i = 0; i < courses.length; i++) {
        const course = courses[i];
        const card = createCourseCard(course);
        coursesList.appendChild(card);
    }
}));

fetch("/cors?url=https://fenix.tecnico.ulisboa.pt/api/fenix/v1/degrees/").then(response => response.json()).then(degrees => {
    const degreeMap = new Map();
    for (let i = 0; i < degrees.length; i++) {
        const degree = degrees[i];
        const type = degree.type;
        if (!degreeMap.has(type)) {
            degreeMap.set(type, []);
        }
        degreeMap.get(type).push(degree);
    }
    console.log(degreeMap);
    for (const key of degreeMap.keys()) {
        console.log(key);
        const degrees = degreeMap.get(key);
        degrees.sort((a, b) => a.name <= b.name ? -1 : 1);
        const optgroup = document.createElement("optgroup");
        optgroup.setAttribute("label", key);
        for (const degree of degrees) {
            const opt = document.createElement("option");
            opt.setAttribute("value", degree.id);
            opt.textContent = degree.name;
            optgroup.appendChild(opt);
        }
        degreeSelect.appendChild(optgroup);
    }
    degreeSelect.addEventListener("change", (ev) => {
        selectDegree(degreeSelect.value);
    });
});

confirmAddCourseButton.addEventListener("click", async () => {
    popupContainer.style.visibility = "hidden";
    if (courseSelect.value != -1) {
        await addCourse(courseSelect.value);
    }
});

cancelAddCourseButton.addEventListener("click", () => {
    popupContainer.style.visibility = "hidden";
});

addCourseButton.addEventListener("click", () => {
    popupContainer.style.visibility = "visible";
});
