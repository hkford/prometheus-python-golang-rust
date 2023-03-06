package main

import (
	"main/server"
)

func main() {
	e := server.NewRouter()

	e.Logger.Fatal(e.Start(":8888"))
}
