# Digital Coop

Welcome to the digital coop project

This project is a simple implementation of an online order placing application.

Users should be able to enter a `member id` and then place `items` in a `order`. 

The application does not handle payment (all done in person), the goal it to increase the number of people who can shop by removing the physical limit of being in the store.

<img src="screen.png" width="400px"/>

## 🏃‍♀️ How to run me

You'll need to start the webapp and the backend server. 


To run the webapp just navigate to this repo and type. This app was templated with `create-react-app`

```bash
yarn start
```

Now start the server

```bash
cd backend
cargo run --release
```


## Notes for a developer

The backend does very little, and can easily be replaced if Rust is too much of a learning curve (otherwise this light backend should be robust and easy to run). 

All orders are stored in a SQLite database (in the backend code) and is stored a JSON in SQLites native JSON1 extension (if this is confusing compared to a traditional DB - let me know and I can anwser any questions.)

The checkout page is not stylized and the timeslot feature is not implented. This app is like.... ~80% from having a solid ordering system. The last parts are unimplemented since there are no real requirements... This can change and if this proves useful those features can be added quickly.