package model

import "fmt"

type ProxyReportRequest struct {
	Hostname string
	IP		 string
	ProxyVersion	string
	PluginVersion	string
}

func (res *ProxyReportRequest) String() string {
	return fmt.Sprintf(
		"<Hostname:%s, IP:%s, ProxyVersion:%s, PluginVersion:%s>",
		res.Hostname,
		res.IP,
		res.ProxyVersion,
		res.PluginVersion,
	)
}

type ProxyUpdateInfo struct {
	LastUpdate    int64
	ReportRequest *ProxyReportRequest
}