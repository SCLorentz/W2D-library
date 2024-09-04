//https://www.digitalocean.com/community/tutorials/how-to-make-an-http-server-in-go
//https://blog.logrocket.com/creating-a-web-server-with-golang/

package main

import (
	"fmt"
	"net/http"
	"log"
)

/*func sendGzip(w http.ResponseWriter, r *http.Request, mime string, path string) error {
	File := "../frontend/" + path
	// Verifica se o cliente aceita compressão Gzip
	if !isGzipAccepted(r) {
		//http.Error(w, "Not Acceptable", http.StatusNotAcceptable) // err 406
		send(path, w, mime)
		return errors.New("not acceptable! Sending uncompressed file")
	}

	// Cria um escritor gzip
	w.Header().Set("Content-Encoding", "gzip")
	w.Header().Set("Content-Type", mime)
	gz := gzip.NewWriter(w)
	defer gz.Close()

	_, error := os.Stat(File)
	exist := !errors.Is(error, os.ErrNotExist)

	if !exist {
		send(path, w, mime)
		return errors.New("file not found! Trying to send uncompressed file")
	}

	// Abre o arquivo HTML
	file, err := os.Open(File) // todo: create a better way to do this
	if err != nil {
		return_err := errors.New("error opening the file")
		//
		config.Err(w, 500, return_err)
		return return_err
	}
	defer file.Close()
	//file, _, _ := config.ReadFile(File);

	// Copia o conteúdo do arquivo para o escritor Gzip
	_, err = io.Copy(gz, file)
	if err != nil {
		return_err := errors.New("error copying the content")
		//
		config.Err(w, 500, return_err)
		return return_err
	}
	return nil
}*/

func main() {
    fmt.Println("The server has started successfully in http://localhost:8080")
	// file handle
	fileServer := http.FileServer(http.Dir("./frontend"))
    http.Handle("/", fileServer)

	if err := http.ListenAndServe(":8080", nil); err != nil {
        log.Fatal(err)
    }
}
