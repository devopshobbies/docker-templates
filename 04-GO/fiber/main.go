package main

import (
	"log"

	"github.com/gofiber/fiber/v2"
)

func indexHandler(c *fiber.Ctx) error {
	return c.SendString("We are DevOps Hobbies :)")
}

func main() {
	app := fiber.New()

	app.Get("/", indexHandler)

	if err := app.Listen(":80"); err != nil {
		log.Println(err)
	}
}
