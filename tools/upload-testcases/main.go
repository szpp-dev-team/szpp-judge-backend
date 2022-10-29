package main

import (
	"bytes"
	"encoding/json"
	"fmt"
	"io"
	"log"
	"net/http"
	"os"
	"path/filepath"
)

type TestcasePayload struct {
	Name   string `json:"name"`
	Input  string `json:"input"`
	Output string `json:"output"`
}

const ProdBackendURL = "http://35.221.67.128:8080"
const DevBackendURL = "http://localhost:8080"

func main() {
	testcaseEntries, _ := os.ReadDir(".")
	for _, testcaseEntry := range testcaseEntries {
		if !testcaseEntry.IsDir() {
			return
		}
		taskId := testcaseEntry.Name()
		if taskId != "2" {
			continue
		}
		fmt.Println(testcaseEntry.Name())
		inEntries, _ := os.ReadDir(filepath.Join(testcaseEntry.Name(), "in"))
		outEntries, _ := os.ReadDir(filepath.Join(testcaseEntry.Name(), "out"))
		if len(inEntries) != len(outEntries) {
			fmt.Fprintln(os.Stderr, "the number of testcase entries in \"in\" and \"out\" is incorrect")
			continue
		}

		for i := range inEntries {

			if inEntries[i].Name() != outEntries[i].Name() {
				fmt.Fprintln(os.Stderr, inEntries[i].Name(), outEntries[i].Name())
				fmt.Fprintln(os.Stderr, "the name of testcase entry in \"in\" and \"out\" is incorrect")
				break
			}
			name := inEntries[i].Name()
			input, _ := os.ReadFile(filepath.Join(testcaseEntry.Name(), "in", inEntries[i].Name()))
			output, _ := os.ReadFile(filepath.Join(testcaseEntry.Name(), "out", outEntries[i].Name()))

			testcasePayload := &TestcasePayload{
				Name:   name,
				Input:  string(input),
				Output: string(output),
			}
			testcasePayloadBody := &bytes.Buffer{}
			jwttoken := "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJleHAiOjE2Njc1OTc4NjUsImlkIjoxLCJyb2xlIjoiZ2VuZXJhbCJ9.9kojuJq4lEV9NPY__HRFEVZeNfrX-szzjO9UpZFuHzQ"
			_ = json.NewEncoder(testcasePayloadBody).Encode(testcasePayload)
			req, _ := http.NewRequest(http.MethodPost, fmt.Sprintf("%s/tasks/%s/testcases", ProdBackendURL, taskId), testcasePayloadBody)
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
	}
}
