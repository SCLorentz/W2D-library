package main

// this is the server backend, made to be used automaticly by hosting servers

import (
	"net/http"
	"log"
)

/*func sendGzip(w http.ResponseWriter, r *http.Request, mime string, path string) error {
	File := "../../frontend/" + path
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
    http.Handle("/", http.FileServer(http.Dir("../../frontend")))

	if err := http.ListenAndServe(":80", nil); err != nil {
        log.Fatal(err)
    }
}