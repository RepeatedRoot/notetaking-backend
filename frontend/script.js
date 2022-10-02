//database url
const DATABASE_URL = "http://127.0.0.1:8000";
const loginModal = new bootstrap.Modal('#loginModal');
const loginToast = new bootstrap.Toast(document.getElementById('logintoast'));

loginModal.show();

//Append a client entry to the list
$.fn.appendClient = function (clientdata) {
	"use strict";
	let clientList = $(this); //The ID of the client list div

	//Add the HTML string of the entry to the div, inserting information about the client
	clientList.append(
		`<a id=${clientdata._id.$oid} notes=${clientdata.notes.$oid} class="client-entry list-group-item list-group-item-action py-3 lh-sm" aria-current="true" style="cursor: pointer"	>
			<div class="d-flex w-100 align-items-center justify-content-between">
				<strong class="mb-1">${clientdata.firstname} ${clientdata.surname}</strong>
				<small>${clientdata.sex}</small>
			</div>
			<div class="col-10 mb-1 small">${clientdata.address}</div>
		 </a>`
	);
}

/* Get list of clients from the database */
$.fn.getClients = function () {
	"use strict";

	var response; //to store the JSON object

	/* AJAX GET request to the client API */
	$.ajax({
		url: `${DATABASE_URL}/clients/`,
		method: "GET",
		async: false,
		xhrFields: { withCredentials: true },
		success: function (data) {
			response = data;
		},
		error: function (error) {
			console.log(error);
		}
	});

	/* Append each entry in the list to the client container */
	response.forEach((client) => {
		this.appendClient(client);
	});
}

/* Render notes */
$.fn.viewNotes = function () {
	"use strict";

	let client = $(this); //The ID of the client information div

	let NoteId = client.attr('notes'); //The note ID attribute of the client entry

	var response; //to store returned information

	/* Request the notes entry for the client from the database */
	$.ajax({
		url: `${DATABASE_URL}/notes/${NoteId}`,
		method: "GET",
		async: false,
		success: function (data) {
			response = data;
			console.log(data);
		},
		error: function (error) {
			console.log(error);
		}
	})

	/* Append each note to the website */
	response.notes.forEach((note) => {
		$('.notes-container').renderNote(note);
	});
}

/* Rendering notes to the website */
$.fn.renderNote = function (note) {
	"use strict";
	
	const container = $(this);

	console.log('rendering note...');
	console.log(note);

	/* Parse the datetime object to a string */
	let date = new Date(parseInt(note.datetime));

	/* Append the HTML container */
	container.append(
		`
		<div class="card">
			<div class="card-header">
				${date.toDateString()}
			</div>
			<div class="card-body">
				<blockquote class="blockquote mb-0">
					<p>${note.note}</p>
					<footer class="blockquote-footer">${note.clinician.$oid}<cite title="Source Title"></cite></footer>
				</blockquote>
			</div>
		</div>
		`
	);
}

/* Set options in a dropdown */
$.fn.setDropdown = async function (data, key) {
	"use strict";
	const dropdown = $(this);

	var data = await data;
	data.forEach((entry) => {
		console.log(entry[key]);
		dropdown.append(`<option>${entry[key]}</option>`);
	});
};

$.fn.createClient = async function () {
	"use strict";
};

$.fn.createNote = async function (clientId) {
	"use strict";
};

$.fn.retrieveNotes = async function (clientId) {
	"use strict";
};

$.fn.login = function () {
	"use strict";

	let username = $('#loginEmail').val();
	let password = $("#loginPassword").val();

	let loginObject = {
		username: username,
		password: password
	};

	$.ajax({
		url: `${DATABASE_URL}/login`,
		method: "POST",
		async: false,
		data: JSON.stringify(loginObject),
		success: function (data) {
			console.log("Login success " + data);
			loginModal.hide();
			loginToast.show();
		},
		error: function (error) {
			console.log(error);
		}
	})
}

$.fn.logout = function () {
	"use strict";

	$.ajax({
		url: `${DATABASE_URL}/logout`,
		method: "POST",
		async: false,
		success: function (data) {
			console.log("Successfully logged out " + data);
		},
		error: function (error) {
			console.log("Error logging out");
		}
	})
}
// $("#workplace_select").setDropdown(getWorkplaces(), "name");
