# ATS Monitoring
Application for collecting data from controllers, recording values and events in the PostgreSQL DBMS, sending notifications via the SMS gateway API

[Diagram](https://viewer.diagrams.net/?tags=%7B%7D&highlight=0000ff&edit=_blank&layers=1&nav=1&title=diagram%20ats-monitoring.drawio.xml#R7Vtbd9o4EP41eUyPL9jAY4A27W6yS0q23T4KWzFqbMkry0Dy63dkyxjjSygQII3PySFoJEuj0cw3n8f4whwGy2uOwtktc7F%2FYWju8sIcXRhGt2fApxQ8pQK7b6YCjxM3Fem5YEKesRJqShoTF0eFgYIxX5CwKHQYpdgRBRninC2Kwx6YX1w1RB4uCSYO8svS78QVMyXVNS3v%2BIyJN1NL9yzVMUXOo8dZTNV6lFGc9gQom0YNjWbIZYs1kfnxwhxyxkT6LVgOsS%2Btmlksve5TTe9KZY6p2OaCiMSe9zz69DzrTgfBgHvDuz8u%2B%2Bksc%2BTHyhRKWfGU2Qa7YCrVZFzMmMco8j%2Fm0kGyfyyX0aCVj7lhLAShDsKfWIgnde4oFgxEMxH4qveBUaE6dRva5a2p3UYs5o7Sy%2BzrloZ7nd6DbvU6mnOZOQ7iHhYNe%2B6k4%2BS%2B1iZXRrvGLMCCP8EAjn0kyLzoIkh5mrcal5scviirV59AncYF69s%2B6D5wybxwCvZ%2FsfSTxFKXUWKqKxgARlrmnVI9n3i0cIUDNsR8fZDtqf9%2BNuMuS6WzfKFhLC93SSQ4mcaCMCqbeE7gnNQSYJd0leLKIE62mUk3%2FG6OuSAQoldqS0I60yDboNqWOYhC5BDq3Seu1ssFN%2FhBOoGRS76q%2BNUT2QyFciEnnkoXjpT3aYntOWCM3Ig5AoeTYZx6qNKtU3Teovevolw2HBYQR3330RT7gxVcDJnPQP1RAhhwmeDscYU9WUx8QgHxJaR%2Bw9xFFG2EilEbJ9J2eNno3VlvBk8Kubuquchh0DSVbLaGgCtY2ycgKsPTqA2KsNJPA4h3QlMntaWPalr%2BuVdoTOtiJctf2YV6%2FRzge3Sb8NLtqvC6up%2BsRUs6VymIamKrpD3Iwk3ZjFdaxpfBU7unFJx%2B9arY35T4ZFcwqrbWmC3kcWpRHIYQNi9aqLz%2BoTW6xhRzJBg%2FB2X%2BGU%2FOQY3xDeCCNiZLSSF%2FXR0Qlj3phVSSuWrioMBQAB4f%2FASlH4gEp3U4dzkL7zMKoTWRkV8A2b5VAFndLKOs3qtA2dXAg6Osbr5d5tfE6l5kf7p%2BKvrXpHYtBTw2oftO0tk1D3EX05bCvT0KZ9on53DWHhwuZW8pZ0vll4nf5Pwu7QPriEvlS7JvZza3WxobwiDOpCUcNCUUix0Y2ZRvy9HeCAFLUvs9J4F1Dkzj219SnQjzOeaNJn9X5KNjnJ58aCV8CJg7jSPpPcMxzPllXLKjjPeq7LGO69Jkm0knIK6bkJViVjHLWcVcofgGtHMmkEDT1RIhgyyZGMUawB%2FsZqh9sC4sUHMIbT1vw58czgXABUyNSHKMGEVigaPMkbciQNsfuNHfyAd6xYFX5gP7tc67%2B5uRTX3bWqNunYpt3g6%2FjqhGNH%2Fw0%2Bj4f975wY%2FPl%2BdxDmBu%2FvSvvB6CRDV%2FrPeNlmrytPW03hpjSC84Cf1mhNzpECutdl53DHp91Xg%2F6j9RiVLjjAW12bGl%2BGdA8W3r5BQ%2F29fb4PhHq9jeMAiG7VjnK5Vzy6R%2F17282m0Ci4TH8eTu5jCsPJck9XLtllEiGAcwaZz%2FRXp%2FbkTe7n4oUvluBbMzTOuoVL5cwGqp%2FKtReVs7OZWvL%2B3sWXhk%2FDGCs0keILfc43y5R79zeu5ht9yjqiTpE0xFY857t5wjrQXOCV4cqhaYS%2F7miMKtoaGNILVI%2FkHo7889dK0iFx2bfPRKONByjkNzjl6%2FfM72K1GO6vJL1TPSk1YPja0LT031pJfrTicrPDWpfXDmdzX%2BAn2TW3nr5iGBF6j%2BtzstAzwBA7ROWn2q9MW2%2BFRDAJ3HCaZuywErgcZxIKyaCfKuj4LPjK9ZJ64VNeazg2eQ8Sx57aLNGeebM45bNah2vzZpNASPRuNg2j6xqJ53Esh6doCjSL5I9i7zx5Hv96sjuPyj5fZ%2Bf9%2Bj7lhFpO5W%2FDi9e9T7%2FfIjhuvJbXvAux6wfbyCDjTzF1qTvrX3hc2P%2FwM%3D)

[Documentation](https://docs.rs/crate/ats-monitoring/0.2.0)

[Manual](https://github.com/stepanov-denis/ats-monitoring/blob/master/ATS%20Monitoring%20manual.pdf)
## Prerequisites
* Install Rust for Linux or macOS
```
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```
For Windows, visit [this page](https://www.rust-lang.org/tools/install)
* Install dependencies
```
$ sudo apt-get install libssl-dev build-essential
```
## Initial setup
* Clone the repository
```
$ git clone git@github.com:stepanov-denis/ats-monitoring.git
```
* Edit the http get requests
```
let resp = reqwest::blocking::get("https://api-mapper.clicksend.com/http/v2/send.php?method=http&username=development-service@yandex.ru&key=1E82A334-89D8-985C-526B-712DB70A713D&to=+79139402913&message=Сбой+питания+от+электросети.+Успешный+старт+генератора.").unwrap();
```
* Edit the connection configuration strings to PostgreSQL in module psql (psql.rs)
```
pub fn db_connect() -> String {
    let string_connection = String::from("postgresql://stepanov:postgres@localhost/postgres");
    return string_connection
}
```
* Compile local packages and all their dependencies
```
$ cargo build --release
```
## Setting up the environment
* Install [PostgreSQL](https://www.postgresql.org/download/)
* Install [SMLogix](https://segnetics.com/ru/smlogix)
* Upload into PLC Pixel file "ats control.psl" and edit network addresses
* Upload into PLC Trim5 file "winter garden.psl" and edit network addresses
* Install [Orange Data Mining](https://orangedatamining.com/download/#linux)
* [Сonfigure](https://orangedatamining.com/widget-catalog/data/sqltable/) data reading from SQL database
* Create an account on [ClickSend](https://www.clicksend.com/) and top up your balance
* Run PLC Pixel
* Run PLC Trim5
* Run PostgreSQL
* Run ATS Monitoring
## Use
* Сreate and save the report in the form you need in Orange Data Mining
* Check your phone for SMS messages from ClickSend
