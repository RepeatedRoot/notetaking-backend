//database url
const DATABASE_URL = "http://192.168.0.20:8000";
const loginModal = new bootstrap.Modal('#loginModal');
const loginToast = new bootstrap.Toast(document.getElementById('logintoast'));
const noteModal = new bootstrap.Modal('#noteModal');

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
	
	$('.notes-container').attr('current-client', client.attr('id'));
	$('.notes-container').attr('current-notes', client.attr('notes'));

	$('.notes-container .row').remove(); //remove any old entries if present

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
	let date = new Date(note.datetime);

	/* Append the HTML container */
	container.append(
		`
		<div class="row p-3">
			<div class="col align-self-center">
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
			</div>
		</div>
		`
	);
}

$.fn.createNote = function () {
	"use strict";

	let clientId = $('.notes-container').attr("current-client");
	let clientNotes = $('.notes-container').attr("current-notes");

	let date = $('#noteDate').val();
	let note = $('#noteText').val();

	console.log(clientId);
	console.log(clientNotes);
	console.log(date, note);
  

	$.ajax({
		url: `${DATABASE_URL}/notes/${clientNotes}`,
		method: "PUT",
		async: false,
		xhrFields: { withCredentials: true },
		data: JSON.stringify({
			datetime: date,
			note: note
		}),
		success: function (data) {
			console.log(data);
		},
		error: function (error) {
			console.log(error);
		}
	});

	noteModal.hide();

	$().viewNotes();
};

$.fn.retrieveNotes = async function (clientId) {
	"use strict";
};

$.fn.login = function () {
	"use strict";

	/* let username = $('#loginEmail').val();
	let password = $("#loginPassword").val(); */

	let username = "erikaodinson@cahfs.sa.gov.au";
	let password = "password";

	$.ajax({
		url: `${DATABASE_URL}/login`,
		method: "POST",
		async: false,
		data: JSON.stringify({
			username: username,
			password: password
		}),
  
		success: function (data) {
			console.log("Login success " + data);
			loginModal.hide();
			loginToast.show();

			$('.client-list').getClients();
			 
			$('.client-entry').click(function () {
			  $(this).viewNotes();
			});
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
		xhrFields: { withCredentials: true },
		success: function (data) {
			console.log("Successfully logged out " + data);
		},
		error: function (error) {
			console.log("Error logging out");
		}
	})
}
