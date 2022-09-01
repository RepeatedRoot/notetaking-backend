# Endpoints to the application

The following lists the different endpoints of the application, their functions and the users which are allowed to access them.


## "/api/*"

Endpoints to interact with the database / backend.

### "/api/auth/{create, login, remove}"

 - create = Creating an account (Only accessable to an Admin "SuperUser")
 - login = Logging in and retrieving session variable for that session (Accessable to all accounts)
 - remove = Deleting an account from the database (Accessable to an Admin account)

### "/api/client/{search, fetch, create, remove}"

 - search = Searching the client database (Only accessable to Clinicians)
 - fetch = Fetching more detailed information about a client (Only accessable to a Clinician)
 - create = Creating a new client account (Accessable to Admins and Clinicians)
 - remove = Removing a client's entry in the database (Only accessable to Clinicians)

### "/api/notes/{retrieve, insert, update}" (Only accessable to clinicians)

 - fetch = Retrieving the notes of a given client
 - insert = Inserting a new entry to the notes object

### "/" (Accessable to all users, redirects if not logged in)

 - The main 'index' page of the website, to render notes

### "/notfound"

 - The website's 404 page.
