//database url
const DATABASE_URL = "http://192.168.0.20:8000";

/* Declaring references to main bootstrap components */
const loginModal = new bootstrap.Modal("#loginModal");
const noteModal = new bootstrap.Modal("#noteModal");
const clientModal = new bootstrap.Modal("#clientModal");
const messageToast = new bootstrap.Toast(
  document.getElementById("messageToast")
);

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
};

/* Get list of clients from the database */
$.fn.getClients = function () {
  "use strict";

  var response; //to store the JSON object

  /* AJAX GET request to the client API */
  $.ajax({
    url: `${DATABASE_URL}/clients/`,
    method: "GET",
    async: false,
    xhrFields: { withCredentials: true }, //This endpoint is restricted, send cookies to authenticate
    success: function (data) {
      response = data;
    },
    error: function (error) {
      console.log(error);
    },
  });

  /* Append each entry in the list to the client container */
  response.forEach((client) => {
    this.appendClient(client);
  });

  /* Add an event listener to each of the client entries which will display the client's notes */
  $(".client-entry").click(function () {
    $(this).viewNotes();
  });
};

$.fn.clientUpdate = function () {
  "use strict";

  let ID = $(".notes-container").attr("current-client"); //ID of currently selected client

  $.ajax({
    url: `${DATABASE_URL}/client/${ID}`,
    method: "GET",
    async: false,
    xhrFields: { withCredentials: true }, //This endpoint is restricted, send cookies to authenticate
    success: function (data) {
      console.log(data);

      /* Update input field values */
      $("#clientFirstname").val(data.firstname);
      $("#clientMiddleNames").val(data.middlenames);
      $("#clientLastname").val(data.surname);
      $("#clientSex").val(data.sex);
      $("#clientAddress").val(data.address);
      $("#clientPhoneNumber").val(data.phone);

      /* Show the modal so values can be updated */
      clientModal.show();
    },
    error: function (error) {
      $().showMessage(
        "There was an error fetching the client's information",
        "warning"
      );

      console.log(error);
    },
  });
};

/* Create a new client entry in the database or update existing information */
$.fn.submitClient = function (update) {
  "use strict";

  /* Object to store information from the input field */
  let information = {};

  /* Only add keys to the object which contain a value (so that deserialisation does not fail) */
  if ($("#clientFirstname").val() != "")
    information["firstname"] = String($("#clientFirstname").val());
  if ($("#clientMiddleNames").val() != "")
    information["middlenames"] = String($("#clientMiddleNames").val());
  if ($("#clientLastname").val() != "")
    information["surname"] = String($("#clientLastname").val());
  if ($("#clientSex").val() != null)
    information["sex"] = String($("#clientSex").val());
  if ($("#clientAddress").val() != "")
    information["address"] = String($("#clientAddress").val());
  if ($("#clientPhoneNumber").val() != "")
    information["phone"] = String($("#clientPhoneNumber").val());

  console.log(information);

  /* Initialise variables */
  let endpoint = new String();
  let method = new String();
  let message = new String();

  /* If an account's information is being updated */
  if (update) {
    let ID = $(".notes-container").attr("current-client"); //ID of currently selected client
    endpoint = `${DATABASE_URL}/client/${ID}`;
    method = "PUT";
    message = "updated client information";
  } else {
    endpoint = `${DATABASE_URL}/client`;
    method = "POST";
    message = "created client account";
  }

  /* Send the new information to the server */
  $.ajax({
    url: endpoint,
    method: method,
    async: false,
    xhrFields: { withCredentials: true }, //This endpoint is restricted, send cookies to authenticate
    data: JSON.stringify(information),
    success: function (data) {
      $().showMessage(`Successfully ${message}.`, "success"); //show status message

      clientModal.hide();

      /* update client list */ 50;
      $(".client-list a").remove();
      $(".client-list").getClients();
    },
    error: function (error) {
      $().showMessage("Error processing client information.", "danger"); //show error message
      console.log(error);
    },
  });
};

