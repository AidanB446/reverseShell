package main

import (
	"fmt"
	"log"
	"net/http"
)

func ping(w http.ResponseWriter, r *http.Request) {
	fmt.Fprintf(w, "pong")	
}

func main() {
	fmt.Println("server start")
	
	http.HandleFunc("/", ping)
	http.HandleFunc("/wsCli", wsCliEndpoint)
	http.HandleFunc("/wsAtt", attEndpoint)	


	log.Fatal(http.ListenAndServe(":8080", nil))
}



