# Rusty Connect4 with TootandOtto

Rusty Connect4 with TootandOtto is a computerized version of "Connect4" and "Toot & Otto", written in rust and based on the open source project [Connect4-with-TootandOtto](https://github.com/thinking-fun/Connect4-with-TootandOtto) from which this project is named. This rusty project uses the [Actix web](https://actix.rs/) framework for the server side, [Yew](https://yew.rs/docs/) for the frontend, and [r2d2](https://github.com/sfackler/r2d2) as well as [rusqlite](https://github.com/rusqlite/rusqlite) for the database. 

# Requirements

Cargo and Rust are required to run this project, and they can be installed using the most recent [installation instructions](https://doc.rust-lang.org/cargo/getting-started/installation.html) available on the official rust documentation.

Cargo web must also be installed on your system. 
To install it run:
```bash
cargo install cargo-web
```

To update an existing install of cargo web:
```bash
cargo install --force cargo-web
```

SQLite is also required. For modern MacOS systems this will be preinstalled, but for other systems it can be downloaded from the [SQLite Download Page](https://www.sqlite.org/download.html), and extracted into the appropriate folder for your opperating system, then initialized by running the appropriate tools. An easier way to install sqlite is to use a package management system such as vcpkg for windows or sudo for Linux, as follows:

Windows:
```bash
.\vcpkg install sqlite3
```

Linux:
```bash
sudo apt-get install sqlite3
```

Windoes does not natively support .sh files, but there are 3rd party applications which can add the ability to run sh files. One such untility is included in Git, and as such is already present on many windows devices. If the Git\bin folder is included in your PATH envrionment variable then the .sh files can be run as described below. Otherwise another 3rd party utility can be used, in which case you must follow that utilities instructions for running .sh files.

# Installation and Building

Download the .zip file containing the root of the project, and extract it to your desired location. 

Build the project by running build.sh. 

On Linux or Mac:

```bash
./build.sh
```

On Windows with git utility:

```bash
.\build.sh
```

The build file can be executed again whenever a rebuild is necessary, such as if changes are made to the executble files, or if the generated built files become corrupted in some way and must be regenerated. However for most users this command will only be run upon installation.

**Rationale**

Each part of the project has its own build pipelines and resources produced. `build.sh` ensures the each part of the project is built correct and the resources are allocated are moved where they need to be within the project. The frontend is built first and its resources moved to the server. Next the game is built. Lastly, the server is built. The process ensures correct placement of files, as the `index.html` which doesn't change with new builds change. Further, cargo web hardcodes the locations of `app.js` so the files need to be placed correctly. 

# Usage

To run the server simply execute the run.sh file.

On Linux or Mac:

```bash
./run.sh
```

On Windows with git utility:

```bash
.\run.sh
```

This will run the backend server and make it available at http://localhost:3000/. This is simply where curl calls to the database are directed from the frontend.

To run the client side and frontend simply execute the run.sh file.

On Linux or Mac:

```bash
./run.sh
```

On Windows with git utility:

```bash
.\run.sh
```

To access the game, go to http://localhost:3000/ on any browser. 

**Rationale:**

Since there are three subdirectories, this allows the server to be run from the root directory. 

# Design Considerations

### 1) What can we do on a computer than we can’t do on a printed board?
​    There are a variety of advantages we get when using a computer over a printed board. For one, we can save information about the game and each user automatically, so we can see scores and track a user's wins and losses over time. 
​    We also gain the ability to play these games whenever and wherever we would like, requiring only an internet connection and a device to connect us, such as a laptop or a phone that many people have with them at any given moment. We don't have to bring with us any additional items, and we can't lose pieces of the game that might require us to repurchase items just to keep the game functional. Similarily, a digital game can be played as many times as a user would like, without developing any wear or degrading the appearance or usability of the game in any way. 

### 2) What is a computerized opponent? What are its objectives? (Remember, not everyone is an expert.)
​    A computerized opponent is an element that the user can interact with as they would interact with an opponent. The computerized oponent will act as another player, chosing moves based on an algorithm, to allow the user to play the game on their own, without requiring another human player. The primary purpose of a computerized opponent is usually not to play the game perfectly, but to allow the user the opportunity to play the game and enjoy it as they would if they were playing against another person. Different users will have different skill sets, and users will generally require a computerized opponent with a similar skill level to their own in order to enjoy the game. Few new users will enjoy loosing every game to a computer which makes perfect choices, but similarily, few skilled users will enjoy playing without a challenge. Since the goal of developing a computerized opponent is to improve users' enjoyment, it is often beneficial to have multiple opponents with varrying difficulty levels available.

### 3) What design choices exist for the Interface components? 

This was the biggest flaw in our design and ultimately prevented the successful completion of the application. We split the application into three parts, the frontend, the backend, and the game logic. This modular approach would allow us to parallelize the work and allow the developers to interact at an interface level. Here was our original design intent:

 1) **Backend**: Actix Web Server with sqlite3 DB. Using Rust and the Actix server, the backend would setup the GET and POST endpoints for interacting with the database as well as serving the application from a single entry point, `index.html`. The interface level here is only with the endpoints defined by the Actix server. This simplifies making data requests and sending new items to the database.

When chosing the database to use for our project, we debated between MongoDB and SQL, and in the end we chose to use sql. When working with MongoDB in rust, we found it was difficult to get cross platform support, which was important because our project must be able to run in Windows, Linux, or Mac OS due to the variety of platforms on which it was developed and on which it may be run. We found SQL was more reliable accross platforms, and because of our developers extensive previous experience with SQL, this option was more intuitive. We chose to use the r2d2 library to control access to this database in order to utilize it's pool system with actix-web, to allow multiple concurrent connections to our database to occur safely. 

2) **Frontend:** The frontend was completely redesigned to use the Rust frontend framework Yew along with Yew-router. Yew router is a front-end routing library that allows components (not pages) to be loaded based upon the url. From a user perspective, this allows components to be swapped in and out without having to reload the page. From a developer perspective, this allows the backend and the frontend to have a single point of interface: `index.html`, `app.js` and `app.wasm`. The `index.html` file has only a couple base tags and loads the entire application using `app.js`. This means the entire application can be componentized and only a single `index.html` file need be sent from the server.

3) **Game Logic:** The third module we intended to be the game logic, also built in rust. We thought we would follow a similar model to what was presented in the MEAN stack example provided in class. The example had entry points in the HTML templates that allowed the game logic and animations be loaded with the template. This proved a substantially larger challenge than we had anticipated. Reimplementing the gameboard in rust was a large task in itself, and our initial efforts focused on having command-line debug capabilities. 

**Ultimately, we didn't properly design the interface between the front-end and the game logic.** So while our code was modular, it proved a huge task to integrate the front-end and game logic. 

### 4) What does exception handling mean in a GUI system? 
​    In a GUI system, exception handling means showing users when something has failed, possibly an internal server call or something withint the client side code, without crashing the GUI. 'Failing safely' so to speak, such that the user can see when something has gone wrong, and be presented with options to fix the problem or return to a safe state (Such as the option to reload a web page, or being prompted to enter valid input into a field). 

### 5) Do we require a command-line interface for debugging purposes?
​    Yes, we do require a command-line interface for debugging purposes, because it allows us to more easily track and run tests directly on the logic of our code, without introducing the complications of a GUI and the potential for this GUI to hide errors or introduce new errors unrelated to the code we would like to test. 

### 6) What are Model-View-Controller and Model-View-Viewmodel? Are either applicable to your design? Justify your answer.

Model-view-controller is a paradi

