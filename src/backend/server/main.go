package main

// This is the server backend, made to be used automatically by hosting servers

import (
	"net/http"
	"log"
)

/*
func sendGzip(w http.ResponseWriter, r *http.Request, mime string, path string) error {
	File := "../../frontend/" + path
	// Checks if the client supports Gzip compression
	if !isGzipAccepted(r) {
		//http.Error(w, "Not Acceptable", http.StatusNotAcceptable) // err 406
		send(path, w, mime)
		return errors.New("Not acceptable! Sending uncompressed file")
	}

	// Create a gzip writer
	w.Header().Set("Content-Encoding", "gzip")
	w.Header().Set("Content-Type", mime)
	gz := gzip.NewWriter(w)
	defer gz.Close()

	_, error := os.Stat(File)
	exist := !errors.Is(error, os.ErrNotExist)

	if !exist {
		send(path, w, mime)
		return errors.New("File not found! Trying to send uncompressed file")
	}

	// Open the HTML file
	file, err := os.Open(File) // TODO: create a better way to do this
	if err != nil {
		return_err := errors.New("Error opening the file")
		//
		config.Err(w, 500, return_err)
		return return_err
	}
	defer file.Close()
	//file, _, _ := config.ReadFile(File);

	// Copy file contents to Gzip writer
	_, err = io.Copy(gz, file)
	if err != nil {
		return_err := errors.New("Error copying the content")
		//
		config.Err(w, 500, return_err)
		return return_err
	}
	return nil
}
*/

func main() {
    http.Handle("/", http.FileServer(http.Dir("../../frontend")))

	if err := http.ListenAndServe(":80", nil); err != nil {
        log.Fatal(err)
    }
}