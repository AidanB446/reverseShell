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

func cliReader(conn *websocket.Conn) {
	for {
		_, p, err := conn.ReadMessage()		
		handleErr(err)	
		userMessage := string(p)				
		
		// error protection incase rReader connections isnt yet established.
		if rReader1.conn != nil {
			err := rReader1.conn.WriteMessage(1, []byte(userMessage))		
			handleErr(err)
		}
		
		fmt.Println("Client message recieved") // debug
	}
}

func wsCliEndpoint(w http.ResponseWriter, r *http.Request) {
	// allow all origins
	upgrader.CheckOrigin = func(r *http.Request) bool { return true }
	
	// create websocket connection, upgrade current http
	ws, err := upgrader.Upgrade(w, r, nil)

	handleErr(err)	

	fmt.Println("client connected") // debug

	cli1 = Client{ws}
	err1 := ws.WriteMessage(1, []byte("websocket connected"))
	handleErr(err1)
	cliReader(ws)
}

func attReader(conn *websocket.Conn) {
	for {
		_, p, err := conn.ReadMessage()		
		handleErr(err)	

		userMessage := string(p)				
	
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

// this will never be used. But if it isnt here websocket will break 
func rReader(conn *websocket.Conn) {

	for {
		_, p, err := conn.ReadMessage()		
		handleErr(err)	

		userMessage := string(p)				
		
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




