let selections = {
    'actor_1': null,
    'actor_2': null
};

// const base_path = 'http://localhost:8000';
const base_path = '';


function capitalizeWords(str) {
    return str.replace(/\b\w/g, (match) => match.toUpperCase());
}

async function get_actor_search_results(actor_name) {
    const urlSearchParams = new URLSearchParams();
    urlSearchParams.append('name', actor_name);

    try {
        const response = await fetch(`${base_path}/actor_prefix`, {
            method: 'POST',
            body: urlSearchParams,
            headers: {
                'Content-Type': 'application/x-www-form-urlencoded',
            },
        });

        if (!response.ok) {
            throw new Error('Failed to fetch actor search results');
        }

        return response.json();
    } catch (error) {
        console.error(error);
        return [];
    }
}

function update_table(actor_data, actor_index) {
    console.log(actor_data);
    let table_id = `search_table_${actor_index}`;
    let table = document.getElementById(table_id);
    table.innerHTML = table.rows[0].innerHTML;

    try {
        for (let row of actor_data) {
            let newRow = table.insertRow(-1);
            let newCellId = newRow.insertCell(0);
            newCellId.innerHTML = row.id;
            let newCellName = newRow.insertCell(1);
            // capitalize the first letter of the name and insert it
            newCellName.innerHTML = capitalizeWords(row.full_name);


            // newCellName.innerHTML = row.full_name;


            let newCellBirthYear = newRow.insertCell(2);
            newCellBirthYear.innerHTML = row.birth_year;

            // Add event listener to the 
            addRowClickListener(newRow, actor_index);

        }
    } catch (error) {
        console.error(error);
    }
}

// function update_search_results(input_value, target, selection) {

//     let search_results = await get_actor_search_results(input_value);


//         .then(data => {
//         // Handle the response data
//         console.log(data);
//         // get the searc_table_1 element
//         let table = document.getElementById(target);
//         // Clear the search_table_1 body, not the header
//         table.innerHTML = table.rows[0].innerHTML;
//         // Loop through the data
//         data.forEach(row => {
//             // Create a new row
//             let newRow = table.insertRow(-1);
//             // insert the id
//             let newCellId = newRow.insertCell(0);
//             newCellId.innerHTML = row.id;
//             // insert the name
//             let newCellName = newRow.insertCell(1);
//             newCellName.innerHTML = row.full_name;
//             // insert birth year
//             let newCellBirthYear = newRow.insertCell(2);
//             newCellBirthYear.innerHTML = row.birth_year;

//             // add row event listener
//             addRowClickListener(newRow, selection);


//             return row.id;

//         })
//             .catch(error => {
//                 // Handle any errors
//                 console.error(error);
//             });

//     });

// }

function addRowClickListener(row, actor_index) {
    row.addEventListener('click', () => {
        // Remove highlight from previously selected row
        let table = row.closest('table');
        let previouslySelectedRow = table.querySelector('.table-active');
        if (previouslySelectedRow) {
            previouslySelectedRow.classList.remove('table-active');
        }
        // Highlight the clicked row
        row.classList.add('table-active');
        // remove the table-hover from this row
        // update selections
        selections[`actor_${actor_index}`] = row.cells[0].textContent;
    }, { once: true });
}
// Get the actor_form_1 element
const actorInput1 = document.getElementById('actor_input_1');
const actorInput2 = document.getElementById('actor_input_2');

// Add an event listener for keyup event
actorInput1.addEventListener('keyup', async () => {

    let search_results = await get_actor_search_results(actorInput1.value);
    update_table(search_results, 1);
});


actorInput2.addEventListener('keyup', async () => {

    let search_results = await get_actor_search_results(actorInput2.value);
    update_table(search_results, 2);


});


// Get the submit button element
const submitButton = document.getElementById('submit-button');
const errorSection = document.getElementById('error-section');
const submissionResultsList = document.getElementById('submission-results-list');
const pathLengthHeader = document.getElementById('path-length-header');
const resultsSection = document.getElementById('submission-results');

async function get_shortest_path(actor_1_id, actor_2_id) {
    try {
        const urlSearchParams = new URLSearchParams();
        urlSearchParams.append('actor_1', actor_1_id);
        urlSearchParams.append('actor_2', actor_2_id);

        const response = await fetch(`${base_path}/shortest_path`, {
            method: 'POST',
            body: urlSearchParams,
            headers: {
                'Content-Type': 'application/x-www-form-urlencoded',
            },
        });

        if (!response.ok) {
            throw new Error('Failed to fetch shortest path');
        }

        const data = await response.json();
        return data;
    } catch (error) {
        console.error(error);
        throw error;
    }
}

async function render_path(shortest_path) {
    console.log('rendering path');
    try {
        // set the path length header
        pathLengthHeader.textContent = `Shortest path length: ${shortest_path.length}`;

        // Clear the submission results
        submissionResultsList.innerHTML = '';

        for (const element of shortest_path) {
            // Extract actor_1, movie, and actor_2 from each JSON list element
            const { actor_1, movie, actor_2 } = element;

            // Create a new list item element
            const listItem = document.createElement('li');
            listItem.classList.add('list-group-item');

            // Set the text content with bullet points
            listItem.textContent = `${actor_1} acted in ${movie} with ${actor_2}`;

            // Append the list item to the submission results
            submissionResultsList.appendChild(listItem);
        }
    } catch (error) {
        console.error(error);
    }
}

// Add an event listener for submit button click
submitButton.addEventListener('click', async () => {
    let actor_1 = selections['actor_1'];
    let actor_2 = selections['actor_2'];

    if (actor_1 && actor_2) {
        try {
            let shortest_path_json = await get_shortest_path(actor_1, actor_2);
            render_path(shortest_path_json);
            // Hide the error section
            errorSection.style.display = 'none';
            // Show the submission results
            resultsSection.style.display = 'block';
        } catch (error) {
            console.error(error);
            // Show the error section
            errorSection.style.display = 'block';
            // Hide the submission results
            resultsSection.style.display = 'none';
        }
    } else {
        // Show the error section
        errorSection.style.display = 'block';
        // Hide the submission results
        submissionResultsList.style.display = 'none';
    }
});

