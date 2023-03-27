package main

import (
	"fmt"
	"log"
	"net/http"
)

func indexHandler(w http.ResponseWriter, _ *http.Request) {
	_, _ = fmt.Fprintf(w, "We are DevOps Hobbies.")
}

func main() {
	log.Println("We are ready. Then please go to the localhost...")

	http.HandleFunc("/", indexHandler)
	if err := http.ListenAndServe(":80", nil); err != nil {
		log.Fatal(err)
	}
}