/* Render notes */
$.fn.viewNotes = function () {
  "use strict";

  let client = $(this); //The ID of the client information div

  let NoteId = client.attr("notes"); //The note ID attribute of the client entry

  $(".notes-container").attr("current-client", client.attr("id"));
  $(".notes-container").attr("current-notes", client.attr("notes"));

  $(".notes-container .row").remove(); //remove any old entries if present

  var response; //to store returned information

  /* Request the notes entry for the client from the database */
  $.ajax({
    url: `${DATABASE_URL}/notes/${NoteId}`,
    method: "GET",
    async: false,
    xhrFields: { withCredentials: true }, //This endpoint is restricted, send cookies to authenticate
    success: function (data) {
      response = data;
      console.log(data);
    },
    error: function (error) {
      console.log(error);
    },
  });

  /* Append each note to the website */
  response.notes.forEach((note) => {
    $(".notes-container").renderNote(note);
  });
};

/* Rendering notes to the website */
$.fn.renderNote = function (note) {
  "use strict";

  const container = $(this); //the parent element to insert notes into

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
							<footer class="blockquote-footer">${
                note.clinician.$oid
              }<cite title="Source Title"></cite></footer>
						</blockquote>
					</div>
				</div>
			</div>
		</div>
		`
  );
};

/* Creating a new note to be appended in the database */
$.fn.createNote = function () {
  "use strict";

  /* Attribute to store the ID of currently selected client's notes */
  let clientNotes = $(".notes-container").attr("current-notes");
  let current_client = `#${$(".notes-container").attr("current-client")}`;

  /* Values of inputs elements in the notes modal */
  let date = $("#noteDate").val();
  let note = $("#noteText").val();

  /* PUT request to insert a notes */
  $.ajax({
    url: `${DATABASE_URL}/notes/${clientNotes}`,
    method: "PUT",
    async: false,
    xhrFields: { withCredentials: true }, //This endpoint is restricted, send cookies to authenticate
    data: JSON.stringify({
      //Encode data into a json object
      datetime: date,
      note: note,
    }),
    success: function () {
      $().showMessage("Successfully created note", "success"); //Display a success message

      noteModal.hide(); //Hide the notes modal (it is not needed)

      $(current_client).viewNotes(); //Re-render the notes to update any changes
    },
    error: function (error) {
      $().showMessage("There was an error retrieving notes", "danger");
      console.log(error);
    },
  });
};

/* Show a message using a bootstrap toast */
$.fn.showMessage = function (message, state) {
  "use strict";

  let colour = new String();

  /* Set colour based on keyword */
  switch (state) {
    case "danger":
      colour = "red";
      break;
    case "warning":
      colour = "orange";
      break;
    case "success":
      colour = "green";
      break;
    default:
      colour = "white";
      break;
  }

  $("#messageToast rect").attr("fill", colour); //Update the colour of the message toast
  $(".toast-body").text(message); //Update the messageToast's message

  messageToast.show(); //Show the message toast
};

/* Login to the website */
$.fn.login = function () {
  "use strict";

  /* Input values from the login modal */
  /* let username = $('#loginEmail').val();
	let password = $("#loginPassword").val(); */

  let username = "erikaodinson@cahfs.sa.gov.au";
  let password = "password";

  /* POST request to authenticate with the backend (and gain a private cookie if authorised) */
  $.ajax({
    url: `${DATABASE_URL}/login`,
    method: "POST",
    async: false,
    data: JSON.stringify({
      //JSON encode input data
      username: username,
      password: password,
    }),
    success: function (data) {
      loginModal.hide(); //Hide the login modal (login was successful)
      $().showMessage("Successfully logged in.", "success");

      $(".client-list").getClients(); // Display the list of clients
    },
    error: function (error) {
      $().showMessage("There was an error logging in.", "danger");
      console.log(error);
    },
  });
};

/* Logout from the website */
$.fn.logout = function () {
  "use strict";

  /* POST request to the backend to remove the authenticated private cookie */
  $.ajax({
    url: `${DATABASE_URL}/logout`,
    method: "POST",
    async: false,
    xhrFields: { withCredentials: true }, //ensure that cookie is sent
    success: function (data) {
      /* Remove elements which may have confidential information */
      $(".client-list a").remove();
      $(".notes-container .row").remove();

      $().showMessage("Successfully logged out.", "success");
    },
    error: function (error) {
      console.log("Error logging out.");
    },
  });
};
