package main

import (
	"fmt"
	"log"
	"net/http"
	"github.com/gorilla/websocket"
)
	
func handleErr(err error) {
	if err != nil {
		log.Println(err)
	}
}

var upgrader = websocket.Upgrader{
	ReadBufferSize: 1024,
	WriteBufferSize: 1024,
}	

type Client struct {
	conn *websocket.Conn
}

var cli1 Client
var att1 Client
var rReader1 Client

// every time cli sends message this function runs
// for loop just makes sure it continues to run next time
func cliReader(conn *websocket.Conn) {

	// take message and log it into cli text file	
	for {
		_, p, err := conn.ReadMessage()		

		handleErr(err)	

		userMessage := string(p)				
	
		// handle users message, write into file, send back to client
		if att1.conn != nil {
			err := rReader1.conn.WriteMessage(1, []byte(userMessage))		
			handleErr(err)
		}

		fmt.Println("Client message recieved")
	}
}

func wsCliEndpoint(w http.ResponseWriter, r *http.Request) {
	// allow all origins
	upgrader.CheckOrigin = func(r *http.Request) bool { return true }
	
	// create websocket connection, upgrade current http
	ws, err := upgrader.Upgrade(w, r, nil)
	handleErr(err)	

	fmt.Println("client connected")
	
	cli1 = Client{ws}
	
	// everything written to client needs to be base64 encoded string

	err1 := ws.WriteMessage(1, []byte("websocket connected"))
	handleErr(err1)
	
	cliReader(ws)
}

func attReader(conn *websocket.Conn) {

	// take message and log it into cli text file	
	for {
		_, p, err := conn.ReadMessage()		
		handleErr(err)	

		userMessage := string(p)				
		// handle users message, write into file, send back to client
	
		if cli1.conn != nil {
			err := cli1.conn.WriteMessage(1, []byte(userMessage))		
			handleErr(err)
		}
		
		fmt.Println("Attacker message recieved")
	}
}

func attEndpoint(w http.ResponseWriter, r *http.Request) {
	// allow all origins
	upgrader.CheckOrigin = func(r *http.Request) bool { return true }
	
	// create websocket connection, upgrade current http
	ws, err := upgrader.Upgrade(w, r, nil)
	handleErr(err)	

	fmt.Println("client connected")
	
	att1 = Client{ws}

	err1 := ws.WriteMessage(1, []byte("websocket connected"))
	handleErr(err1)
	
	attReader(ws)
}

func rReader(conn *websocket.Conn) {

	// take message and log it into cli text file	
	for {
		_, p, err := conn.ReadMessage()		
		handleErr(err)	

		userMessage := string(p)				
		// handle users message, write into file, send back to client
		
		fmt.Println("From Client", userMessage)
	
	}
}

func rEndpoint(w http.ResponseWriter, r *http.Request) {
	// allow all origins
	upgrader.CheckOrigin = func(r *http.Request) bool { return true }
	
	// create websocket connection, upgrade current http
	ws, err := upgrader.Upgrade(w, r, nil)
	handleErr(err)	

	fmt.Println("client connected")
	
	rReader1 = Client{ws}

	err1 := ws.WriteMessage(1, []byte("websocket connected"))
	handleErr(err1)
	
	rReader(ws)
}




