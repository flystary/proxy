package main

import (
	"proxy/g"
	"flag"
	"fmt"
	"os"
	"proxy/http"
)

func main() {
	//cfg
	cfg := flag.String("c", "cfg.toml", "configuretion file")
	version := flag.Bool("v", false, "proxyplay Version")
	flag.Parse()

	if *version {
		fmt.Println(g.VERSION)
		os.Exit(0)
	}

	g.ParseConfig(*cfg)

	if g.Config().Debug {
		g.InitLog("debug")
	} else {
		g.InitLog("info")
	}

	g.InitRootDir()
	go http.Start()

	select {}
}
