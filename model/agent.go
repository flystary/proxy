package model

import "fmt"

type AgentReportRequest struct {
	Hostname string
	IP		 string
	AgentVersion	string
	PluginVersion	string
}

func (res *AgentReportRequest) String() string {
	return fmt.Sprintf(
		"<Hostname:%s, IP:%s, AgentVersion:%s, PluginVersion:%s>",
		res.Hostname,
		res.IP,
		res.AgentVersion,
		res.PluginVersion,
	)
}

type AgentUpdateInfo struct {
	LastUpdate    int64
	ReportRequest *AgentReportRequest
}