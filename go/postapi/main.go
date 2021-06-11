package main

import (
	"log"
	"net/http"
	"os"

	"postapi/app"

	"postapi/app/database"
)

func main() {
	app := app.New()
	app.DB = &database.DB{}
	err := app.DB.Open()
	check(err)

	http.HandleFunc("/", app.Router.ServeHTTP)

	err = http.ListenAndServe(":9000", nil)
	check(err)
}

func check(e error) {
	if e != nil {
		log.Panicln(e)
		os.Exit(1)
	}
}
