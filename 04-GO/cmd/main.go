package main

import (
	"fmt"
	"net/http"
)

func indexHandler(w http.ResponseWriter, r *http.Request) {
	fmt.Fprintf(w, "We are DevOps Hobbies.")

}

func main() {
	fmt.Println("We are ready. Then please go to the localhost...")
	http.HandleFunc("/", indexHandler)
	http.ListenAndServe(":80", nil)
}
