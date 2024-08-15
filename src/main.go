//https://www.digitalocean.com/community/tutorials/how-to-make-an-http-server-in-go
//https://blog.logrocket.com/creating-a-web-server-with-golang/

package main

import (
	"fmt"
	"net/http"
	"log"
)

func main() {
    fmt.Println("The server has started successfully in http://localhost:8080")
	// file handle
	fileServer := http.FileServer(http.Dir("./public"))
    http.Handle("/", fileServer)

	if err := http.ListenAndServe(":8080", nil); err != nil {
        log.Fatal(err)
    }
}