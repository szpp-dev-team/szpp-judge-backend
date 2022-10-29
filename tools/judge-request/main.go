package main

import (
	"bytes"
	"encoding/json"
	"fmt"
	"io"
	"log"
	"net/http"
	"os"
)

type SubmitPayload struct {
	TaskID     int    `json:"taskId"`
	ContestID  int    `json:"contestId"`
	LanguageID string `json:"languageId"`
	SourceCode string `json:"sourceCode"`
}

const (
	TaskID         = 2
	ContestID      = 2
	SourceName     = "1.cpp"
	ProdBackendURL = "http://35.221.67.128:8080"
	DevBackendURL  = "http://localhost:8080"
)

func main() {
	sourceCode, err := os.ReadFile(SourceName)
	if err != nil {
		log.Fatal(err)
	}

	submitPayload := &SubmitPayload{
		TaskID:     TaskID,
		ContestID:  ContestID,
		SourceCode: string(sourceCode),
		LanguageID: "cpp",
	}
	submitPayloadBody := &bytes.Buffer{}
	_ = json.NewEncoder(submitPayloadBody).Encode(submitPayload)
	fmt.Println(submitPayloadBody.String())

	jwttoken := "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJleHAiOjE2Njc2MDQzNjMsImlkIjoxLCJyb2xlIjoiZ2VuZXJhbCJ9.TGbaIxS4iPjnRBYUqVcxXE7mgDmeecgwaipwsm1GWi4"
	req, _ := http.NewRequest(http.MethodPost, fmt.Sprintf("%s/submits", DevBackendURL), submitPayloadBody)
	req.Header.Add("Content-Type", "application/json")
	req.Header.Add("Authorization", fmt.Sprintf("Bearer %s", jwttoken))
	resp, err := http.DefaultClient.Do(req)
	if err != nil {
		log.Fatal(err)
	}
	defer resp.Body.Close()
	b, _ := io.ReadAll(resp.Body)
	fmt.Println(string(b))
}
