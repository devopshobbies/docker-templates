package main

import (
	"log"
	"net/http"

	"github.com/labstack/echo/v4"
)

func indexHandler(c echo.Context) error {
	return c.String(http.StatusOK, "We are DevOps Hobbies :)")
}

func main() {
	app := echo.New()

	app.GET("/", indexHandler)

	log.Println("We are ready. Then please go to the localhost...")

	if err := app.Start(":80"); err != nil {
		log.Fatal(err)
	}
}
